use super::handlers::{MultiKeyState, mdel_handler, mget_handler, mset_handler};
use crate::application::controllers::MultiKeyController;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn multi_key_routes(controller: Arc<MultiKeyController>) -> Router {
    let state = MultiKeyState { controller };
    Router::new()
        .route("/mset", post(mset_handler))
        .route("/mget", post(mget_handler))
        .route("/mdel", post(mdel_handler))
        .with_state(state)
}
