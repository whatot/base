use serde::{Deserialize, Serialize};

use super::errors::Status;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub status: Status,
    pub token: String,
}
