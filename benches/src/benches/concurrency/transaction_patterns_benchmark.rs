use criterion::{Criterion, criterion_group, criterion_main};
use raptor::{
    application::use_cases::{
        basic::operations::{
            get_key_use_case::{GetKeyInput, GetKeyUseCase},
            set_key_use_case::{SetKeyInput, SetKeyUseCase},
        },
        counter::operations::incr_counter_use_case::IncrCounterUseCase,
        multi_key::operations::mset_use_case::{MSetInput, MSetUseCase},
    },
    domain::value_objects::{key::Key, value::Value},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn transaction_patterns_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let _incr_use_case = Arc::new(IncrCounterUseCase::new(Arc::clone(&storage)));
    let mset_use_case = Arc::new(MSetUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::MultiRepository>
    ));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("transaction_patterns");

    group.bench_function("read_modify_write", |b| {
        b.iter(|| {
            rt.block_on(async {
                let key = Key::new(format!("rmw_{}", fastrand::u64(..))).unwrap();

                let get_input = GetKeyInput::new(key.clone());
                let current = match get_use_case.execute(get_input).await.unwrap() {
                    Some(Value::Integer(val)) => val,
                    _ => 0,
                };

                let new_value = Value::Integer(current + 1);
                let set_input = SetKeyInput::new(key, new_value);
                set_use_case.execute(set_input).await.unwrap();
            })
        })
    });

    group.bench_function("conditional_update", |b| {
        rt.block_on(async {
            for i in 0..100 {
                let key = Key::new(format!("cond_{}", i)).unwrap();
                let value = Value::Integer(0);
                let input = SetKeyInput::new(key, value);
                set_use_case.execute(input).await.unwrap();
            }
        });

        b.iter(|| {
            rt.block_on(async {
                let key_idx = fastrand::u32(0..100);
                let key = Key::new(format!("cond_{}", key_idx)).unwrap();

                let get_input = GetKeyInput::new(key.clone());
                if let Some(Value::Integer(current)) =
                    get_use_case.execute(get_input).await.unwrap()
                    && current % 2 == 0
                {
                    let new_value = Value::Integer(current + 1);
                    let set_input = SetKeyInput::new(key, new_value);
                    set_use_case.execute(set_input).await.unwrap();
                }
            })
        })
    });

    group.bench_function("batch_with_dependencies", |b| {
        b.iter(|| {
            rt.block_on(async {
                let batch_id = fastrand::u64(..);

                let mut pairs = Vec::new();
                for i in 0..10 {
                    let key = Key::new(format!("batch_{}_item_{}", batch_id, i)).unwrap();
                    let value = Value::String(format!("batch_data_{}", i).into_bytes());
                    pairs.push((key, value));
                }

                let index_key = Key::new(format!("batch_{}_index", batch_id)).unwrap();
                let index_value = Value::String(format!("items:0-{}", 10).into_bytes());
                pairs.push((index_key, index_value));

                let input = MSetInput::new(pairs);
                mset_use_case.execute(input).await.unwrap();
            })
        })
    });

    group.finish();
}

criterion_group!(transaction_patterns_benches, transaction_patterns_benchmark);
criterion_main!(transaction_patterns_benches);
