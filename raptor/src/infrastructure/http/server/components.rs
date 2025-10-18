use crate::infrastructure::persistence::InMemoryStorage;
use axum::Router;
use std::sync::Arc;

use super::router::RouterComponents;
use crate::application::controllers::Controllers;

pub struct ServerComponents {
    controllers: Controllers,
}

impl ServerComponents {
    pub fn new(in_memory_store: Arc<InMemoryStorage>) -> Self {
        let controllers = Controllers::new(&in_memory_store);
        Self { controllers }
    }

    pub fn build_router(&self) -> Router {
        let router_components = RouterComponents::new(self.controllers.clone());
        router_components.build_router()
    }
}
