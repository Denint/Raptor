use crate::application::controllers::SortedSetController;
use crate::infrastructure::http::sorted_set::handlers::{
    SortedSetState, zadd_handler, zcard_handler, zrange_handler, zrem_handler, zscore_handler,
};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn sorted_set_routes(controller: Arc<SortedSetController>) -> Router {
    let state = SortedSetState { controller };
    Router::new()
        .route("/zadd/:key", post(zadd_handler))
        .route("/zrange/:key", post(zrange_handler))
        .route("/zrem/:key", post(zrem_handler))
        .route("/zscore/:key/:member", get(zscore_handler))
        .route("/zcard/:key", get(zcard_handler))
        .with_state(state)
}
