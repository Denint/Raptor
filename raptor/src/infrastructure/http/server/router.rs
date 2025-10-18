use crate::application::controllers::Controllers;
use crate::infrastructure::http::counter::routes::counter_routes;
use crate::infrastructure::http::hash::routes::hash_routes;
use crate::infrastructure::http::list::routes::list_routes;
use crate::infrastructure::http::multi_key::routes::multi_key_routes;
use crate::infrastructure::http::set::routes::set_routes;
use crate::infrastructure::http::single_key::routes::single_key_routes;
use crate::infrastructure::http::sorted_set::routes::sorted_set_routes;

use axum::Json;
use axum::Router;
use axum::routing::get;
use serde_json::json;

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub struct RouterComponents {
    controllers: Controllers,
}

impl RouterComponents {
    pub fn new(controllers: Controllers) -> Self {
        Self { controllers }
    }

    pub fn build_router(&self) -> Router {
        Router::new()
            .route("/health", get(health_handler))
            .nest(
                "/single_key",
                single_key_routes(self.controllers.single_key.clone()),
            )
            .nest("/counter", counter_routes(self.controllers.counter.clone()))
            .nest("/hash", hash_routes(self.controllers.hash.clone()))
            .nest("/list", list_routes(self.controllers.list.clone()))
            .nest(
                "/multi_key",
                multi_key_routes(self.controllers.multi_key.clone()),
            )
            .nest("/set", set_routes(self.controllers.set.clone()))
            .nest(
                "/sorted_set",
                sorted_set_routes(self.controllers.sorted_set.clone()),
            )
    }
}
