use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::basic::{
    GetKeyInput, GetKeyUseCase, SetKeyInput, SetKeyUseCase,
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn get_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case: Arc<GetKeyUseCase<InMemoryStorage>> =
        Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case: Arc<SetKeyUseCase<InMemoryStorage>> =
        Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let key = Key::new("my_key".to_string()).unwrap();
        let value = Value::String(b"my_value".to_vec());
        let input = SetKeyInput::new(key, value);
        set_use_case.execute(input).await.unwrap();
    });

    c.bench_function("get_command", |b| {
        b.iter(|| {
            let key = Key::new("my_key".to_string()).unwrap();
            let get_use_case_clone = Arc::clone(&get_use_case);
            rt.block_on(get_use_case_clone.execute(GetKeyInput::new(key)))
        })
    });
}

criterion_group!(get_command_bench, get_benchmark);
criterion_main!(get_command_bench);
