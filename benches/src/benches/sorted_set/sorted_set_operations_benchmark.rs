use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::sorted_set::operations::{
    zadd_use_case::{ZAddInput, ZAddUseCase},
    zcard_use_case::{ZCardInput, ZCardUseCase},
    zrange_use_case::{ZRangeInput, ZRangeUseCase},
    zrem_use_case::{ZRemInput, ZRemUseCase},
    zscore_use_case::{ZScoreInput, ZScoreUseCase},
};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn sorted_set_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let zadd_use_case = Arc::new(ZAddUseCase::new(Arc::clone(&storage)));
    let zrem_use_case = Arc::new(ZRemUseCase::new(Arc::clone(&storage)));
    let zrange_use_case = Arc::new(ZRangeUseCase::new(Arc::clone(&storage)));
    let zscore_use_case = Arc::new(ZScoreUseCase::new(Arc::clone(&storage)));
    let zcard_use_case = Arc::new(ZCardUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("sorted_set_operations");

    rt.block_on(async {
        for i in 0..100 {
            let key = Key::new(format!("zset_test_key_{}", i)).unwrap();
            for j in 0..100 {
                let score = fastrand::f64() * 1000.0;
                let member = format!("member_{}_{}", i, j).into_bytes();
                let input = ZAddInput::new(key.clone(), score, member);
                zadd_use_case.execute(input).await.unwrap();
            }
        }
    });

    group.bench_function("zadd", |b| {
        b.iter(|| {
            let key = Key::new(format!("zset_key_{}", fastrand::u64(0..1000))).unwrap();
            let score = fastrand::f64() * 1000.0;
            let member = format!("member_{}", fastrand::u64(..)).into_bytes();
            let input = ZAddInput::new(key, score, member);
            let zadd_clone = Arc::clone(&zadd_use_case);
            rt.block_on(zadd_clone.execute(input))
        })
    });

    group.bench_function("zscore", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let member_idx = fastrand::u32(0..100);
            let key = Key::new(format!("zset_test_key_{}", key_idx)).unwrap();
            let member = format!("member_{}_{}", key_idx, member_idx).into_bytes();
            let input = ZScoreInput::new(key, member);
            let zscore_clone = Arc::clone(&zscore_use_case);
            rt.block_on(zscore_clone.execute(input))
        })
    });

    group.bench_function("zcard", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("zset_test_key_{}", key_idx)).unwrap();
            let input = ZCardInput { key };
            let zcard_clone = Arc::clone(&zcard_use_case);
            rt.block_on(zcard_clone.execute(input))
        })
    });

    group.bench_function("zrange", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("zset_test_key_{}", key_idx)).unwrap();
            let start = fastrand::u32(0..50) as isize;
            let stop = (start + fastrand::u32(10..50) as isize).min(99);
            let input = ZRangeInput { key, start, stop };
            let zrange_clone = Arc::clone(&zrange_use_case);
            rt.block_on(zrange_clone.execute(input))
        })
    });

    group.bench_function("zrange_full", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("zset_test_key_{}", key_idx)).unwrap();
            let input = ZRangeInput {
                key,
                start: 0,
                stop: -1,
            };
            let zrange_clone = Arc::clone(&zrange_use_case);
            rt.block_on(zrange_clone.execute(input))
        })
    });

    rt.block_on(async {
        for i in 0..50 {
            let key = Key::new(format!("zset_rem_key_{}", i)).unwrap();
            for j in 0..20 {
                let score = j as f64 * 10.0;
                let member = format!("rem_member_{}_{}", i, j).into_bytes();
                let input = ZAddInput::new(key.clone(), score, member);
                zadd_use_case.execute(input).await.unwrap();
            }
        }
    });

    group.bench_function("zrem", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..50);
            let key = Key::new(format!("zset_rem_key_{}", key_idx)).unwrap();
            let members = vec![
                format!("rem_member_{}_{}", key_idx, fastrand::u32(0..20)).into_bytes(),
                format!("rem_member_{}_{}", key_idx, fastrand::u32(0..20)).into_bytes(),
            ];
            let input = ZRemInput::new(key, members);
            let zrem_clone = Arc::clone(&zrem_use_case);
            rt.block_on(zrem_clone.execute(input))
        })
    });

    group.bench_function("zrange_top_scores", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("zset_test_key_{}", key_idx)).unwrap();
            let input = ZRangeInput {
                key,
                start: -10,
                stop: -1,
            };
            let zrange_clone = Arc::clone(&zrange_use_case);
            rt.block_on(zrange_clone.execute(input))
        })
    });

    group.finish();
}

criterion_group!(
    sorted_set_operations_benches,
    sorted_set_operations_benchmark
);
criterion_main!(sorted_set_operations_benches);
