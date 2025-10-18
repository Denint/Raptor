pub use lru::*;
pub use smart_lru::*;
pub use basic::*;
pub use ttl::*;
pub use counter::*;
pub use multi::*;
pub use hash::*;
pub use list::*;
pub use sorted_set::*;
pub use set::*;
pub use array::*;

use crate::{
    domain::{
        errors::DomainError,
        entities::stored_value::{StoredValue, get_unix_timestamp},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::config::Config,
};
use dashmap::DashMap;
use std::sync::Arc;
use tracing::info;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::value::Value;

    fn create_test_storage() -> InMemoryStorage {
        let config = Arc::new(Config {
            port: 3000,
            log_format: crate::infrastructure::config::LogFormat::Json,
            max_memory_bytes: 1024 * 1024,
            snapshot_interval_seconds: Some(10),
            audit_log_path: None,
            snapshot_path: Some("./test_snapshot.bin".to_string()),
            smart_lru_eviction_threshold: 0.8,
            smart_lru_check_interval_seconds: 30,
            smart_lru_min_eviction_ratio: 0.05,
            smart_lru_max_eviction_ratio: 0.2,
            smart_lru_ttl_weight: 0.4,
            smart_lru_access_weight: 0.4,
            smart_lru_size_weight: 0.2,
        });
        InMemoryStorage::new(config)
    }

    #[tokio::test]
    async fn test_in_memory_storage_new() {
        let storage = create_test_storage();
        assert_eq!(storage.store.len(), 0);
    }

    #[tokio::test]
    async fn test_in_memory_storage_default() {
        let storage = InMemoryStorage::default();
        assert_eq!(storage.store.len(), 0);
    }

}

#[derive(Clone)]
pub struct InMemoryStorage {
    pub(crate) store: DashMap<Key, StoredValue>,
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

        let storage = Self {
            store: DashMap::new(),
            config: config.clone(),
        };

        if config.max_memory_bytes > 0 {
            let smart_lru_manager = SmartLruManager::new(
                config.smart_lru_eviction_threshold,
                std::time::Duration::from_secs(config.smart_lru_check_interval_seconds),
                config.smart_lru_min_eviction_ratio,
                config.smart_lru_max_eviction_ratio,
                config.smart_lru_ttl_weight,
                config.smart_lru_access_weight,
                config.smart_lru_size_weight,
            );
            storage.start_smart_lru_manager(smart_lru_manager);
            info!("Smart LRU manager started with configuration: threshold={:.1}%, interval={}s, ttl_weight={:.1}, access_weight={:.1}, size_weight={:.1}",
                config.smart_lru_eviction_threshold * 100.0,
                config.smart_lru_check_interval_seconds,
                config.smart_lru_ttl_weight,
                config.smart_lru_access_weight,
                config.smart_lru_size_weight
            );
        }

        storage
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
