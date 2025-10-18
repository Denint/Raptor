use super::handlers::{
    HashState, hdel_handler, hexists_handler, hget_handler, hkeys_handler, hlen_handler,
    hset_handler, hvals_handler,
};
use crate::application::controllers::HashController;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn hash_routes(controller: Arc<HashController>) -> Router {
    let state = HashState { controller };
    Router::new()
        .route("/hset/:key/:field", post(hset_handler))
        .route("/hget/:key/:field", get(hget_handler))
        .route("/hdel/:key", post(hdel_handler))
        .route("/hexists/:key/:field", get(hexists_handler))
        .route("/hkeys/:key", get(hkeys_handler))
        .route("/hvals/:key", get(hvals_handler))
        .route("/hlen/:key", get(hlen_handler))
        .with_state(state)
}
