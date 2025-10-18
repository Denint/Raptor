use super::dtos::{LPopResponse, LPushResponse, LRangeResponse, RPopResponse, RPushResponse};
use crate::application::controllers::ListController;
use crate::{domain::value_objects::key::Key, infrastructure::http::error::ApiError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct ListState {
    pub controller: Arc<ListController>,
}

#[derive(Deserialize)]
pub struct ListPushRequest {
    pub values: Vec<String>,
}

#[derive(Deserialize)]
pub struct LRangeRequest {
    pub start: isize,
    pub stop: isize,
}

#[instrument(skip(state, body))]
pub async fn lpush_handler(
    State(state): State<ListState>,
    Path(key): Path<String>,
    Json(body): Json<ListPushRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let decoded_values = body
        .values
        .into_iter()
        .map(|v| general_purpose::STANDARD.decode(v))
        .collect::<Result<Vec<_>, _>>()?;
    let new_length = state.controller.lpush(key, decoded_values).await?;
    let response = LPushResponse { new_length };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn rpush_handler(
    State(state): State<ListState>,
    Path(key): Path<String>,
    Json(body): Json<ListPushRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let decoded_values = body
        .values
        .into_iter()
        .map(|v| general_purpose::STANDARD.decode(v))
        .collect::<Result<Vec<_>, _>>()?;
    let new_length = state.controller.rpush(key, decoded_values).await?;
    let response = RPushResponse { new_length };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn lpop_handler(
    State(state): State<ListState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let value = state.controller.lpop(key).await?;
    let response = LPopResponse {
        value: value.map(|v| general_purpose::STANDARD.encode(v)),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn rpop_handler(
    State(state): State<ListState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let value = state.controller.rpop(key).await?;
    let response = RPopResponse {
        value: value.map(|v| general_purpose::STANDARD.encode(v)),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn lrange_handler(
    State(state): State<ListState>,
    Path(key): Path<String>,
    Json(body): Json<LRangeRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let values = state.controller.lrange(key, body.start, body.stop).await?;
    let response = LRangeResponse {
        values: values
            .into_iter()
            .map(|v| general_purpose::STANDARD.encode(v))
            .collect(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}
