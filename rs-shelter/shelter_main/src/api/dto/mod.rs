use serde::{Deserialize, Serialize};

pub mod dogs;
pub mod errors;
pub mod login;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    // the token subject (generally username or user id)
    pub sub: String,
    // the unix timestamp of the token generation
    pub iat: usize,
    // the unix timestamp of the token expiration time
    pub exp: usize,
}
