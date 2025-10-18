use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::list::operations::{
    lpop_use_case::{LPopInput, LPopUseCase},
    lpush_use_case::{LPushInput, LPushUseCase},
    lrange_use_case::{LRangeInput, LRangeUseCase},
    rpop_use_case::{RPopInput, RPopUseCase},
    rpush_use_case::{RPushInput, RPushUseCase},
};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn list_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let lpush_use_case = Arc::new(LPushUseCase::new(Arc::clone(&storage)));
    let rpush_use_case = Arc::new(RPushUseCase::new(Arc::clone(&storage)));
    let lpop_use_case = Arc::new(LPopUseCase::new(Arc::clone(&storage)));
    let rpop_use_case = Arc::new(RPopUseCase::new(Arc::clone(&storage)));
    let lrange_use_case = Arc::new(LRangeUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("list_operations");

    rt.block_on(async {
        for i in 0..50 {
            let key = Key::new(format!("list_range_key_{}", i)).unwrap();
            let mut values = Vec::new();
            for j in 0..100 {
                values.push(format!("item_{}_{}", i, j).into_bytes());
            }
            let input = LPushInput::new(key, values);
            lpush_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("lpush", |b| {
        b.iter(|| {
            let key = Key::new(format!("list_key_{}", fastrand::u64(0..1000))).unwrap();
            let values = vec![
                format!("item_{}", fastrand::u64(..)).into_bytes(),
                format!("item_{}", fastrand::u64(..)).into_bytes(),
            ];
            let input = LPushInput::new(key, values);
            let lpush_clone = Arc::clone(&lpush_use_case);
            rt.block_on(lpush_clone.execute(input))
        })
    });

    group.bench_function("rpush", |b| {
        b.iter(|| {
            let key = Key::new(format!("list_key_{}", fastrand::u64(0..1000))).unwrap();
            let values = vec![
                format!("item_{}", fastrand::u64(..)).into_bytes(),
                format!("item_{}", fastrand::u64(..)).into_bytes(),
            ];
            let input = RPushInput::new(key, values);
            let rpush_clone = Arc::clone(&rpush_use_case);
            rt.block_on(rpush_clone.execute(input))
        })
    });

    rt.block_on(async {
        for i in 0..100 {
            let key = Key::new(format!("list_pop_key_{}", i)).unwrap();
            let values = vec![
                format!("pop_item_{}_0", i).into_bytes(),
                format!("pop_item_{}_1", i).into_bytes(),
                format!("pop_item_{}_2", i).into_bytes(),
                format!("pop_item_{}_3", i).into_bytes(),
                format!("pop_item_{}_4", i).into_bytes(),
            ];
            let input = LPushInput::new(key, values);
            lpush_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("lpop", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("list_pop_key_{}", key_idx)).unwrap();
            let input = LPopInput { key };
            let lpop_clone = Arc::clone(&lpop_use_case);
            rt.block_on(lpop_clone.execute(input))
        })
    });

    group.bench_function("rpop", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("list_pop_key_{}", key_idx)).unwrap();
            let input = RPopInput { key };
            let rpop_clone = Arc::clone(&rpop_use_case);
            rt.block_on(rpop_clone.execute(input))
        })
    });

    group.bench_function("lrange", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..50);
            let key = Key::new(format!("list_range_key_{}", key_idx)).unwrap();
            let start = fastrand::u32(0..50) as isize;
            let stop = (start + fastrand::u32(10..50) as isize).min(99);
            let input = LRangeInput { key, start, stop };
            let lrange_clone = Arc::clone(&lrange_use_case);
            rt.block_on(lrange_clone.execute(input))
        })
    });

    group.bench_function("lrange_full", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..50);
            let key = Key::new(format!("list_range_key_{}", key_idx)).unwrap();
            let input = LRangeInput {
                key,
                start: 0,
                stop: -1,
            };
            let lrange_clone = Arc::clone(&lrange_use_case);
            rt.block_on(lrange_clone.execute(input))
        })
    });

    group.finish();
}

criterion_group!(list_operations_benches, list_operations_benchmark);
criterion_main!(list_operations_benches);
