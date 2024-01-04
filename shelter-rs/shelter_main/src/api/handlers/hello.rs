use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::state::ApplicationState;

pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello World! Using configuration from {}\n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("-".to_string())
    ))
}
