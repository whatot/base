use crate::api::dto::dogs::DogCreateResponse;
use crate::api::dto::dogs::DogGetResponse;
use crate::api::dto::dogs::DogListResponse;
use crate::api::dto::errors::AppError;
use crate::api::dto::errors::Status;
use crate::api::dto::TokenClaims;
use crate::api::middleware::json::CustomJson;
use crate::state::ApplicationState;
use axum::extract::Path;
use axum::extract::State;
use axum::Extension;
use axum::Json;
use entity::dog::DogCreateRequest;
use sea_orm::EntityTrait;
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

pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<DogListResponse>, AppError> {
    let dogs = entity::dog::Entity::find()
        .all(state.db_conn.load().as_ref())
        .await?;
    let n = dogs.len();

    let response = DogListResponse {
        status: Status::Success,
        data: dogs,
    };

    tracing::info!("number of dogs: {}", n);

    Ok(Json(response))
}

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(dog_id): Path<i32>,
) -> Result<Json<DogGetResponse>, AppError> {
    let dog = entity::dog::Entity::find_by_id(dog_id)
        .one(state.db_conn.load().as_ref())
        .await?;

    let response = DogGetResponse {
        status: Status::Success,
        data: dog,
    };

    Ok(Json(response))
}
