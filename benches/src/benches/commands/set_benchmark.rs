use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::basic::operations::set_key_use_case::{
    SetKeyInput, SetKeyUseCase,
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn set_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    c.bench_function("set_command", |b| {
        b.iter(|| {
            let key = Key::new(format!("key_{}", fastrand::u64(0..10000))).unwrap();
            let value = Value::String(b"my_value".to_vec());
            let set_use_case_clone = Arc::clone(&set_use_case);
            rt.block_on(set_use_case_clone.execute(SetKeyInput::new(key, value)))
        })
    });
}

criterion_group!(set_command_bench, set_benchmark);
criterion_main!(set_command_bench);
