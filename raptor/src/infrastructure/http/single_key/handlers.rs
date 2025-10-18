use super::dtos::{
    ArrayAppendRequest, ArrayAppendResponse, ArrayGetRequest, ArrayGetResponse,
    ArrayLengthResponse, ArraySetRequest, ArraySliceRequest, ArraySliceResponse,
    ArrayUpdateRequest, ArrayUpdateResponse,
};
use crate::application::controllers::SingleKeyController;
use crate::{
    application::dtos::value_dto::ValueDto,
    domain::value_objects::{key::Key, value::Value},
    infrastructure::http::error::ApiError,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct SingleKeyState {
    pub controller: Arc<SingleKeyController>,
}

#[derive(Deserialize)]
pub struct SetWithTtlRequest {
    pub value: ValueDto,
    pub ttl_seconds: u64,
}

#[derive(Deserialize)]
pub struct ExpireRequest {
    pub ttl_seconds: u64,
}

#[instrument(skip(state))]
pub async fn get_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;

    match state.controller.get_key(key).await? {
        Some(value) => Ok((StatusCode::OK, axum::Json(value)).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Key not found".to_string()).into_response()),
    }
}

#[instrument(skip(state, body))]
pub async fn set_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    body: Bytes,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let value = serde_json::from_slice::<ValueDto>(&body)
        .map(|dto| dto.into())
        .unwrap_or_else(|_| Value::String(body.to_vec()));

    let is_new = state.controller.set_key(key, value).await?;
    if is_new {
        Ok(StatusCode::CREATED.into_response())
    } else {
        Ok(StatusCode::OK.into_response())
    }
}

#[instrument(skip(state))]
pub async fn delete_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;

    match state.controller.delete_key(key).await? {
        Some(_) => Ok((StatusCode::OK, "Key deleted".to_string()).into_response()),
        None => Ok((StatusCode::OK, Json(serde_json::Value::Null)).into_response()),
    }
}

#[instrument(skip(state, body))]
pub async fn set_with_ttl_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<SetWithTtlRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let value: Value = body.value.into();
    state
        .controller
        .set_with_ttl(key, value, body.ttl_seconds)
        .await?;
    Ok(StatusCode::OK.into_response())
}

#[instrument(skip(state))]
pub async fn ttl_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let ttl = state.controller.ttl(key).await?;
    Ok(Json(json!({ "ttl": ttl })).into_response())
}

#[instrument(skip(state))]
pub async fn persist_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let persisted = state.controller.persist(key).await?;
    Ok(Json(json!({ "persisted": persisted })).into_response())
}

#[instrument(skip(state, body))]
pub async fn expire_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ExpireRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let expired = state.controller.expire(key, body.ttl_seconds).await?;
    Ok(Json(json!({ "expired": expired })).into_response())
}

#[instrument(skip(state, body))]
pub async fn array_set_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ArraySetRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    state.controller.array_set(key, body.values).await?;
    Ok((StatusCode::OK, "Array set successfully".to_string()).into_response())
}

#[instrument(skip(state, body))]
pub async fn array_get_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ArrayGetRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let values = state.controller.array_get(key, body.indices).await?;
    let response = ArrayGetResponse { values };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn array_append_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ArrayAppendRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let new_length = state.controller.array_append(key, body.values).await?;
    let response = ArrayAppendResponse { new_length };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn array_slice_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ArraySliceRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let values = state
        .controller
        .array_slice(key, body.start, body.end)
        .await?;
    let response = ArraySliceResponse { values };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn array_update_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
    Json(body): Json<ArrayUpdateRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let updates: Vec<(usize, Value)> = body
        .updates
        .into_iter()
        .map(|entry| (entry.index, entry.value))
        .collect();
    let updated_count = state.controller.array_update(key, updates).await?;
    let response = ArrayUpdateResponse { updated_count };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn array_length_handler(
    State(state): State<SingleKeyState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let length = state.controller.array_length(key).await?;
    let response = ArrayLengthResponse { length };
    Ok((StatusCode::OK, Json(response)).into_response())
}
