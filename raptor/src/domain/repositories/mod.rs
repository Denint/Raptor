use crate::domain::{
    errors::DomainError,
    value_objects::{key::Key, value::Value},
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::mock;

#[async_trait]
pub trait BasicRepository: Send + Sync {
    async fn get(&self, key: &Key) -> Result<Option<Value>, DomainError>;
    async fn set(&self, key: &Key, value: Value) -> Result<bool, DomainError>;
    async fn delete(&self, key: &Key) -> Result<Option<Value>, DomainError>;
}

#[async_trait]
pub trait TtlRepository: Send + Sync {
    async fn set_with_ttl(
        &self,
        key: &Key,
        value: Value,
        ttl_seconds: u64,
    ) -> Result<(), DomainError>;
    async fn set_with_ttl_ms(
        &self,
        key: &Key,
        value: Value,
        ttl_ms: u64,
    ) -> Result<(), DomainError>;
    async fn expire(&self, key: &Key, ttl_seconds: u64) -> Result<bool, DomainError>;
    async fn ttl(&self, key: &Key) -> Result<Option<i64>, DomainError>;
    async fn persist(&self, key: &Key) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait CounterRepository: Send + Sync {
    async fn incr(&self, key: &Key, increment: i64) -> Result<i64, DomainError>;
    async fn decr(&self, key: &Key) -> Result<i64, DomainError>;
    async fn reset(&self, key: &Key) -> Result<Option<i64>, DomainError>;
}

#[async_trait]
pub trait MultiRepository: Send + Sync {
    async fn mset(&self, pairs: Vec<(Key, Value)>) -> Result<(), DomainError>;
    async fn mget(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError>;
    async fn mdel(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError>;
}

#[async_trait]
pub trait HashRepository: Send + Sync {
    async fn hset(&self, key: &Key, field: String, value: Vec<u8>) -> Result<bool, DomainError>;
    async fn hget(&self, key: &Key, field: &str) -> Result<Option<Vec<u8>>, DomainError>;
    async fn hdel(&self, key: &Key, fields: Vec<String>) -> Result<usize, DomainError>;
    async fn hexists(&self, key: &Key, field: &str) -> Result<bool, DomainError>;
    async fn hkeys(&self, key: &Key) -> Result<Vec<String>, DomainError>;
    async fn hvals(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError>;
    async fn hlen(&self, key: &Key) -> Result<usize, DomainError>;
}

#[async_trait]
pub trait ListRepository: Send + Sync {
    async fn lpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError>;
    async fn rpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError>;
    async fn lpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError>;
    async fn rpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError>;
    async fn lrange(
        &self,
        key: &Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, DomainError>;
}

#[async_trait]
pub trait SortedSetRepository: Send + Sync {
    async fn zadd(&self, key: &Key, score: f64, member: Vec<u8>) -> Result<bool, DomainError>;
    async fn zrange(
        &self,
        key: &Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, DomainError>;
    async fn zrem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
    async fn zscore(&self, key: &Key, member: &[u8]) -> Result<Option<f64>, DomainError>;
    async fn zcard(&self, key: &Key) -> Result<usize, DomainError>;
}

#[async_trait]
pub trait SetRepository: Send + Sync {
    async fn sadd(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
    async fn smembers(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError>;
    async fn srem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
    async fn sismember(&self, key: &Key, member: &[u8]) -> Result<bool, DomainError>;
    async fn scard(&self, key: &Key) -> Result<usize, DomainError>;
}

#[async_trait]
pub trait ArrayRepository: Send + Sync {
    async fn array_set(&self, key: &Key, values: Vec<Value>) -> Result<(), DomainError>;
    async fn array_get(
        &self,
        key: &Key,
        indices: Vec<usize>,
    ) -> Result<Vec<Option<Value>>, DomainError>;
    async fn array_append(&self, key: &Key, values: Vec<Value>) -> Result<usize, DomainError>;
    async fn array_slice(
        &self,
        key: &Key,
        start: usize,
        end: Option<usize>,
    ) -> Result<Vec<Value>, DomainError>;
    async fn array_update(
        &self,
        key: &Key,
        updates: Vec<(usize, Value)>,
    ) -> Result<usize, DomainError>;
    async fn array_length(&self, key: &Key) -> Result<Option<usize>, DomainError>;
}

#[async_trait]
pub trait LruRepository: Send + Sync + 'static {
    async fn get_memory_usage(&self) -> Result<u64, DomainError>;
    async fn evict_lru(&self, count: usize) -> Result<usize, DomainError>;
}

#[cfg(test)]
mock! {
    pub BasicRepository {}

    #[async_trait]
    impl BasicRepository for BasicRepository {
        async fn get(&self, key: &Key) -> Result<Option<Value>, DomainError>;
        async fn set(&self, key: &Key, value: Value) -> Result<bool, DomainError>;
        async fn delete(&self, key: &Key) -> Result<Option<Value>, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub TtlRepository {}

    #[async_trait]
    impl TtlRepository for TtlRepository {
        async fn set_with_ttl(&self, key: &Key, value: Value, ttl_seconds: u64) -> Result<(), DomainError>;
        async fn set_with_ttl_ms(&self, key: &Key, value: Value, ttl_ms: u64) -> Result<(), DomainError>;
        async fn expire(&self, key: &Key, ttl_seconds: u64) -> Result<bool, DomainError>;
        async fn ttl(&self, key: &Key) -> Result<Option<i64>, DomainError>;
        async fn persist(&self, key: &Key) -> Result<bool, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub CounterRepository {}

    #[async_trait]
    impl CounterRepository for CounterRepository {
        async fn incr(&self, key: &Key, increment: i64) -> Result<i64, DomainError>;
        async fn decr(&self, key: &Key) -> Result<i64, DomainError>;
        async fn reset(&self, key: &Key) -> Result<Option<i64>, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub MultiRepository {}

    #[async_trait]
    impl MultiRepository for MultiRepository {
        async fn mset(&self, pairs: Vec<(Key, Value)>) -> Result<(), DomainError>;
        async fn mget(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError>;
        async fn mdel(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub HashRepository {}

    #[async_trait]
    impl HashRepository for HashRepository {
        async fn hset(&self, key: &Key, field: String, value: Vec<u8>) -> Result<bool, DomainError>;
        async fn hget(&self, key: &Key, field: &str) -> Result<Option<Vec<u8>>, DomainError>;
        async fn hdel(&self, key: &Key, fields: Vec<String>) -> Result<usize, DomainError>;
        async fn hexists(&self, key: &Key, field: &str) -> Result<bool, DomainError>;
        async fn hkeys(&self, key: &Key) -> Result<Vec<String>, DomainError>;
        async fn hvals(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError>;
        async fn hlen(&self, key: &Key) -> Result<usize, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub ListRepository {}

    #[async_trait]
    impl ListRepository for ListRepository {
        async fn lpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError>;
        async fn rpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError>;
        async fn lpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError>;
        async fn rpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError>;
        async fn lrange(
            &self,
            key: &Key,
            start: isize,
            stop: isize,
        ) -> Result<Vec<Vec<u8>>, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub SortedSetRepository {}

    #[async_trait]
    impl SortedSetRepository for SortedSetRepository {
        async fn zadd(&self, key: &Key, score: f64, member: Vec<u8>) -> Result<bool, DomainError>;
        async fn zrange(
            &self,
            key: &Key,
            start: isize,
            stop: isize,
        ) -> Result<Vec<Vec<u8>>, DomainError>;
        async fn zrem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
        async fn zscore(&self, key: &Key, member: &[u8]) -> Result<Option<f64>, DomainError>;
        async fn zcard(&self, key: &Key) -> Result<usize, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub SetRepository {}

    #[async_trait]
    impl SetRepository for SetRepository {
        async fn sadd(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
        async fn smembers(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError>;
        async fn srem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError>;
        async fn sismember(&self, key: &Key, member: &[u8]) -> Result<bool, DomainError>;
        async fn scard(&self, key: &Key) -> Result<usize, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub ArrayRepository {}

    #[async_trait]
    impl ArrayRepository for ArrayRepository {
        async fn array_set(&self, key: &Key, values: Vec<Value>) -> Result<(), DomainError>;
        async fn array_get(
            &self,
            key: &Key,
            indices: Vec<usize>,
        ) -> Result<Vec<Option<Value>>, DomainError>;
        async fn array_append(&self, key: &Key, values: Vec<Value>) -> Result<usize, DomainError>;
        async fn array_slice(
            &self,
            key: &Key,
            start: usize,
            end: Option<usize>,
        ) -> Result<Vec<Value>, DomainError>;
        async fn array_update(
            &self,
            key: &Key,
            updates: Vec<(usize, Value)>,
        ) -> Result<usize, DomainError>;
        async fn array_length(&self, key: &Key) -> Result<Option<usize>, DomainError>;
    }
}

#[cfg(test)]
mock! {
    pub LruRepository {}

    #[async_trait]
    impl LruRepository for LruRepository {
        async fn get_memory_usage(&self) -> Result<u64, DomainError>;
        async fn evict_lru(&self, count: usize) -> Result<usize, DomainError>;
    }
}
