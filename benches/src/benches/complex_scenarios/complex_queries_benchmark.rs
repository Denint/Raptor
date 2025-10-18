use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::{
    array::operations::{
        array_length_use_case::ArrayLengthUseCase, array_slice_use_case::ArraySliceUseCase,
    },
    hash::operations::{
        hget_use_case::HGetUseCase, hkeys_use_case::HKeysUseCase, hvals_use_case::HValsUseCase,
    },
    list::operations::lrange_use_case::LRangeUseCase,
    set::operations::{sismember_use_case::SIsMemberUseCase, smembers_use_case::SMembersUseCase},
    sorted_set::operations::{zrange_use_case::ZRangeUseCase, zscore_use_case::ZScoreUseCase},
};
use raptor::domain::repositories::BasicRepository;
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn complex_queries_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));

    let _hget_use_case = Arc::new(HGetUseCase::new(Arc::clone(&storage)));
    let hkeys_use_case = Arc::new(HKeysUseCase::new(Arc::clone(&storage)));
    let hvals_use_case = Arc::new(HValsUseCase::new(Arc::clone(&storage)));
    let lrange_use_case = Arc::new(LRangeUseCase::new(Arc::clone(&storage)));
    let sismember_use_case = Arc::new(SIsMemberUseCase::new(Arc::clone(&storage)));
    let smembers_use_case = Arc::new(SMembersUseCase::new(Arc::clone(&storage)));
    let zrange_use_case = Arc::new(ZRangeUseCase::new(Arc::clone(&storage)));
    let _zscore_use_case = Arc::new(ZScoreUseCase::new(Arc::clone(&storage)));
    let array_slice_use_case = Arc::new(ArraySliceUseCase::new(Arc::clone(&storage)));
    let array_length_use_case = Arc::new(ArrayLengthUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("complex_queries");

    rt.block_on(async {
        for i in 0..50 {
            let hash_key = Key::new(format!("complex_hash_{}", i)).unwrap();
            for j in 0..100 {
                let field = format!("field_{}", j);
                let value = format!("value_{}_{}_{}", i, j, fastrand::u64(..)).into_bytes();
                let hash_value = match storage.get(&hash_key).await.unwrap() {
                    Some(Value::Hash(mut existing)) => {
                        existing.insert(field, value);
                        Value::Hash(existing)
                    }
                    _ => {
                        let mut new_hash = std::collections::HashMap::new();
                        new_hash.insert(field, value);
                        Value::Hash(new_hash)
                    }
                };
                storage.set(&hash_key, hash_value).await.unwrap();
            }

            let list_key = Key::new(format!("complex_list_{}", i)).unwrap();
            let list_items: Vec<Vec<u8>> = (0..200)
                .map(|j| format!("list_item_{}_{}", i, j).into_bytes())
                .collect();
            let list_value = Value::List(list_items);
            storage.set(&list_key, list_value).await.unwrap();

            let set_key = Key::new(format!("complex_set_{}", i)).unwrap();
            let set_items: std::collections::HashSet<Vec<u8>> = (0..150)
                .map(|j| format!("set_item_{}_{}", i, j).into_bytes())
                .collect();
            let set_value = Value::Set(set_items);
            storage.set(&set_key, set_value).await.unwrap();

            let zset_key = Key::new(format!("complex_zset_{}", i)).unwrap();
            let zset_items: Vec<(f64, Vec<u8>)> = (0..100)
                .map(|j| {
                    let score = (i * 100 + j) as f64;
                    let member = format!("zset_member_{}_{}", i, j).into_bytes();
                    (score, member)
                })
                .collect();
            let zset_value = Value::SortedSet(zset_items);
            storage.set(&zset_key, zset_value).await.unwrap();

            let array_key = Key::new(format!("complex_array_{}", i)).unwrap();
            let array_items: Vec<Value> = (0..80)
                .map(|j| match j % 3 {
                    0 => Value::String(format!("array_str_{}_{}", i, j).into_bytes()),
                    1 => Value::Integer((i * 80 + j) as i64),
                    _ => Value::String(format!("array_data_{}_{}", i, j).into_bytes()),
                })
                .collect();
            let array_value = Value::Array(array_items);
            storage.set(&array_key, array_value).await.unwrap();
        }
    });

    group.bench_function("complex_hash_scan", |b| {
        b.iter(|| {
            rt.block_on(async {
                let hash_idx = fastrand::u32(0..50);
                let hash_key = Key::new(format!("complex_hash_{}", hash_idx)).unwrap();

                let _keys = hkeys_use_case.execute(raptor::application::use_cases::hash::operations::hkeys_use_case::HKeysInput {
                    key: hash_key.clone(),
                }).await.unwrap();

                let _vals = hvals_use_case.execute(raptor::application::use_cases::hash::operations::hvals_use_case::HValsInput {
                    key: hash_key,
                }).await.unwrap();
            })
        })
    });

    group.bench_function("complex_list_pagination", |b| {
        b.iter(|| {
            rt.block_on(async {
                let list_idx = fastrand::u32(0..50);
                let list_key = Key::new(format!("complex_list_{}", list_idx)).unwrap();

                let page_size = 20;
                let total_pages = 10;

                for page in 0..total_pages {
                    let start = page * page_size;
                    let stop = start + page_size - 1;
                    let _ = lrange_use_case.execute(raptor::application::use_cases::list::operations::lrange_use_case::LRangeInput {
                        key: list_key.clone(),
                        start: start as isize,
                        stop: stop as isize,
                    }).await.unwrap();
                }
            })
        })
    });

    group.bench_function("complex_set_operations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let set_idx = fastrand::u32(0..50);
                let set_key = Key::new(format!("complex_set_{}", set_idx)).unwrap();

                    let members = smembers_use_case.execute(raptor::application::use_cases::set::operations::smembers_use_case::SMembersInput {
                        key: set_key.clone(),
                    }).await.unwrap();

                for _ in 0..10 {
                    if let Some(random_member) = members.get(fastrand::usize(0..members.len().min(1))) {
                        let _ = sismember_use_case.execute(raptor::application::use_cases::set::operations::sismember_use_case::SIsMemberInput::new(
                            set_key.clone(),
                            random_member.clone(),
                        )).await.unwrap();
                    }
                }
            })
        })
    });

    group.bench_function("complex_sorted_set_queries", |b| {
        b.iter(|| {
            rt.block_on(async {
                let zset_idx = fastrand::u32(0..50);
                let zset_key = Key::new(format!("complex_zset_{}", zset_idx)).unwrap();

                let _top_scores = zrange_use_case.execute(raptor::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeInput {
                    key: zset_key.clone(),
                    start: -10,
                    stop: -1,
                }).await.unwrap();

                let _bottom_scores = zrange_use_case.execute(raptor::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeInput {
                    key: zset_key,
                    start: 0,
                    stop: 9,
                }).await.unwrap();
            })
        })
    });

    group.bench_function("complex_array_processing", |b| {
        b.iter(|| {
            rt.block_on(async {
                let array_idx = fastrand::u32(0..50);
                let array_key = Key::new(format!("complex_array_{}", array_idx)).unwrap();

                let length_result = array_length_use_case.execute(raptor::application::use_cases::array::operations::array_length_use_case::ArrayLengthInput {
                    key: array_key.clone(),
                }).await.unwrap();

                if let Some(length) = length_result {
                    let chunk_size = 10;
                    let chunks = length.div_ceil(chunk_size);

                    for chunk in 0..chunks {
                        let start = chunk * chunk_size;
                        let end = (start + chunk_size).min(length);

                        let _ = array_slice_use_case.execute(raptor::application::use_cases::array::operations::array_slice_use_case::ArraySliceInput::new(
                            array_key.clone(),
                            start,
                            Some(end),
                        )).await.unwrap();
                    }
                }
            })
        })
    });

    group.bench_function("cross_structure_query", |b| {
        b.iter(|| {
            rt.block_on(async {
                let idx = fastrand::u32(0..50);

                let hash_key = Key::new(format!("complex_hash_{}", idx)).unwrap();
                let list_key = Key::new(format!("complex_list_{}", idx)).unwrap();
                let set_key = Key::new(format!("complex_set_{}", idx)).unwrap();
                let zset_key = Key::new(format!("complex_zset_{}", idx)).unwrap();
                let array_key = Key::new(format!("complex_array_{}", idx)).unwrap();

                let hash_keys_future = hkeys_use_case.execute(raptor::application::use_cases::hash::operations::hkeys_use_case::HKeysInput {
                    key: hash_key,
                });
                let list_range_future = lrange_use_case.execute(raptor::application::use_cases::list::operations::lrange_use_case::LRangeInput {
                    key: list_key,
                    start: 0,
                    stop: 9,
                });
                let set_members_future = smembers_use_case.execute(raptor::application::use_cases::set::operations::smembers_use_case::SMembersInput {
                    key: set_key,
                });
                let zset_top_future = zrange_use_case.execute(raptor::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeInput {
                    key: zset_key,
                    start: -5,
                    stop: -1,
                });
                let array_slice_future = array_slice_use_case.execute(raptor::application::use_cases::array::operations::array_slice_use_case::ArraySliceInput::new(
                    array_key,
                    0,
                    Some(10),
                ));

                let (_hash_keys, _list_items, _set_members, _zset_top, _array_slice) =
                    tokio::join!(hash_keys_future, list_range_future, set_members_future, zset_top_future, array_slice_future);
            })
        })
    });

    group.finish();
}

criterion_group!(complex_queries_benches, complex_queries_benchmark);
criterion_main!(complex_queries_benches);
