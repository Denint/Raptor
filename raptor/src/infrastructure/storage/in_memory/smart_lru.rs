use crate::{
    domain::{
        entities::stored_value::{get_unix_timestamp, StoredValue},
        errors::DomainError,
        repositories::LruRepository,
        value_objects::value::Value,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use std::{
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::RwLock,
    time::{self, Instant},
};
use tracing::{debug, info, warn};


#[derive(Clone)]
pub struct SmartLruManager {

    pub eviction_threshold: f64,

    pub check_interval: Duration,

    pub min_eviction_ratio: f64,

    pub max_eviction_ratio: f64,

    pub ttl_weight: f64,

    pub access_weight: f64,

    pub size_weight: f64,
}

impl Default for SmartLruManager {
    fn default() -> Self {
        Self {
            eviction_threshold: 0.8,
            check_interval: Duration::from_secs(30),
            min_eviction_ratio: 0.05,
            max_eviction_ratio: 0.2,
            ttl_weight: 0.4,
            access_weight: 0.4,
            size_weight: 0.2,
        }
    }
}

impl SmartLruManager {
    pub fn new(
        eviction_threshold: f64,
        check_interval: Duration,
        min_eviction_ratio: f64,
        max_eviction_ratio: f64,
        ttl_weight: f64,
        access_weight: f64,
        size_weight: f64,
    ) -> Self {
        Self {
            eviction_threshold: eviction_threshold.clamp(0.1, 0.95),
            check_interval,
            min_eviction_ratio: min_eviction_ratio.clamp(0.01, 0.5),
            max_eviction_ratio: max_eviction_ratio.clamp(0.05, 0.8),
            ttl_weight: ttl_weight.clamp(0.0, 1.0),
            access_weight: access_weight.clamp(0.0, 1.0),
            size_weight: size_weight.clamp(0.0, 1.0),
        }
    }


    pub fn start_background_task(self, storage: Arc<InMemoryStorage>) {
        tokio::spawn(async move {
            info!(
                "Starting Smart LRU background task with threshold {:.1}%, interval {:?}",
                self.eviction_threshold * 100.0,
                self.check_interval
            );

            let mut interval = time::interval(self.check_interval);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = self.perform_eviction_check(&storage).await {
                            warn!("Smart LRU eviction check failed: {}", e);
                        }
                    }
                }
            }
        });
    }


    async fn perform_eviction_check(&self, storage: &InMemoryStorage) -> Result<(), DomainError> {

        if storage.config.max_memory_bytes == 0 {
            return Ok(());
        }

        let max_memory = storage.config.max_memory_bytes;
        let current_usage = storage.get_memory_usage().await?;
        let usage_ratio = current_usage as f64 / max_memory as f64;

        debug!(
            "Memory usage: {}/{} bytes ({:.1}%)",
            current_usage,
            max_memory,
            usage_ratio * 100.0
        );


        if usage_ratio >= self.eviction_threshold {
            let target_usage_ratio = self.eviction_threshold * 0.9;
            let target_memory = (max_memory as f64 * target_usage_ratio) as u64;
            let memory_to_free = current_usage.saturating_sub(target_memory);

            info!(
                "Memory usage {:.1}% exceeds threshold {:.1}%, need to free {} bytes",
                usage_ratio * 100.0,
                self.eviction_threshold * 100.0,
                memory_to_free
            );


            let ttl_evicted = storage.evict_expired_ttl().await?;
            if ttl_evicted > 0 {
                info!("Evicted {} expired TTL entries", ttl_evicted);
            }


            let current_usage_after_ttl = storage.get_memory_usage().await?;
            if current_usage_after_ttl <= target_memory {
                debug!("Memory freed by TTL eviction, no further action needed");
                return Ok(());
            }


            let remaining_to_free = current_usage_after_ttl.saturating_sub(target_memory);
            let score_evicted = self.evict_by_score(storage, remaining_to_free).await?;

            info!(
                "Smart LRU eviction completed: TTL={}, Score={}, Memory freed",
                ttl_evicted, score_evicted
            );
        }

        Ok(())
    }


    fn calculate_score(&self, entry: &StoredValue, key_size: u64, value_size: u64, current_time: u64) -> f64 {
        let ttl_score = match entry.ttl_remaining() {
            Some(remaining_ms) => {

                let ttl_hours = remaining_ms as f64 / (1000.0 * 3600.0);
                1.0 / (1.0 + ttl_hours)
            }
            None => 0.0,
        };

        let access_score = {

            let hours_since_access = (current_time.saturating_sub(entry.last_accessed)) as f64 / (1000.0 * 3600.0);
            hours_since_access / (hours_since_access + 1.0)
        };

        let size_score = {

            let total_size_kb = (key_size + value_size) as f64 / 1024.0;
            (total_size_kb / (total_size_kb + 100.0)).min(0.8)
        };


        ttl_score * self.ttl_weight + access_score * self.access_weight + size_score * self.size_weight
    }


    async fn evict_by_score(&self, storage: &InMemoryStorage, memory_to_free: u64) -> Result<usize, DomainError> {
        let current_time = get_unix_timestamp();


        let mut entries_with_score: Vec<_> = storage.store.iter()
            .filter_map(|entry| {
                let key_size = entry.key().as_str().len() as u64;
                let value_size = match &entry.value().value {
                    Value::String(v) => v.len() as u64,
                    Value::Integer(_) => 8,
                    Value::Hash(h) => {
                        h.iter().map(|(k, v)| k.len() as u64 + v.len() as u64).sum()
                    }
                    Value::List(l) => {
                        l.iter().map(|v| v.len() as u64).sum()
                    }
                    Value::Set(s) => {
                        s.iter().map(|v| v.len() as u64).sum()
                    }
                    Value::SortedSet(ss) => {
                        ss.iter().map(|(_, v)| v.len() as u64 + 8).sum()
                    }
                    Value::Array(a) => {
                        a.iter().map(|v| v.clone().into_vec().map(|vec| vec.len() as u64).unwrap_or(0)).sum()
                    }
                };

                let score = self.calculate_score(entry.value(), key_size, value_size, current_time);
                Some((entry.key().clone(), score, key_size + value_size))
            })
            .collect();


        entries_with_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let total_entries = entries_with_score.len();
        let min_to_evict = (total_entries as f64 * self.min_eviction_ratio) as usize;
        let max_to_evict = (total_entries as f64 * self.max_eviction_ratio) as usize;

        let mut evicted_count = 0;
        let mut memory_freed = 0u64;


        for (key, score, size) in entries_with_score {
            if memory_freed >= memory_to_free && evicted_count >= min_to_evict {
                break;
            }
            if evicted_count >= max_to_evict {
                break;
            }

            storage.store.remove(&key);
            memory_freed += size;
            evicted_count += 1;

            debug!(
                "Evicted key by score: {} (score: {:.3}, size: {} bytes)",
                key.as_str(), score, size
            );
        }

        info!(
            "Score-based eviction: removed {}/{} entries, freed {} bytes",
            evicted_count, total_entries, memory_freed
        );

        Ok(evicted_count)
    }
}

#[async_trait::async_trait]
impl LruRepository for InMemoryStorage {
    async fn get_memory_usage(&self) -> Result<u64, DomainError> {
        let mut total_size = 0;
        for entry in self.store.iter() {
            let key_size = entry.key().as_str().len() as u64;
            let value_size = match &entry.value().value {
                Value::String(v) => v.len() as u64,
                Value::Integer(_) => 8,
                Value::Hash(h) => {
                    h.iter().map(|(k, v)| k.len() as u64 + v.len() as u64).sum()
                }
                Value::List(l) => {
                    l.iter().map(|v| v.len() as u64).sum()
                }
                Value::Set(s) => {
                    s.iter().map(|v| v.len() as u64).sum()
                }
                Value::SortedSet(ss) => {
                    ss.iter().map(|(_, v)| v.len() as u64 + 8).sum()
                }
                Value::Array(a) => {
                    a.iter().map(|v| v.clone().into_vec().map(|vec| vec.len() as u64).unwrap_or(0)).sum()
                }
            };
            total_size += key_size + value_size;
        }
        Ok(total_size)
    }

    async fn evict_lru(&self, count: usize) -> Result<usize, DomainError> {
        let mut entries: Vec<(_, _)> = self
            .store
            .iter()
            .map(|e| (e.key().clone(), e.value().last_accessed))
            .collect();

        entries.sort_by_key(|(_, last_accessed)| *last_accessed);

        let mut evicted_count = 0;
        for (key, _) in entries.into_iter().take(count) {
            self.store.remove(&key);
            evicted_count += 1;
        }
        Ok(evicted_count)
    }
}


#[async_trait::async_trait]
pub trait SmartLruRepository: LruRepository {

    async fn evict_expired_ttl(&self) -> Result<usize, DomainError>;


    fn start_smart_lru_manager(&self, manager: SmartLruManager);
}

#[async_trait::async_trait]
impl SmartLruRepository for InMemoryStorage {
    async fn evict_expired_ttl(&self) -> Result<usize, DomainError> {
        let mut expired_keys = Vec::new();


        for entry in self.store.iter() {
            if entry.value().is_expired() {
                expired_keys.push(entry.key().clone());
            }
        }


        let mut evicted_count = 0;
        for key in expired_keys {
            self.store.remove(&key);
            evicted_count += 1;
        }

        Ok(evicted_count)
    }

    fn start_smart_lru_manager(&self, manager: SmartLruManager) {
        let storage = Arc::new(self.clone());
        manager.start_background_task(storage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::value_objects::{key::Key, value::Value},
        infrastructure::config::{create_test_config, Config, LogFormat},
    };
    use std::sync::Arc;

    fn create_test_storage() -> InMemoryStorage {
        let config = Arc::new(create_test_config());
        InMemoryStorage::new(config)
    }

    #[tokio::test]
    async fn test_smart_lru_manager_default() {
        let manager = SmartLruManager::default();
        assert_eq!(manager.eviction_threshold, 0.8);
        assert_eq!(manager.check_interval, Duration::from_secs(30));
        assert_eq!(manager.min_eviction_ratio, 0.05);
        assert_eq!(manager.max_eviction_ratio, 0.2);
    }

    #[tokio::test]
    async fn test_smart_lru_manager_new() {
        let manager = SmartLruManager::new(0.9, Duration::from_secs(60), 0.1, 0.3, 0.5, 0.3, 0.2);
        assert_eq!(manager.eviction_threshold, 0.9);
        assert_eq!(manager.check_interval, Duration::from_secs(60));
        assert_eq!(manager.min_eviction_ratio, 0.1);
        assert_eq!(manager.max_eviction_ratio, 0.3);
        assert_eq!(manager.ttl_weight, 0.5);
        assert_eq!(manager.access_weight, 0.3);
        assert_eq!(manager.size_weight, 0.2);
    }

    #[tokio::test]
    async fn test_evict_expired_ttl() {
        let storage = create_test_storage();
        let key1 = Key::new("key1".to_string()).unwrap();
        let key2 = Key::new("key2".to_string()).unwrap();


        storage.set(&key1, Value::String(b"expired".to_vec())).await.unwrap();
        storage.expire(&key1, 0).await.unwrap();

        storage.set(&key2, Value::String(b"valid".to_vec())).await.unwrap();


        tokio::time::sleep(Duration::from_millis(10)).await;


        let evicted = storage.evict_expired_ttl().await.unwrap();
        assert_eq!(evicted, 1);


        let value1 = storage.get(&key1).await.unwrap();
        let value2 = storage.get(&key2).await.unwrap();

        assert!(value1.is_none());
        assert!(value2.is_some());
    }

    #[tokio::test]
    async fn test_calculate_score() {
        let manager = SmartLruManager::default();
        let current_time = get_unix_timestamp();


        let recent_value = StoredValue::new(Value::String(b"test".to_vec()));
        let score1 = manager.calculate_score(&recent_value, 10, 100, current_time);
        assert!(score1 < 0.5);


        let expired_value = StoredValue::with_ttl_ms(Value::String(b"expired".to_vec()), 1);
        tokio::time::sleep(Duration::from_millis(5)).await;
        let score2 = manager.calculate_score(&expired_value, 10, 100, get_unix_timestamp());
        assert!(score2 > score1);
    }
}
