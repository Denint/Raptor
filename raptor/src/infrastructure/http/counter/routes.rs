use super::handlers::{CounterState, decr_handler, incr_handler, reset_handler};
use crate::application::controllers::CounterController;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn counter_routes(controller: Arc<CounterController>) -> Router {
    let state = CounterState { controller };
    Router::new()
        .route("/incr/:key", post(incr_handler))
        .route("/decr/:key", post(decr_handler))
        .route("/reset/:key", post(reset_handler))
        .with_state(state)
}
