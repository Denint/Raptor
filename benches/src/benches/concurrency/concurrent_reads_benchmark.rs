use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use futures::future::join_all;
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

fn concurrent_reads_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case: Arc<GetKeyUseCase<InMemoryStorage>> =
        Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case: Arc<SetKeyUseCase<InMemoryStorage>> =
        Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_reads");

    let concurrency_levels = vec![1, 4, 8, 16, 32];

    for &concurrency in &concurrency_levels {
        group.bench_with_input(
            BenchmarkId::new("concurrent_reads", concurrency),
            &concurrency,
            |b, &concurrency| {
                rt.block_on(async {
                    for i in 0..1000 {
                        let key = Key::new(format!("concurrency_key_{}", i)).unwrap();
                        let value = Value::String(format!("value_{}", i).into_bytes());
                        let input = SetKeyInput::new(key, value);
                        set_use_case.execute(input).await.unwrap();
                    }
                });

                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::with_capacity(concurrency);

                        for _ in 0..concurrency {
                            let get_use_case_clone = Arc::clone(&get_use_case);
                            let task = tokio::spawn(async move {
                                let key_idx = fastrand::u32(0..1000);
                                let key = Key::new(format!("concurrency_key_{}", key_idx)).unwrap();
                                let input = GetKeyInput::new(key);
                                let _ = get_use_case_clone.execute(input).await.unwrap();
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

criterion_group!(concurrent_reads_benches, concurrent_reads_benchmark);
criterion_main!(concurrent_reads_benches);
