use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use actix_web::{get, web, App, HttpServer, Responder};
use bincode::{deserialize, serialize};
use clap::{command, Parser};
use log::info;
use riteraft::{async_trait, Mailbox, Raft, Result as RaftResult, Store};
use serde::{Deserialize, Serialize};
use slog::Drain;

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

#[get("/put/{id}/{name}")]
async fn put(
    data: web::Data<(Arc<Mailbox>, HashStore)>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let message = Message::Insert {
        key: path.0.clone(),
        value: path.1.clone(),
    };
    let message = serialize(&message).unwrap();
    let result = data.0.send(message).await.unwrap();
    let result: String = deserialize(&result).unwrap();
    format!("{:?}", result)
}

#[get("/get/{id}")]
async fn get(
    data: web::Data<(Arc<Mailbox>, HashStore)>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let response = data.1.get(id);
    format!("{:?}", response)
}

#[get("/leave")]
async fn leave(data: web::Data<(Arc<Mailbox>, HashStore)>) -> impl Responder {
    data.0.leave().await.unwrap();
    "OK".to_string()
}

#[actix_rt::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, slog_o!("version"=> env!("CARGO_PKG_VERSION")));
    // converts log to slog
    let _scope_guard = slog_scope::set_global_logger(logger.clone());
    let _log_guard = slog_stdlog::init_with_level(log::Level::Debug).unwrap();

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

    if let Some(addr) = options.web_server {
        let _server = tokio::spawn(
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new((mailbox.clone(), store.clone())))
                    .service(put)
                    .service(get)
                    .service(leave)
            })
            .bind(addr)
            .unwrap()
            .disable_signals()
            .shutdown_timeout(2)
            .workers(2)
            .run(),
        );
    }

    let result = tokio::try_join!(raft_handle)?;
    result.0?;
    Ok(())
}
