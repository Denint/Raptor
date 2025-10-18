use crate::application::controllers::SingleKeyController;
use crate::infrastructure::http::single_key::handlers::{
    SingleKeyState, array_append_handler, array_get_handler, array_length_handler,
    array_set_handler, array_slice_handler, array_update_handler, delete_handler, expire_handler,
    get_handler, persist_handler, set_handler, set_with_ttl_handler, ttl_handler,
};
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn single_key_routes(controller: Arc<SingleKeyController>) -> Router {
    let state = SingleKeyState { controller };
    Router::new()
        .route("/get/:key", get(get_handler))
        .route("/set/:key", post(set_handler))
        .route("/del/:key", delete(delete_handler))
        .route("/ttl/:key", get(ttl_handler))
        .route("/persist/:key", post(persist_handler))
        .route("/expire/:key", post(expire_handler))
        .route("/set_with_ttl/:key", post(set_with_ttl_handler))
        .route("/array/:key", post(array_set_handler))
        .route("/array/get/:key", post(array_get_handler))
        .route("/array/append/:key", put(array_append_handler))
        .route("/array/slice/:key", post(array_slice_handler))
        .route("/array/update/:key", put(array_update_handler))
        .route("/array/len/:key", get(array_length_handler))
        .with_state(state)
}
