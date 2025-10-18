use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::basic::{SetKeyInput, SetKeyUseCase};
use raptor::application::use_cases::multi_key::operations::mget_use_case::{
    MGetInput, MGetUseCase,
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn mget_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let mget_use_case = Arc::new(MGetUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::MultiRepository>
    ));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let keys: Vec<Key> = (0..100)
        .map(|i| Key::new(format!("key{}", i)).unwrap())
        .collect();

    rt.block_on(async {
        for key in &keys {
            let input = SetKeyInput::new(key.clone(), Value::String(b"some_value".to_vec()));
            set_use_case.execute(input).await.unwrap();
        }
    });

    c.bench_function("mget_command", |b| {
        b.iter(|| {
            let mget_use_case_clone = Arc::clone(&mget_use_case);
            let keys_clone = keys.clone();
            rt.block_on(mget_use_case_clone.execute(MGetInput::new(keys_clone)))
        })
    });
}

criterion_group!(mget_command_bench, mget_benchmark);
criterion_main!(mget_command_bench);
