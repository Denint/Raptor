use super::dtos::{CounterResponse, OptionCounterResponse};
use crate::application::controllers::CounterController;
use crate::{domain::value_objects::key::Key, infrastructure::http::error::ApiError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct CounterState {
    pub controller: Arc<CounterController>,
}

#[instrument(skip(state))]
pub async fn incr_handler(
    State(state): State<CounterState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;

    let value = state.controller.incr_counter(key).await?;
    let response = CounterResponse { value };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn decr_handler(
    State(state): State<CounterState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;

    let value = state.controller.decr_counter(key).await?;
    let response = CounterResponse { value };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn reset_handler(
    State(state): State<CounterState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;

    let value = state.controller.reset_counter(key).await?;
    let response = OptionCounterResponse { value };
    Ok((StatusCode::OK, Json(response)).into_response())
}
