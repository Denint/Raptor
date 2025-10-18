pub mod storage;
pub mod lru;
pub mod smart_lru;
pub mod basic;
pub mod ttl;
pub mod counter;
pub mod multi;
pub mod hash;
pub mod list;
pub mod sorted_set;
pub mod set;
pub mod array;

pub use storage::InMemoryStorage;

