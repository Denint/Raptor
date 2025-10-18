use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use raptor::{
    application::use_cases::basic::operations::{
        get_key_use_case::{GetKeyInput, GetKeyUseCase},
        set_key_use_case::{SetKeyInput, SetKeyUseCase},
    },
    domain::value_objects::{key::Key, value::Value},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn large_value_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("large_value_operations");

    let data_sizes = vec![64];

    for &size in &data_sizes {
        group.bench_with_input(
            BenchmarkId::new("large_value_operations", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    rt.block_on(async {
                        let key = Key::new(format!("large_key_{}", fastrand::u64(..))).unwrap();
                        let value = Value::String(vec![b'A'; size]);

                        let set_input = SetKeyInput::new(key.clone(), value);
                        set_use_case.execute(set_input).await.unwrap();

                        let get_input = GetKeyInput::new(key);
                        let retrieved = get_use_case.execute(get_input).await.unwrap();
                        assert!(retrieved.is_some());
                    })
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    large_value_operations_benches,
    large_value_operations_benchmark
);
criterion_main!(large_value_operations_benches);
