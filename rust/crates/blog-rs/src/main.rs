use axum::extract::TypedHeader;
use axum::headers::UserAgent;

use axum::routing::get;
use axum::Router;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite:///tmp/sqlite.db";

async fn homepage() -> String {
    String::from("homepage")
}

async fn visit_info(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    String::from(user_agent.as_str())
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

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
