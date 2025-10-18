use super::dtos::{
    SAddRequest, SAddResponse, SCardResponse, SIsMemberRequest, SIsMemberResponse,
    SMembersResponse, SRemRequest, SRemResponse,
};
use crate::application::controllers::SetController;
use crate::{domain::value_objects::key::Key, infrastructure::http::error::ApiError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose};
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct SetState {
    pub controller: Arc<SetController>,
}

#[instrument(skip(state, body))]
pub async fn sadd_handler(
    State(state): State<SetState>,
    Path(key): Path<String>,
    Json(body): Json<SAddRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let decoded_members = body
        .members
        .into_iter()
        .map(|m| general_purpose::STANDARD.decode(m))
        .collect::<Result<Vec<_>, _>>()?;
    let added_count = state.controller.sadd(key, decoded_members).await?;
    let response = SAddResponse { added_count };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn smembers_handler(
    State(state): State<SetState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let members = state.controller.smembers(key).await?;
    let response = SMembersResponse {
        members: members
            .into_iter()
            .map(|m| general_purpose::STANDARD.encode(m))
            .collect(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn srem_handler(
    State(state): State<SetState>,
    Path(key): Path<String>,
    Json(body): Json<SRemRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let decoded_members = body
        .members
        .into_iter()
        .map(|m| general_purpose::STANDARD.decode(m))
        .collect::<Result<Vec<_>, _>>()?;
    let removed_count = state.controller.srem(key, decoded_members).await?;
    let response = SRemResponse { removed_count };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn sismember_handler(
    State(state): State<SetState>,
    Path(key): Path<String>,
    Json(body): Json<SIsMemberRequest>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let decoded_member = general_purpose::STANDARD.decode(&body.member)?;
    let is_member = state.controller.sismember(key, &decoded_member).await?;
    let response = SIsMemberResponse { is_member };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state))]
pub async fn scard_handler(
    State(state): State<SetState>,
    Path(key): Path<String>,
) -> Result<Response, ApiError> {
    let key = Key::new(key)?;
    let card = state.controller.scard(key).await?;
    let response = SCardResponse { card };
    Ok((StatusCode::OK, Json(response)).into_response())
}
