use criterion::{Criterion, criterion_group, criterion_main};
use raptor::application::use_cases::array::operations::{
    array_get_use_case::{ArrayGetInput, ArrayGetUseCase},
    array_length_use_case::{ArrayLengthInput, ArrayLengthUseCase},
    array_set_use_case::{ArraySetInput, ArraySetUseCase},
    array_slice_use_case::{ArraySliceInput, ArraySliceUseCase},
    array_update_use_case::{ArrayUpdateInput, ArrayUpdateUseCase},
};
use raptor::domain::value_objects::{key::Key, value::Value};
use raptor::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::runtime::Runtime;

fn array_operations_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let array_set_use_case = Arc::new(ArraySetUseCase::new(Arc::clone(&storage)));
    let array_get_use_case = Arc::new(ArrayGetUseCase::new(Arc::clone(&storage)));
    let array_slice_use_case = Arc::new(ArraySliceUseCase::new(Arc::clone(&storage)));
    let array_update_use_case = Arc::new(ArrayUpdateUseCase::new(Arc::clone(&storage)));
    let array_length_use_case = Arc::new(ArrayLengthUseCase::new(Arc::clone(&storage)));
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("array_operations");

    group.bench_function("array_set", |b| {
        b.iter(|| {
            let key = Key::new(format!("array_key_{}", fastrand::u64(0..1000))).unwrap();
            let values = vec![
                Value::String(b"string_value".to_vec()),
                Value::Integer(42),
                Value::String(b"another_string".to_vec()),
                Value::Integer(-100),
                Value::String(b"final_string".to_vec()),
            ];
            let input = ArraySetInput::new(key, values);
            let array_set_clone = Arc::clone(&array_set_use_case);
            rt.block_on(array_set_clone.execute(input))
        })
    });

    rt.block_on(async {
        for i in 0..100 {
            let key = Key::new(format!("array_test_key_{}", i)).unwrap();
            let mut values = Vec::new();
            for j in 0..50 {
                match j % 4 {
                    0 => values.push(Value::String(format!("str_{}_{}", i, j).into_bytes())),
                    1 => values.push(Value::Integer((i * 50 + j) as i64)),
                    2 => values.push(Value::String(format!("data_{}_{}", i, j).into_bytes())),
                    _ => values.push(Value::Integer(-(i * 50 + j) as i64)),
                }
            }
            let input = ArraySetInput::new(key, values);
            array_set_use_case.execute(input).await.unwrap();
        }
    });

    group.bench_function("array_get_single", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("array_test_key_{}", key_idx)).unwrap();
            let indices = vec![fastrand::u32(0..50) as usize];
            let input = ArrayGetInput::new(key, indices);
            let array_get_clone = Arc::clone(&array_get_use_case);
            rt.block_on(array_get_clone.execute(input))
        })
    });

    group.bench_function("array_get_multiple", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("array_test_key_{}", key_idx)).unwrap();
            let indices = vec![
                fastrand::u32(0..25) as usize,
                fastrand::u32(25..50) as usize,
                fastrand::u32(0..50) as usize,
            ];
            let input = ArrayGetInput::new(key, indices);
            let array_get_clone = Arc::clone(&array_get_use_case);
            rt.block_on(array_get_clone.execute(input))
        })
    });

    group.bench_function("array_slice", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("array_test_key_{}", key_idx)).unwrap();
            let start = fastrand::u32(0..25) as usize;
            let end = (start + fastrand::u32(5..25) as usize).min(50);
            let input = ArraySliceInput::new(key, start, Some(end));
            let array_slice_clone = Arc::clone(&array_slice_use_case);
            rt.block_on(array_slice_clone.execute(input))
        })
    });

    group.bench_function("array_length", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("array_test_key_{}", key_idx)).unwrap();
            let input = ArrayLengthInput { key };
            let array_length_clone = Arc::clone(&array_length_use_case);
            rt.block_on(array_length_clone.execute(input))
        })
    });

    group.bench_function("array_update_single", |b| {
        b.iter(|| {
            let key = Key::new(format!("array_update_key_{}", fastrand::u64(..))).unwrap();

            let values = vec![
                Value::String(b"old_value_0".to_vec()),
                Value::String(b"old_value_1".to_vec()),
                Value::String(b"old_value_2".to_vec()),
            ];
            let set_input = ArraySetInput::new(key.clone(), values);
            rt.block_on(array_set_use_case.execute(set_input)).unwrap();

            let updates = vec![(
                1,
                Value::String(format!("new_value_{}", fastrand::u64(..)).into_bytes()),
            )];
            let input = ArrayUpdateInput::new(key, updates);
            let array_update_clone = Arc::clone(&array_update_use_case);
            rt.block_on(array_update_clone.execute(input))
        })
    });

    group.bench_function("array_slice_full", |b| {
        b.iter(|| {
            let key_idx = fastrand::u32(0..100);
            let key = Key::new(format!("array_test_key_{}", key_idx)).unwrap();
            let input = ArraySliceInput::new(key, 0, Some(50));
            let array_slice_clone = Arc::clone(&array_slice_use_case);
            rt.block_on(array_slice_clone.execute(input))
        })
    });

    group.bench_function("array_update_multiple", |b| {
        b.iter(|| {
            let key = Key::new(format!("array_update_multi_key_{}", fastrand::u64(..))).unwrap();

            let values = vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
                Value::Integer(4),
                Value::Integer(5),
            ];
            let set_input = ArraySetInput::new(key.clone(), values);
            rt.block_on(array_set_use_case.execute(set_input)).unwrap();

            let updates = vec![
                (0, Value::Integer(10)),
                (2, Value::Integer(30)),
                (4, Value::Integer(50)),
            ];
            let input = ArrayUpdateInput::new(key, updates);
            let array_update_clone = Arc::clone(&array_update_use_case);
            rt.block_on(array_update_clone.execute(input))
        })
    });

    group.finish();
}

criterion_group!(array_operations_benches, array_operations_benchmark);
criterion_main!(array_operations_benches);
