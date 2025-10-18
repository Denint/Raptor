use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::hash::operations::{
    hget_use_case::{HGetInput, HGetUseCase},
    hset_use_case::{HSetInput, HSetUseCase},
};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn hget_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let hset_use_case = Arc::new(HSetUseCase::new(Arc::clone(&storage)));
    let hget_use_case = Arc::new(HGetUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        for i in 0..1000 {
            let key = Key::new(format!("hash_key_{}", i)).unwrap();
            for j in 0..10 {
                let field = format!("field_{}", j);
                let value = format!("value_{}_{}", i, j).into_bytes();
                let input = HSetInput::new(key.clone(), field, value);
                hset_use_case.execute(input).await.unwrap();
            }
        }
    });

    c.bench_function("hget_command", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..1000);
            let field_idx = fastrand::u32(0..10);
            let key = Key::new(format!("hash_key_{}", key_idx)).unwrap();
            let field = format!("field_{}", field_idx);
            let input = HGetInput::new(key, field);
            let hget_use_case_clone = Arc::clone(&hget_use_case);
            rt.block_on(hget_use_case_clone.execute(input))
        })
    });
}

criterion_group!(hget_command_bench, hget_benchmark);
criterion_main!(hget_command_bench);
