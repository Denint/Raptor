#[cfg(all(test, not(miri)))]
mod common {
    include!("e2e/common.rs");
}

#[cfg(all(test, not(miri)))]
mod array_e2e {
    include!("e2e/usecases/array_e2e.rs");
}

#[cfg(all(test, not(miri)))]
mod audit_e2e {
    include!("e2e/usecases/audit_e2e.rs");
}

#[cfg(all(test, not(miri)))]
mod basic_flows {
    include!("e2e/usecases/basic_flows.rs");
}

#[cfg(all(test, not(miri)))]
mod basic_operations {
    include!("e2e/usecases/basic_operations.rs");
}

#[cfg(all(test, not(miri)))]
mod counter_operations {
    include!("e2e/usecases/counter_operations.rs");
}

#[cfg(all(test, not(miri)))]
mod hash_e2e {
    include!("e2e/usecases/hash_e2e.rs");
}

#[cfg(all(test, not(miri)))]
mod hash_extended {
    include!("e2e/usecases/hash_extended.rs");
}

#[cfg(all(test, not(miri)))]
mod list {
    include!("e2e/usecases/list.rs");
}

#[cfg(all(test, not(miri)))]
mod lru {
    include!("e2e/usecases/lru.rs");
}

#[cfg(all(test, not(miri)))]
mod lru_operations {
    include!("e2e/usecases/lru_operations.rs");
}

#[cfg(all(test, not(miri)))]
mod multi_operations {
    include!("e2e/usecases/multi_operations.rs");
}

#[cfg(all(test, not(miri)))]
mod set {
    include!("e2e/usecases/set.rs");
}

#[cfg(all(test, not(miri)))]
mod single_key_e2e {
    include!("e2e/usecases/single_key_e2e.rs");
}

#[cfg(all(test, not(miri)))]
mod sorted_set {
    include!("e2e/usecases/sorted_set.rs");
}

#[cfg(all(test, not(miri)))]
mod snapshot_persistence_e2e {
    include!("e2e/usecases/snapshot_persistence_e2e.rs");
}

#[cfg(all(test, not(miri)))]
mod ttl_e2e {
    include!("e2e/usecases/ttl_e2e.rs");
}
