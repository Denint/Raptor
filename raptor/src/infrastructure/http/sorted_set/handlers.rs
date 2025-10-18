use super::dtos::{
    ZAddRequest, ZAddResponse, ZCardResponse, ZRangeRequest, ZRemRequest, ZRemResponse,
    ZScoreResponse,
};
use crate::application::controllers::SortedSetController;
use crate::{domain::value_objects::key::Key, infrastructure::http::error::ApiError};
use axum::{
    Json,
    extract::{Path, State},
};
use base64::{Engine as _, engine::general_purpose};
use std::sync::Arc;

#[derive(Clone)]
pub struct SortedSetState {
    pub controller: Arc<SortedSetController>,
}

pub async fn zadd_handler(
    State(state): State<SortedSetState>,
    Path(key): Path<String>,
    Json(request): Json<ZAddRequest>,
) -> Result<Json<ZAddResponse>, ApiError> {
    let member = general_purpose::STANDARD.decode(&request.member)?;
    let is_new_entry = state
        .controller
        .zadd(Key::from(key), request.score, member)
        .await?;
    Ok(Json(ZAddResponse { is_new_entry }))
}

pub async fn zrange_handler(
    State(state): State<SortedSetState>,
    Path(key): Path<String>,
    Json(request): Json<ZRangeRequest>,
) -> Result<Json<Vec<String>>, ApiError> {
    let members = state
        .controller
        .zrange(Key::from(key), request.start, request.stop)
        .await?;
    let encoded_members = members
        .into_iter()
        .map(|m| general_purpose::STANDARD.encode(m))
        .collect();
    Ok(Json(encoded_members))
}

pub async fn zrem_handler(
    State(state): State<SortedSetState>,
    Path(key): Path<String>,
    Json(request): Json<ZRemRequest>,
) -> Result<Json<ZRemResponse>, ApiError> {
    let decoded_members = request
        .members
        .into_iter()
        .map(|m| general_purpose::STANDARD.decode(m))
        .collect::<Result<Vec<_>, _>>()?;
    let removed_count = state
        .controller
        .zrem(Key::from(key), decoded_members)
        .await?;
    Ok(Json(ZRemResponse { removed_count }))
}

pub async fn zscore_handler(
    State(state): State<SortedSetState>,
    Path((key, member)): Path<(String, String)>,
) -> Result<Json<ZScoreResponse>, ApiError> {
    let decoded_member = general_purpose::STANDARD.decode(&member)?;
    let score = state
        .controller
        .zscore(Key::from(key), &decoded_member)
        .await?;
    Ok(Json(ZScoreResponse { score }))
}

pub async fn zcard_handler(
    State(state): State<SortedSetState>,
    Path(key): Path<String>,
) -> Result<Json<ZCardResponse>, ApiError> {
    let card = state.controller.zcard(Key::from(key)).await?;
    Ok(Json(ZCardResponse { card }))
}
