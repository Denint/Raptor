use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::basic::operations::{
    get_key_use_case::{GetKeyInput, GetKeyUseCase},
    set_key_use_case::{SetKeyInput, SetKeyUseCase},
};
use raptor::application::use_cases::counter::operations::incr_counter_use_case::IncrCounterUseCase;
use raptor::application::use_cases::multi_key::operations::{
    mget_use_case::{MGetInput, MGetUseCase},
    mset_use_case::{MSetInput, MSetUseCase},
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn allocator_performance_benchmark(c: &mut Criterion) {
    let allocator_name = "system";
    println!("🔧 Testing allocator: {}", allocator_name);

    let mut group = c.benchmark_group(format!("allocator_{}", allocator_name));

    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let _incr_use_case = Arc::new(IncrCounterUseCase::new(Arc::clone(&storage)));
    let mget_use_case = Arc::new(MGetUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::MultiRepository>
    ));
    let mset_use_case = Arc::new(MSetUseCase::new(
        Arc::clone(&storage) as Arc<dyn raptor::domain::repositories::MultiRepository>
    ));

    let rt = Runtime::new().unwrap();

    group.bench_function("set_single", |b| {
        b.iter(|| {
            let key = Key::new(format!("key_{}", fastrand::u64(..))).unwrap();
            let value = Value::String(format!("value_{}", fastrand::u64(..)).into_bytes());
            let set_use_case_clone = Arc::clone(&set_use_case);
            let input = SetKeyInput::new(key, value);
            rt.block_on(set_use_case_clone.execute(input)).unwrap();
        })
    });

    group.bench_function("get_single", |b| {
        rt.block_on(async {
            for i in 0..1000 {
                let key = Key::new(format!("key_{}", i)).unwrap();
                let value = Value::String(format!("value_{}", i).into_bytes());
                let input = SetKeyInput::new(key, value);
                set_use_case.execute(input).await.unwrap();
            }
        });

        b.iter(|| {
            let key = Key::new(format!("key_{}", fastrand::u64(0..1000))).unwrap();
            let get_use_case_clone = Arc::clone(&get_use_case);
            let input = GetKeyInput::new(key);
            let _ = rt.block_on(get_use_case_clone.execute(input));
        })
    });

    group.bench_function("mset_batch", |b| {
        b.iter(|| {
            let pairs: Vec<(Key, Value)> = (0..100)
                .map(|i| {
                    let key =
                        Key::new(format!("batch_key_{}", fastrand::u64(..) + i as u64)).unwrap();
                    let value = Value::String(format!("batch_value_{}", i).into_bytes());
                    (key, value)
                })
                .collect();
            let mset_use_case_clone = Arc::clone(&mset_use_case);
            let input = MSetInput::new(pairs);
            rt.block_on(mset_use_case_clone.execute(input)).unwrap();
        })
    });

    group.bench_function("mget_batch", |b| {
        rt.block_on(async {
            for i in 0..1000 {
                let key = Key::new(format!("mget_key_{}", i)).unwrap();
                let value = Value::String(format!("mget_value_{}", i).into_bytes());
                let input = SetKeyInput::new(key, value);
                set_use_case.execute(input).await.unwrap();
            }
        });

        b.iter(|| {
            let keys: Vec<Key> = (0..100)
                .map(|i| {
                    Key::new(format!("mget_key_{}", fastrand::u64(0..1000) + i as u64)).unwrap()
                })
                .collect();
            let mget_use_case_clone = Arc::clone(&mget_use_case);
            let input = MGetInput::new(keys);
            let _ = rt.block_on(mget_use_case_clone.execute(input));
        })
    });

    group.bench_function("string_allocations", |b| {
        b.iter(|| {
            let mut strings = Vec::with_capacity(1000);
            for i in 0..1000 {
                strings.push(format!(
                    "allocation_test_string_{}_{}",
                    i,
                    fastrand::u64(..)
                ));
            }
            strings
        })
    });

    group.bench_function("object_lifecycle", |b| {
        b.iter(|| {
            let mut objects = Vec::with_capacity(1000);
            for i in 0..1000 {
                let key = Key::new(format!("lifecycle_key_{}_{}", i, fastrand::u64(..))).unwrap();
                let value = Value::String(format!("lifecycle_value_{}", i).into_bytes());
                objects.push((key, value));
            }
            objects
        })
    });

    group.finish();
}

criterion_group!(
    name = allocator_benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(2));
    targets = allocator_performance_benchmark
);
criterion_main!(allocator_benches);
