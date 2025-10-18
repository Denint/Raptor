use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::multi_key::operations::mset_use_case::{
    MSetInput, MSetUseCase,
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn mset_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let mset_use_case = Arc::new(MSetUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::MultiRepository>
    ));
    let rt = Runtime::new().unwrap();

    let pairs: Vec<(Key, Value)> = (0..100)
        .map(|i| {
            (
                Key::new(format!("key{}", i)).unwrap(),
                Value::String(format!("value{}", i).into_bytes()),
            )
        })
        .collect();

    c.bench_function("mset_command", |b| {
        b.iter(|| {
            let mset_use_case_clone = Arc::clone(&mset_use_case);
            let pairs_clone = pairs.clone();
            rt.block_on(mset_use_case_clone.execute(MSetInput::new(pairs_clone)))
        })
    });
}

criterion_group!(mset_command_bench, mset_benchmark);
criterion_main!(mset_command_bench);
