use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::hash::operations::{
    hdel_use_case::{HDelInput, HDelUseCase},
    hexists_use_case::{HExistsInput, HExistsUseCase},
    hget_use_case::{HGetInput, HGetUseCase},
    hkeys_use_case::{HKeysInput, HKeysUseCase},
    hlen_use_case::{HLenInput, HLenUseCase},
    hset_use_case::{HSetInput, HSetUseCase},
    hvals_use_case::{HValsInput, HValsUseCase},
};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn hash_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let hset_use_case = Arc::new(HSetUseCase::new(Arc::clone(&storage)));
    let hget_use_case = Arc::new(HGetUseCase::new(Arc::clone(&storage)));
    let hdel_use_case = Arc::new(HDelUseCase::new(Arc::clone(&storage)));
    let hkeys_use_case = Arc::new(HKeysUseCase::new(Arc::clone(&storage)));
    let hvals_use_case = Arc::new(HValsUseCase::new(Arc::clone(&storage)));
    let hlen_use_case = Arc::new(HLenUseCase::new(Arc::clone(&storage)));
    let hexists_use_case = Arc::new(HExistsUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("hash_operations");

    rt.block_on(async {
        for i in 0..100 {
            let key = Key::new(format!("hash_test_key_{}", i)).unwrap();
            for j in 0..20 {
                let field = format!("field_{}", j);
                let value = format!("value_{}_{}", i, j).into_bytes();
                let input = HSetInput::new(key.clone(), field, value);
                hset_use_case.execute(input).await.unwrap();
            }
        }
    });

    group.bench_function("hset", |b| {
        b.iter(|| {
            let key = Key::new(format!("hash_key_{}", fastrand::u64(0..1000))).unwrap();
            let field = format!("field_{}", fastrand::u64(..));
            let value = format!("value_{}", fastrand::u64(..)).into_bytes();
            let input = HSetInput::new(key, field, value);
            let hset_clone = Arc::clone(&hset_use_case);
            rt.block_on(hset_clone.execute(input))
        })
    });

    group.bench_function("hget", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let field_idx = fastrand::u32(0..20);
            let key = Key::new(format!("hash_test_key_{}", key_idx)).unwrap();
            let field = format!("field_{}", field_idx);
            let input = HGetInput::new(key, field);
            let hget_clone = Arc::clone(&hget_use_case);
            rt.block_on(hget_clone.execute(input))
        })
    });

    group.bench_function("hexists", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let field_idx = fastrand::u32(0..20);
            let key = Key::new(format!("hash_test_key_{}", key_idx)).unwrap();
            let field = format!("field_{}", field_idx);
            let input = HExistsInput::new(key, field);
            let hexists_clone = Arc::clone(&hexists_use_case);
            rt.block_on(hexists_clone.execute(input))
        })
    });

    group.bench_function("hlen", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("hash_test_key_{}", key_idx)).unwrap();
            let input = HLenInput { key };
            let hlen_clone = Arc::clone(&hlen_use_case);
            rt.block_on(hlen_clone.execute(input))
        })
    });

    group.bench_function("hkeys", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("hash_test_key_{}", key_idx)).unwrap();
            let input = HKeysInput { key };
            let hkeys_clone = Arc::clone(&hkeys_use_case);
            rt.block_on(hkeys_clone.execute(input))
        })
    });

    group.bench_function("hvals", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("hash_test_key_{}", key_idx)).unwrap();
            let input = HValsInput { key };
            let hvals_clone = Arc::clone(&hvals_use_case);
            rt.block_on(hvals_clone.execute(input))
        })
    });

    group.bench_function("hdel", |b| {
        b.iter(|| {
            let key = Key::new(format!("hash_del_key_{}", fastrand::u64(..))).unwrap();
            rt.block_on(async {
                for j in 0..5 {
                    let field = format!("del_field_{}", j);
                    let value = format!("del_value_{}", j).into_bytes();
                    let input = HSetInput::new(key.clone(), field, value);
                    hset_use_case.execute(input).await.unwrap();
                }
            });

            let fields = vec![
                format!("del_field_{}", fastrand::u32(0..5)),
                format!("del_field_{}", fastrand::u32(0..5)),
            ];
            let input = HDelInput::new(key, fields);
            let hdel_clone = Arc::clone(&hdel_use_case);
            rt.block_on(hdel_clone.execute(input))
        })
    });

    group.finish();
}

criterion_group!(hash_operations_benches, hash_operations_benchmark);
criterion_main!(hash_operations_benches);
