#[cfg(all(test, not(miri)))]
mod array_integration {
    include!("integration/array_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod audit_test {
    include!("integration/audit_test.rs");
}

#[cfg(all(test, not(miri)))]
mod counter_integration {
    include!("integration/counter_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod hash_integration {
    include!("integration/hash_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod list_integration {
    include!("integration/list_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod multi_key_integration {
    include!("integration/multi_key_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod set_integration {
    include!("integration/set_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod single_key_integration {
    include!("integration/single_key_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod snapshot_test {
    include!("integration/snapshot_test.rs");
}

#[cfg(all(test, not(miri)))]
mod sorted_set_integration {
    include!("integration/sorted_set_integration.rs");
}

#[cfg(all(test, not(miri)))]
mod ttl_integration {
    include!("integration/ttl_integration.rs");
}
