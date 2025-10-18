use super::handlers::{
    ListState, lpop_handler, lpush_handler, lrange_handler, rpop_handler, rpush_handler,
};
use crate::application::controllers::ListController;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn list_routes(controller: Arc<ListController>) -> Router {
    let state = ListState { controller };
    Router::new()
        .route("/lpush/:key", post(lpush_handler))
        .route("/rpush/:key", post(rpush_handler))
        .route("/lpop/:key", post(lpop_handler))
        .route("/rpop/:key", post(rpop_handler))
        .route("/lrange/:key", post(lrange_handler))
        .with_state(state)
}
