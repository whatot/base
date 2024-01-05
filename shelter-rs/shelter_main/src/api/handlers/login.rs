use anyhow::anyhow;
use argon2::Argon2;
use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    api::dto::{login::LoginRequest, login::LoginResponse, TokenClaims},
    state::ApplicationState,
};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if let Ok(Some(user)) = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&payload.username))
        .one(state.db_conn.load().as_ref())
        .await
    {
        if validate_password(&payload.password, &user.password).is_err() {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let secret = &state.settings.load().token_secret;
    let timeout = state.settings.load().token_timeout_seconds;
    let token = calc_token(timeout, payload, secret);

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}

fn calc_token(timeout: i64, payload: LoginRequest, secret: &String) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(timeout)).timestamp() as usize;

    let claims = TokenClaims {
        sub: payload.username,
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))
}
