use axum::{routing::get, Router};

use super::handlers;

pub fn configure() -> Router {
    Router::new().route("/hello", get(handlers::hello::hello))
}
