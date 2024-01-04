use std::sync::Arc;

use axum::{routing::get, Router};

use crate::state::ApplicationState;

use super::handlers;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new().route("/hello", get(handlers::hello::hello).with_state(state))
}
