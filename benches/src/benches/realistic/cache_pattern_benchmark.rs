use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::basic::operations::{
    get_key_use_case::{GetKeyInput, GetKeyUseCase},
    set_key_use_case::{SetKeyInput, SetKeyUseCase},
};
use raptor::application::use_cases::ttl::operations::{
    set_with_ttl_use_case::SetWithTtlUseCase, ttl_use_case::TtlUseCase,
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn cache_pattern_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let _set_with_ttl_use_case = Arc::new(SetWithTtlUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::TtlRepository>
    ));
    let _ttl_use_case = Arc::new(TtlUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::TtlRepository>
    ));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("cache_pattern");

    rt.block_on(async {
        for i in 0..1000 {
            let cache_key = Key::new(format!("cache_{}", i)).unwrap();
            let cache_value = Value::String(format!("cached_data_{}", i).into_bytes());
            let input = SetKeyInput::new(cache_key, cache_value);
            set_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("cache_pattern", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache_key = fastrand::u32(0..1000);

                let key = Key::new(format!("cache_{}", cache_key)).unwrap();
                let input = GetKeyInput::new(key.clone());
                let result = get_use_case.execute(input).await.unwrap();

                if result.is_none() {
                    let new_key = Key::new(format!("cache_{}", fastrand::u64(..))).unwrap();
                    let new_value = Value::String(b"new_cached_data".to_vec());
                    let input = SetKeyInput::new(new_key, new_value);
                    set_use_case.execute(input).await.unwrap();
                }
            })
        })
    });

    group.finish();
}

criterion_group!(cache_pattern_benches, cache_pattern_benchmark);
criterion_main!(cache_pattern_benches);
