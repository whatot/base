use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::state::ApplicationState;

use super::handlers::{self};
use super::middleware::jwt::auth;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/login",
            post(handlers::login::login).with_state(state.clone()),
        )
        .route(
            "/dogs",
            post(handlers::dogs::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route("/dogs", get(handlers::dogs::list).with_state(state.clone()))
        .route("/dogs/:id", get(handlers::dogs::get).with_state(state))
}
