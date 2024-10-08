use std::{
    collections::HashMap,
    convert::Infallible,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use bincode::deserialize;
use bincode::serialize;
use clap::{command, Parser};
use log::info;
use riteraft::{async_trait, Mailbox, Raft, Result as RaftResult, Store};
use serde::{Deserialize, Serialize};
use slog::Drain;
use warp::{reply, Filter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(long)]
    raft_addr: String,
    #[arg(long)]
    peer_addr: Option<String>,
    #[arg(long)]
    web_server: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub enum Message {
    Insert { key: String, value: String },
}

#[derive(Clone)]
struct HashStore(Arc<RwLock<HashMap<String, String>>>);

impl HashStore {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    fn insert(&self, key: String, value: String) {
        let mut map = self.0.write().unwrap();
        map.insert(key.to_string(), value.to_string());
        info!("inserted: ({}, {})", key, value);
    }

    fn get(&self, key: String) -> Option<String> {
        self.0.read().unwrap().get(&key).cloned()
    }
}

#[async_trait]
impl Store for HashStore {
    async fn apply(&mut self, message: &[u8]) -> RaftResult<Vec<u8>> {
        let message: Message = deserialize(message).unwrap();
        let message: Vec<u8> = match message {
            Message::Insert { key, value } => {
                self.insert(key.clone(), value.clone());
                serialize(&value).unwrap()
            }
        };
        Ok(message)
    }

    async fn snapshot(&self) -> RaftResult<Vec<u8>> {
        Ok(serialize(&self.0.read().unwrap().clone())?)
    }

    async fn restore(&mut self, snapshot: &[u8]) -> RaftResult<()> {
        let new: HashMap<String, String> = deserialize(snapshot).unwrap();
        let mut db = self.0.write().unwrap();
        let _ = std::mem::replace(&mut *db, new);
        Ok(())
    }
}

fn with_mailbox(
    mailbox: Arc<Mailbox>,
) -> impl Filter<Extract = (Arc<Mailbox>,), Error = Infallible> + Clone {
    warp::any().map(move || mailbox.clone())
}

fn with_store(store: HashStore) -> impl Filter<Extract = (HashStore,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

async fn put(
    mailbox: Arc<Mailbox>,
    key: String,
    value: String,
) -> Result<impl warp::Reply, Infallible> {
    let message = Message::Insert { key, value };
    let message = serialize(&message).unwrap();
    let result = mailbox.send(message).await.unwrap();
    let result: String = deserialize(&result).unwrap();
    Ok(reply::json(&result))
}

async fn get(store: HashStore, key: String) -> Result<impl warp::Reply, Infallible> {
    let response = store.get(key);
    Ok(reply::json(&response))
}

async fn leave(mailbox: Arc<Mailbox>) -> Result<impl warp::Reply, Infallible> {
    mailbox.leave().await.unwrap();
    Ok(reply::json(&"ok".to_string()))
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog_o!("version"=> env!("CARGO_PKG_VERSION")));
    // converts log to slog
    let _log_guard = slog_stdlog::init().unwrap();

    let options = Options::parse();
    info!("args: {options:?}");
    let store = HashStore::new();

    let raft = Raft::new(options.raft_addr, store.clone(), logger.clone());
    let mailbox = Arc::new(raft.mailbox());
    let (raft_handle, mailbox) = match options.peer_addr {
        Some(addr) => {
            info!("running in follower mode");
            let handle = tokio::spawn(raft.join(addr));
            (handle, mailbox)
        }
        None => {
            info!("running in leader mode");
            let handle = tokio::spawn(raft.lead());
            (handle, mailbox)
        }
    };

    let put_kv = warp::get()
        .and(warp::path!("put" / String / String))
        .and(with_mailbox(mailbox.clone()))
        .and_then(|key, value, mailbox: Arc<Mailbox>| put(mailbox, key, value));

    let get_kv = warp::get()
        .and(warp::path!("get" / String))
        .and(with_store(store.clone()))
        .and_then(|key, store: HashStore| get(store, key));

    let leave_kv = warp::get()
        .and(warp::path!("leave"))
        .and(with_mailbox(mailbox.clone()))
        .and_then(leave);

    let routes = put_kv.or(get_kv).or(leave_kv);

    if let Some(addr) = options.web_server {
        let _server = tokio::spawn(async move {
            warp::serve(routes)
                .run(SocketAddr::from_str(&addr).unwrap())
                .await;
        });
    }

    let result = tokio::try_join!(raft_handle)?;
    result.0?;
    Ok(())
}
