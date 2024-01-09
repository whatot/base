use anyhow::anyhow;
use argon2::Argon2;
use axum::{extract::State, http::StatusCode};
use jsonwebtoken::{encode, EncodingKey, Header};
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    api::{
        dto::{
            errors::{AppError, Status},
            login::LoginRequest,
            login::LoginResponse,
            TokenClaims,
        },
        middleware::json::CustomJson,
    },
    state::ApplicationState,
};
use std::sync::Arc;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<LoginRequest>,
) -> Result<CustomJson<LoginResponse>, AppError> {
    match entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&payload.username))
        .one(state.db_conn.load().as_ref())
        .await
    {
        Ok(Some(user)) => {
            if validate_password(&payload.password, &user.password).is_err() {
                return Err(AppError(
                    StatusCode::UNAUTHORIZED,
                    anyhow!("Password Mismatch"),
                ));
            }
        }
        Ok(None) => {
            return Err(AppError(
                StatusCode::UNAUTHORIZED,
                anyhow!("User is not an admin"),
            ));
        }
        Err(e) => {
            return Err(AppError(StatusCode::UNAUTHORIZED, e.into()));
        }
    }

    let secret = &state.settings.load().token_secret;
    let timeout = state.settings.load().token_timeout_seconds;
    let token = calc_token(timeout, payload, secret);

    let response = LoginResponse {
        status: Status::Success,
        token,
    };

    Ok(CustomJson(response))
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
    .unwrap_or("".to_string())
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))
}
