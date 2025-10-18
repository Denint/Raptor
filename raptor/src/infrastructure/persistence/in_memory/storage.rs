use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::LruRepository,
        value_objects::key::Key,
    },
    infrastructure::config::Config,
};
use dashmap::DashMap;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct InMemoryStorage {
    pub store: DashMap<Key, StoredValue>,
    config: Arc<Config>,
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new(Arc::new(Config::from_env().unwrap()))
    }
}

impl InMemoryStorage {
    pub fn new(config: Arc<Config>) -> Self {
        info!(
            "Initializing InMemoryStorage with max_memory: {} bytes",
            config.max_memory_bytes
        );
        Self {
            store: DashMap::new(),
            config,
        }
    }

    pub fn get_all_entries(&self) -> Vec<(Key, StoredValue)> {
        self.store
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }

    pub fn insert_entry(&self, key: Key, value: StoredValue) {
        self.store.insert(key, value);
    }

    pub async fn get_all_data(&self) -> Vec<(String, String)> {
        self.store
            .iter()
            .map(|entry| (entry.key().to_string(), entry.value().to_string()))
            .collect()
    }

    pub async fn restore_data(&self, data: Vec<(String, String)>) {
        self.store.clear();
        for (key, value) in data {
            self.store.insert(Key::from(key), StoredValue::from(value));
        }
    }

    pub(crate) async fn check_memory_and_evict(&self) -> Result<(), DomainError> {
        if self.config.max_memory_bytes == 0 {
            return Ok(());
        }

        let max_memory = self.config.max_memory_bytes;
        let current_usage = self.get_memory_usage().await?;

        if current_usage > max_memory {
            let keys_to_evict = (self.store.len() / 10).max(1);
            self.evict_lru(keys_to_evict).await?;
        }

        Ok(())
    }
}
