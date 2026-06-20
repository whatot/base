use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    api::dto::{
        TokenClaims,
        errors::{ErrorResponse, Status},
    },
    state::ApplicationState,
};
use std::sync::Arc;

pub async fn auth(
    State(state): State<Arc<ApplicationState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: Status::Error,
            message: "Missing bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let secret = &state.settings.load().token_secret;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: Status::Error,
            message: "Invalid bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
