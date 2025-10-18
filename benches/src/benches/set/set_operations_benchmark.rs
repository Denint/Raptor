use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::set::operations::{
    sadd_use_case::{SAddInput, SAddUseCase},
    scard_use_case::{SCardInput, SCardUseCase},
    sismember_use_case::{SIsMemberInput, SIsMemberUseCase},
    smembers_use_case::{SMembersInput, SMembersUseCase},
    srem_use_case::{SRemInput, SRemUseCase},
};
use raptor::domain::value_objects::key::Key;
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn set_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let sadd_use_case = Arc::new(SAddUseCase::new(Arc::clone(&storage)));
    let srem_use_case = Arc::new(SRemUseCase::new(Arc::clone(&storage)));
    let smembers_use_case = Arc::new(SMembersUseCase::new(Arc::clone(&storage)));
    let sismember_use_case = Arc::new(SIsMemberUseCase::new(Arc::clone(&storage)));
    let scard_use_case = Arc::new(SCardUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("set_operations");

    rt.block_on(async {
        for i in 0..100 {
            let key = Key::new(format!("set_test_key_{}", i)).unwrap();
            let mut members = Vec::new();
            for j in 0..50 {
                members.push(format!("member_{}_{}", i, j).into_bytes());
            }
            let input = SAddInput::new(key, members);
            sadd_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("sadd", |b| {
        b.iter(|| {
            let key = Key::new(format!("set_key_{}", fastrand::u64(0..1000))).unwrap();
            let members = vec![
                format!("member_{}", fastrand::u64(..)).into_bytes(),
                format!("member_{}", fastrand::u64(..)).into_bytes(),
                format!("member_{}", fastrand::u64(..)).into_bytes(),
            ];
            let input = SAddInput::new(key, members);
            let sadd_clone = Arc::clone(&sadd_use_case);
            rt.block_on(sadd_clone.execute(input))
        })
    });

    group.bench_function("sismember", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let member_idx = fastrand::u32(0..50);
            let key = Key::new(format!("set_test_key_{}", key_idx)).unwrap();
            let member = format!("member_{}_{}", key_idx, member_idx).into_bytes();
            let input = SIsMemberInput::new(key, member);
            let sismember_clone = Arc::clone(&sismember_use_case);
            rt.block_on(sismember_clone.execute(input))
        })
    });

    group.bench_function("scard", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("set_test_key_{}", key_idx)).unwrap();
            let input = SCardInput { key };
            let scard_clone = Arc::clone(&scard_use_case);
            rt.block_on(scard_clone.execute(input))
        })
    });

    group.bench_function("smembers", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("set_test_key_{}", key_idx)).unwrap();
            let input = SMembersInput { key };
            let smembers_clone = Arc::clone(&smembers_use_case);
            rt.block_on(smembers_clone.execute(input))
        })
    });

    rt.block_on(async {
        for i in 0..50 {
            let key = Key::new(format!("set_rem_key_{}", i)).unwrap();
            let members = vec![
                format!("rem_member_{}_0", i).into_bytes(),
                format!("rem_member_{}_1", i).into_bytes(),
                format!("rem_member_{}_2", i).into_bytes(),
                format!("rem_member_{}_3", i).into_bytes(),
                format!("rem_member_{}_4", i).into_bytes(),
            ];
            let input = SAddInput::new(key, members);
            sadd_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("srem", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..50);
            let key = Key::new(format!("set_rem_key_{}", key_idx)).unwrap();
            let members = vec![
                format!("rem_member_{}_0", key_idx).into_bytes(),
                format!("rem_member_{}_1", key_idx).into_bytes(),
            ];
            let input = SRemInput::new(key, members);
            let srem_clone = Arc::clone(&srem_use_case);
            rt.block_on(srem_clone.execute(input))
        })
    });

    group.bench_function("sismember_nonexistent", |b| {
        b.iter(|| {
            let key = Key::new(format!("nonexistent_set_{}", fastrand::u64(..))).unwrap();
            let member = b"nonexistent_member".to_vec();
            let input = SIsMemberInput::new(key, member);
            let sismember_clone = Arc::clone(&sismember_use_case);
            rt.block_on(sismember_clone.execute(input))
        })
    });

    group.finish();
}

criterion_group!(set_operations_benches, set_operations_benchmark);
criterion_main!(set_operations_benches);
