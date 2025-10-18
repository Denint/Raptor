use crate::{
    domain::{errors::DomainError, repositories::LruRepository, value_objects::value::Value},
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl LruRepository for InMemoryStorage {
    async fn get_memory_usage(&self) -> Result<u64, DomainError> {
        let mut total_size = 0;
        for entry in self.store.iter() {
            let key_size = entry.key().as_str().len() as u64;
            let value_size = match &entry.value().value {
                Value::String(v) => v.len() as u64,
                Value::Integer(_) => 8,
                Value::Hash(h) => h.iter().map(|(k, v)| k.len() as u64 + v.len() as u64).sum(),
                Value::List(l) => l.iter().map(|v| v.len() as u64).sum(),
                Value::Set(s) => s.iter().map(|v| v.len() as u64).sum(),
                Value::SortedSet(ss) => ss.iter().map(|(_, v)| v.len() as u64 + 8).sum(),
                Value::Array(a) => a
                    .iter()
                    .map(|v| {
                        v.clone()
                            .into_vec()
                            .map(|vec| vec.len() as u64)
                            .unwrap_or(0)
                    })
                    .sum(),
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
