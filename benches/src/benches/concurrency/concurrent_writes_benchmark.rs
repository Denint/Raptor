use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use futures::future::join_all;
use raptor::{
    application::use_cases::basic::operations::set_key_use_case::{SetKeyInput, SetKeyUseCase},
    domain::value_objects::{key::Key, value::Value},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn concurrent_writes_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let set_use_case: Arc<SetKeyUseCase<InMemoryStorage>> =
        Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_writes");

    let concurrency_levels = vec![1, 4, 8, 16, 32];

    for &concurrency in &concurrency_levels {
        group.bench_with_input(
            BenchmarkId::new("concurrent_writes", concurrency),
            &concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::with_capacity(concurrency);

                        for _ in 0..concurrency {
                            let set_use_case_clone = Arc::clone(&set_use_case);
                            let task = tokio::spawn(async move {
                                let key =
                                    Key::new(format!("write_key_{}", fastrand::u64(..))).unwrap();
                                let value = Value::String(
                                    format!("value_{}", fastrand::u64(..)).into_bytes(),
                                );
                                let input = SetKeyInput::new(key, value);
                                let _ = set_use_case_clone.execute(input).await.unwrap();
                            });
                            tasks.push(task);
                        }

                        join_all(tasks).await;
                    })
                })
            },
        );
    }

    group.finish();
}

criterion_group!(concurrent_writes_benches, concurrent_writes_benchmark);
criterion_main!(concurrent_writes_benches);
