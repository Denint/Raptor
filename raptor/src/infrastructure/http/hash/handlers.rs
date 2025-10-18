use super::dtos::{
    HDelResponse, HExistsResponse, HGetResponse, HKeysResponse, HLenResponse, HSetResponse,
    HValsResponse,
};
use crate::application::controllers::HashController;
use crate::{domain::value_objects::key::Key, infrastructure::http::error::ApiError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use bytes::Bytes;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct HashState {
    pub controller: Arc<HashController>,
}

#[derive(Deserialize)]
pub struct HSetRequest {
    pub value: Vec<u8>,
}

#[derive(Deserialize)]
pub struct HDelRequest {
    pub fields: Vec<String>,
}

#[instrument(skip(state, body))]
pub async fn hset_handler(
    State(state): State<HashState>,
    Path((key, field)): Path<(String, String)>,
    body: Bytes,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let is_new_entry = state.controller.hset(key, field, body.to_vec()).await?;
    let response = HSetResponse { is_new_entry };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn hget_handler(
    State(state): State<HashState>,
    Path((key, field)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let field_clone = field.clone();
    let value = state.controller.hget(key, field).await?;
    let response = HGetResponse {
        field: field_clone,
        value: value.map(|v| general_purpose::STANDARD.encode(v)),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn hdel_handler(
    State(state): State<HashState>,
    Path(key): Path<String>,
    Json(body): Json<HDelRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let deleted_count = state.controller.hdel(key, body.fields).await?;
    let response = HDelResponse { deleted_count };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn hexists_handler(
    State(state): State<HashState>,
    Path((key, field)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let exists = state.controller.hexists(key, field).await?;
    let response = HExistsResponse { exists };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn hkeys_handler(
    State(state): State<HashState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let keys = state.controller.hkeys(key).await?;
    let response = HKeysResponse { keys };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn hvals_handler(
    State(state): State<HashState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let values = state.controller.hvals(key).await?;
    let response = HValsResponse {
        values: values
            .into_iter()
            .map(|v| general_purpose::STANDARD.encode(v))
            .collect(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn hlen_handler(
    State(state): State<HashState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let len = state.controller.hlen(key).await?;
    let response = HLenResponse { len };
    Ok((StatusCode::OK, Json(response)).into_response())
}
