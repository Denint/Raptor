use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::{
    array::operations::{array_get_use_case::ArrayGetUseCase, array_set_use_case::ArraySetUseCase},
    basic::operations::set_key_use_case::SetKeyUseCase,
    hash::operations::{hget_use_case::HGetUseCase, hset_use_case::HSetUseCase},
    list::operations::{lpush_use_case::LPushUseCase, lrange_use_case::LRangeUseCase},
    set::operations::{sadd_use_case::SAddUseCase, smembers_use_case::SMembersUseCase},
    sorted_set::operations::{zadd_use_case::ZAddUseCase, zrange_use_case::ZRangeUseCase},
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn mixed_workload_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));

    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let hset_use_case = Arc::new(HSetUseCase::new(Arc::clone(&storage)));
    let hget_use_case = Arc::new(HGetUseCase::new(Arc::clone(&storage)));
    let lpush_use_case = Arc::new(LPushUseCase::new(Arc::clone(&storage)));
    let lrange_use_case = Arc::new(LRangeUseCase::new(Arc::clone(&storage)));
    let sadd_use_case = Arc::new(SAddUseCase::new(Arc::clone(&storage)));
    let smembers_use_case = Arc::new(SMembersUseCase::new(Arc::clone(&storage)));
    let zadd_use_case = Arc::new(ZAddUseCase::new(Arc::clone(&storage)));
    let zrange_use_case = Arc::new(ZRangeUseCase::new(Arc::clone(&storage)));
    let array_set_use_case = Arc::new(ArraySetUseCase::new(Arc::clone(&storage)));
    let array_get_use_case = Arc::new(ArrayGetUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("mixed_workload");

    rt.block_on(async {
        for i in 0..100 {
            let user_key = Key::new(format!("user:{}", i)).unwrap();
            hset_use_case.execute(raptor::application::use_cases::hash::operations::hset_use_case::HSetInput::new(
                user_key.clone(),
                "name".to_string(),
                format!("User {}", i).into_bytes(),
            )).await.unwrap();
            hset_use_case.execute(raptor::application::use_cases::hash::operations::hset_use_case::HSetInput::new(
                user_key.clone(),
                "email".to_string(),
                format!("user{}@example.com", i).into_bytes(),
            )).await.unwrap();

            let posts_key = Key::new(format!("posts:{}", i)).unwrap();
            let posts = (0..10).map(|j| format!("Post {} by user {}", j, i).into_bytes()).collect();
            lpush_use_case.execute(raptor::application::use_cases::list::operations::lpush_use_case::LPushInput::new(
                posts_key, posts
            )).await.unwrap();

            let followers_key = Key::new(format!("followers:{}", i)).unwrap();
            let followers = (0..20).map(|j| format!("user{}", (i + j) % 100).into_bytes()).collect();
            sadd_use_case.execute(raptor::application::use_cases::set::operations::sadd_use_case::SAddInput::new(
                followers_key, followers
            )).await.unwrap();

            let scores_key = Key::new(format!("scores:{}", i)).unwrap();
            for j in 0..5 {
                let score = (i as f64 * 10.0) + (j as f64 * 2.0);
                let member = format!("achievement_{}", j).into_bytes();
                zadd_use_case.execute(raptor::application::use_cases::sorted_set::operations::zadd_use_case::ZAddInput::new(
                    scores_key.clone(), score, member
                )).await.unwrap();
            }

            let prefs_key = Key::new(format!("preferences:{}", i)).unwrap();
            let preferences = vec![
                Value::String(b"dark_theme".to_vec()),
                Value::Integer(if i % 2 == 0 { 1 } else { 0 }),
                Value::String(format!("lang_{}", if i % 3 == 0 { "en" } else { "ru" }).into_bytes()),
            ];
            array_set_use_case.execute(raptor::application::use_cases::array::operations::array_set_use_case::ArraySetInput::new(
                prefs_key, preferences
            )).await.unwrap();
        }
    });

    group.bench_function("user_profile_lookup", |b| {
        b.iter(|| {
            rt.block_on(async {
                let user_id = fastrand::u32(0..100);

                let user_key = Key::new(format!("user:{}", user_id)).unwrap();
                let _ = hget_use_case.execute(raptor::application::use_cases::hash::operations::hget_use_case::HGetInput::new(
                    user_key, "name".to_string()
                )).await.unwrap();

                let posts_key = Key::new(format!("posts:{}", user_id)).unwrap();
                let _ = lrange_use_case.execute(raptor::application::use_cases::list::operations::lrange_use_case::LRangeInput {
                    key: posts_key,
                    start: 0,
                    stop: 4,
                }).await.unwrap();

                let followers_key = Key::new(format!("followers:{}", user_id)).unwrap();
                let _ = smembers_use_case.execute(raptor::application::use_cases::set::operations::smembers_use_case::SMembersInput {
                    key: followers_key,
                }).await.unwrap();
            })
        })
    });

    group.bench_function("social_feed_generation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let user_id = fastrand::u32(0..100);

                let prefs_key = Key::new(format!("preferences:{}", user_id)).unwrap();
                let _ = array_get_use_case.execute(raptor::application::use_cases::array::operations::array_get_use_case::ArrayGetInput::new(
                    prefs_key, vec![0, 2]
                )).await.unwrap();

                let scores_key = Key::new(format!("scores:{}", user_id)).unwrap();
                let _ = zrange_use_case.execute(raptor::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeInput {
                    key: scores_key,
                    start: -3,
                    stop: -1,
                }).await.unwrap();

                let followers_key = Key::new(format!("followers:{}", user_id)).unwrap();
                let _ = smembers_use_case.execute(raptor::application::use_cases::set::operations::smembers_use_case::SMembersInput {
                    key: followers_key,
                }).await.unwrap();
            })
        })
    });

    group.bench_function("analytics_query", |b| {
        b.iter(|| {
            rt.block_on(async {
                let user_id = fastrand::u32(0..100);

                let user_key = Key::new(format!("user:{}", user_id)).unwrap();
                let posts_key = Key::new(format!("posts:{}", user_id)).unwrap();
                let scores_key = Key::new(format!("scores:{}", user_id)).unwrap();

                let user_future = hget_use_case.execute(raptor::application::use_cases::hash::operations::hget_use_case::HGetInput::new(
                    user_key, "email".to_string()
                ));
                let posts_future = lrange_use_case.execute(raptor::application::use_cases::list::operations::lrange_use_case::LRangeInput {
                    key: posts_key,
                    start: 0,
                    stop: 2,
                });
                let scores_future = zrange_use_case.execute(raptor::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeInput {
                    key: scores_key,
                    start: 0,
                    stop: -1,
                });

                let (_user, _posts, _scores) = tokio::join!(user_future, posts_future, scores_future);
            })
        })
    });

    group.bench_function("data_migration_simulation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let user_id = fastrand::u64(..);

                let source_key = Key::new(format!("legacy_user_{}", user_id)).unwrap();
                let target_key = Key::new(format!("new_user_{}", user_id)).unwrap();

                let _old_data = set_use_case.execute(raptor::application::use_cases::basic::operations::set_key_use_case::SetKeyInput::new(
                    source_key.clone(),
                    Value::String(format!("legacy_data_{}", user_id).into_bytes())
                )).await.unwrap();

                hset_use_case.execute(raptor::application::use_cases::hash::operations::hset_use_case::HSetInput::new(
                    target_key.clone(),
                    "migrated_data".to_string(),
                    format!("new_data_{}", user_id).into_bytes(),
                )).await.unwrap();

                let migration_key = Key::new("migrated_users".to_string()).unwrap();
                let migration_data = vec![Value::String(format!("user_{}", user_id).into_bytes())];
                array_set_use_case.execute(raptor::application::use_cases::array::operations::array_set_use_case::ArraySetInput::new(
                    migration_key, migration_data
                )).await.unwrap();
            })
        })
    });

    group.finish();
}

criterion_group!(mixed_workload_benches, mixed_workload_benchmark);
criterion_main!(mixed_workload_benches);
