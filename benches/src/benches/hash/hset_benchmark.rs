use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::hash::operations::hset_use_case::{HSetInput, HSetUseCase};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn hset_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let hset_use_case = Arc::new(HSetUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    c.bench_function("hset_command", |b| {
        b.iter(|| {
            let key = Key::new(format!("hash_key_{}", fastrand::u64(0..10000))).unwrap();
            let field = format!("field_{}", fastrand::u64(..));
            let value = format!("value_{}", fastrand::u64(..)).into_bytes();
            let input = HSetInput::new(key, field, value);
            let hset_use_case_clone = Arc::clone(&hset_use_case);
            rt.block_on(hset_use_case_clone.execute(input))
        })
    });
}

criterion_group!(hset_command_bench, hset_benchmark);
criterion_main!(hset_command_bench);
