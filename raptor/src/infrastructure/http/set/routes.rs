use super::handlers::{
    SetState, sadd_handler, scard_handler, sismember_handler, smembers_handler, srem_handler,
};
use crate::application::controllers::SetController;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn set_routes(controller: Arc<SetController>) -> Router {
    let state = SetState { controller };
    Router::new()
        .route("/sadd/:key", post(sadd_handler))
        .route("/smembers/:key", get(smembers_handler))
        .route("/srem/:key", post(srem_handler))
        .route("/sismember/:key", post(sismember_handler))
        .route("/scard/:key", get(scard_handler))
        .with_state(state)
}
