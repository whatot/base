use crate::api::dto::dogs::DogCreateResponse;
use crate::api::dto::errors::AppError;
use crate::api::dto::errors::Status;
use crate::api::dto::TokenClaims;
use crate::api::middleware::json::CustomJson;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::Extension;
use entity::dog::DogCreateRequest;
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<DogCreateRequest>,
) -> Result<CustomJson<DogCreateResponse>, AppError> {
    let dog_active_model = payload.into_active_model();
    let dog_model = dog_active_model.save(state.db_conn.load().as_ref()).await?;
    let dog = dog_model.try_into_model()?;

    let response = DogCreateResponse {
        status: Status::Success,
        data: Some(dog),
    };

    Ok(CustomJson(response))
}
