use criterion::{Criterion, criterion_group, criterion_main};
use raptor::{
    application::use_cases::{
        basic::operations::{
            get_key_use_case::{GetKeyInput, GetKeyUseCase},
            set_key_use_case::{SetKeyInput, SetKeyUseCase},
        },
        counter::operations::incr_counter_use_case::IncrCounterUseCase,
    },
    domain::value_objects::{key::Key, value::Value},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn concurrent_hot_keys_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let _incr_use_case = Arc::new(IncrCounterUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_hot_keys");

    rt.block_on(async {
        for i in 0..10 {
            let key = Key::new(format!("hot_key_{}", i)).unwrap();
            let value = Value::String(format!("hot_value_{}", i).into_bytes());
            let input = SetKeyInput::new(key, value);
            set_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("concurrent_hot_keys", |b| {
        b.iter(|| {
            rt.block_on(async {
                let key_idx = fastrand::u32(0..10);
                let key = Key::new(format!("hot_key_{}", key_idx)).unwrap();
                let input = GetKeyInput::new(key);
                let _ = get_use_case.execute(input).await.unwrap();
            })
        })
    });

    group.finish();
}

criterion_group!(concurrent_hot_keys_benches, concurrent_hot_keys_benchmark);
criterion_main!(concurrent_hot_keys_benches);
