use super::dtos::{MDelRequest, MDelResponse, MGetRequest, MGetResponse, MSetRequest};
use crate::application::controllers::MultiKeyController;
use crate::domain::value_objects::key::Key;
use crate::infrastructure::http::error::ApiError;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Clone)]
pub struct MultiKeyState {
    pub controller: Arc<MultiKeyController>,
}

#[instrument(skip(state, body))]
pub async fn mset_handler(
    State(state): State<MultiKeyState>,
    Json(body): Json<MSetRequest>,
) -> Result<Response, ApiError> {
    let pairs = body
        .pairs
        .into_iter()
        .map(|p| Ok::<_, ApiError>((Key::new(p.key)?, p.value.into())))
        .collect::<Result<Vec<_>, _>>()?;
    state.controller.mset(pairs).await?;
    Ok(StatusCode::OK.into_response())
}

#[instrument(skip(state, body))]
pub async fn mget_handler(
    State(state): State<MultiKeyState>,
    Json(body): Json<MGetRequest>,
) -> Result<Response, ApiError> {
    let keys = body
        .keys
        .into_iter()
        .map(Key::new)
        .collect::<Result<Vec<_>, _>>()?;
    let values = state.controller.mget(keys).await?;
    let response = MGetResponse {
        values: values.into_iter().map(|v| v.map(Into::into)).collect(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}

#[instrument(skip(state, body))]
pub async fn mdel_handler(
    State(state): State<MultiKeyState>,
    Json(body): Json<MDelRequest>,
) -> Result<Response, ApiError> {
    let keys = body
        .keys
        .into_iter()
        .map(Key::new)
        .collect::<Result<Vec<_>, _>>()?;
    let values = state.controller.mdel(keys).await?;
    let response = MDelResponse {
        values: values.into_iter().map(|v| v.map(Into::into)).collect(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}
