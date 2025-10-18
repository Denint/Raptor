use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use futures::future::join_all;
use raptor::{
    application::use_cases::counter::operations::incr_counter_use_case::{
        IncrCounterInput, IncrCounterUseCase,
    },
    domain::value_objects::key::Key,
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn concurrent_counters_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let incr_use_case: Arc<IncrCounterUseCase<InMemoryStorage>> =
        Arc::new(IncrCounterUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_counters");

    let concurrency_levels = vec![1, 4, 8, 16, 32];

    for &concurrency in &concurrency_levels {
        group.bench_with_input(
            BenchmarkId::new("concurrent_counters", concurrency),
            &concurrency,
            |b, &concurrency| {
                let counter_key = Key::new("shared_counter".to_string()).unwrap();

                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::with_capacity(concurrency);

                        for _ in 0..concurrency {
                            let incr_use_case_clone = Arc::clone(&incr_use_case);
                            let counter_key = counter_key.clone();
                            let task = tokio::spawn(async move {
                                let _ = incr_use_case_clone
                                    .execute(IncrCounterInput::new(counter_key))
                                    .await
                                    .unwrap();
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

criterion_group!(concurrent_counters_benches, concurrent_counters_benchmark);
criterion_main!(concurrent_counters_benches);
