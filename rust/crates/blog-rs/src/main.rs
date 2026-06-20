use axum::Router;
use axum::http::HeaderMap;
use axum::routing::get;

use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite:///tmp/sqlite.db";

async fn homepage() -> String {
    String::from("homepage")
}

async fn visit_info(headers: HeaderMap) -> String {
    headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string()
}

async fn init_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

async fn init_tables() {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);
}

#[tokio::main]
async fn main() {
    init_db().await;
    init_tables().await;

    let app = Router::new()
        .route("/", get(homepage))
        .route("/visit", get(visit_info));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
