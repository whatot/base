use std::collections::HashMap;

use axum::{Router, extract::Query, http::StatusCode, response::Redirect, routing::get};

mod github;
mod google;
mod twitter;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello World"
}

async fn search(Query(params): Query<HashMap<String, String>>) -> Result<Redirect, StatusCode> {
    let Some(cmd) = params.get("cmd") else {
        return Err(StatusCode::BAD_REQUEST);
    };

    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "tw" => twitter::construct_twitter_url(&cmd),
        "gh" => github::construct_github_url(&cmd),
        _ => google::construct_google_search_url(&cmd),
    };
    Ok(Redirect::to(&redirect_url))
}
