use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use futures::future::join_all;
use raptor::application::use_cases::basic::operations::{
    get_key_use_case::{GetKeyInput, GetKeyUseCase},
    set_key_use_case::{SetKeyInput, SetKeyUseCase},
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn contention_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case: Arc<GetKeyUseCase<InMemoryStorage>> =
        Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case: Arc<SetKeyUseCase<InMemoryStorage>> =
        Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("contention");

    let contention_levels = vec![10, 50, 100];

    for &hot_keys in &contention_levels {
        group.bench_with_input(
            BenchmarkId::new("high_contention", hot_keys),
            &hot_keys,
            |b, &hot_keys| {
                rt.block_on(async {
                    for i in 0..hot_keys {
                        let key = Key::new(format!("hot_key_{}", i)).unwrap();
                        let value = Value::String(format!("initial_value_{}", i).into_bytes());
                        let input = SetKeyInput::new(key, value);
                        set_use_case.execute(input).await.unwrap();
                    }
                });

                b.iter(|| {
                    rt.block_on(async {
                        let mut tasks = Vec::new();

                        for _ in 0..50 {
                            let get_use_case_clone = Arc::clone(&get_use_case);
                            let set_use_case_clone = Arc::clone(&set_use_case);
                            let task = tokio::spawn(async move {
                                let key_idx = fastrand::u32(0..hot_keys);
                                let key = Key::new(format!("hot_key_{}", key_idx)).unwrap();

                                if fastrand::bool() {
                                    let input = GetKeyInput::new(key);
                                    let _ = get_use_case_clone.execute(input).await.unwrap();
                                } else {
                                    let value = Value::String(
                                        format!("updated_{}", fastrand::u64(..)).into_bytes(),
                                    );
                                    let input = SetKeyInput::new(key, value);
                                    let _ = set_use_case_clone.execute(input).await.unwrap();
                                }
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

criterion_group!(contention_benches, contention_benchmark);
criterion_main!(contention_benches);
