use crate::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;

pub struct Repositories {
    pub basic: Arc<InMemoryStorage>,
    pub array: Arc<InMemoryStorage>,
    pub counter: Arc<InMemoryStorage>,
    pub hash: Arc<InMemoryStorage>,
    pub list: Arc<InMemoryStorage>,
    pub multi: Arc<InMemoryStorage>,
    pub set: Arc<InMemoryStorage>,
    pub sorted_set: Arc<InMemoryStorage>,
    pub _lru: Arc<InMemoryStorage>,
    pub _ttl: Arc<InMemoryStorage>,
}
