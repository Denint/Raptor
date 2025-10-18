use crate::{
    application::use_cases::array::operations::{
        array_append_use_case::{ArrayAppendUseCase, ArrayAppendInput},
        array_get_use_case::{ArrayGetUseCase, ArrayGetInput},
        array_length_use_case::{ArrayLengthUseCase, ArrayLengthInput},
        array_set_use_case::{ArraySetUseCase, ArraySetInput},
        array_slice_use_case::{ArraySliceUseCase, ArraySliceInput},
        array_update_use_case::{ArrayUpdateUseCase, ArrayUpdateInput},
    },
    domain::{
        errors::DomainError,
        repositories::{ArrayRepository, BasicRepository},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_array_use_cases_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());

    let key = Key::from_static("test_array");
    let values = vec![
        Value::String(b"value1".to_vec()),
        Value::Integer(42),
        Value::String(b"value3".to_vec()),
    ];

    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), values.clone())).await;
    assert_eq!(set_result, Ok(()));

    let indices = vec![0, 1, 2, 3];
    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), indices)).await;
    assert_eq!(
        get_result,
        Ok(vec![
            Some(values[0].clone()),
            Some(values[1].clone()),
            Some(values[2].clone()),
            None,
        ])
    );

    let new_values = vec![
        Value::String(b"new_val1".to_vec()),
        Value::String(b"new_val2".to_vec()),
    ];
    let set_result2 = array_set_uc.execute(ArraySetInput::new(key.clone(), new_values.clone())).await;
    assert_eq!(set_result2, Ok(()));

    let get_result2 = array_get_uc.execute(ArrayGetInput::new(key.clone(), vec![0, 1, 2])).await;
    assert_eq!(
        get_result2,
        Ok(vec![
            Some(new_values[0].clone()),
            Some(new_values[1].clone()),
            None,
        ])
    );
}

#[tokio::test]
async fn test_array_append_nonexistent_key_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_append_uc = ArrayAppendUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());

    let key = Key::from_static("nonexistent_array_append");

    let append_values = vec![Value::String(b"element".to_vec())];
    let append_result = array_append_uc.execute(ArrayAppendInput::new(key.clone(), append_values)).await;

    assert_eq!(
        append_result,
        Err(DomainError::WrongType)
    );

    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), vec![0])).await;
    assert_eq!(get_result, Ok(vec![None]));
}

#[tokio::test]
async fn test_array_slice_empty_array_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_slice_uc = ArraySliceUseCase::new(array_repo.clone());

    let key = Key::from_static("empty_array_slice");

    array_set_uc.execute(ArraySetInput::new(key.clone(), vec![])).await.unwrap();

    let slice_result = array_slice_uc.execute(ArraySliceInput::new(key.clone(), 0, Some(10))).await;
    assert_eq!(slice_result, Ok(Vec::new()));

    let nonexistent_key = Key::from_static("nonexistent_key_slice");
    let slice_result2 = array_slice_uc.execute(ArraySliceInput::new(nonexistent_key.clone(), 0, Some(10))).await;
    assert_eq!(slice_result2, Ok(Vec::new()));
}

#[tokio::test]
async fn test_array_update_empty_updates_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_update_uc = ArrayUpdateUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());

    let key = Key::from_static("empty_updates_array");
    let initial_values = vec![Value::Integer(1), Value::Integer(2)];
    array_set_uc.execute(ArraySetInput::new(key.clone(), initial_values.clone())).await.unwrap();

    let updates = vec![];
    let update_result = array_update_uc.execute(ArrayUpdateInput::new(key.clone(), updates)).await;
    assert_eq!(update_result, Ok(0));

    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), vec![0, 1])).await;
    assert_eq!(get_result, Ok(vec![Some(initial_values[0].clone()), Some(initial_values[1].clone())]));
}

#[tokio::test]
async fn test_array_append_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_append_uc = ArrayAppendUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());

    let key = Key::from_static("test_array_append");

    let initial_values = vec![Value::String(b"initial".to_vec())];
    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), initial_values)).await;
    assert_eq!(set_result, Ok(()));

    let append_values = vec![
        Value::Integer(100),
        Value::String(b"appended".to_vec()),
    ];
    let append_result = array_append_uc.execute(ArrayAppendInput::new(key.clone(), append_values)).await;
    assert_eq!(append_result, Ok(3));

    let indices = vec![0, 1, 2];
    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), indices)).await;
    assert_eq!(
        get_result,
        Ok(vec![
            Some(Value::String(b"initial".to_vec())),
            Some(Value::Integer(100)),
            Some(Value::String(b"appended".to_vec())),
        ])
    );
}

#[tokio::test]
async fn test_array_slice_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_slice_uc = ArraySliceUseCase::new(array_repo.clone());

    let key = Key::from_static("test_array_slice");
    let values = vec![
        Value::String(b"zero".to_vec()),
        Value::String(b"one".to_vec()),
        Value::String(b"two".to_vec()),
        Value::String(b"three".to_vec()),
        Value::String(b"four".to_vec()),
    ];

    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), values)).await;
    assert_eq!(set_result, Ok(()));

    let slice_result = array_slice_uc.execute(ArraySliceInput::new(key.clone(), 1, Some(4))).await;
    assert_eq!(
        slice_result,
        Ok(vec![
            Value::String(b"one".to_vec()),
            Value::String(b"two".to_vec()),
            Value::String(b"three".to_vec()),
        ])
    );

    let slice_result2 = array_slice_uc.execute(ArraySliceInput::new(key.clone(), 3, None)).await;
    assert_eq!(
        slice_result2,
        Ok(vec![
            Value::String(b"three".to_vec()),
            Value::String(b"four".to_vec()),
        ])
    );

    let slice_result3 = array_slice_uc.execute(ArraySliceInput::new(key.clone(), 10, Some(15))).await;
    assert_eq!(slice_result3, Ok(Vec::new()));
}

#[tokio::test]
async fn test_array_update_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_update_uc = ArrayUpdateUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());

    let key = Key::from_static("test_array_update");
    let values = vec![
        Value::String(b"original1".to_vec()),
        Value::String(b"original2".to_vec()),
        Value::String(b"original3".to_vec()),
    ];

    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), values)).await;
    assert_eq!(set_result, Ok(()));

    let updates = vec![
        (
            0,
            Value::String(b"updated1".to_vec()),
        ),
        (2, Value::Integer(999)),
        (
            5,
            Value::String(b"out_of_bounds".to_vec()),
        ),
    ];
    let update_result = array_update_uc.execute(ArrayUpdateInput::new(key.clone(), updates)).await;
    assert_eq!(update_result, Ok(2));

    let indices = vec![0, 1, 2];
    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), indices)).await;
    assert_eq!(
        get_result,
        Ok(vec![
            Some(Value::String(b"updated1".to_vec())),
            Some(Value::String(b"original2".to_vec())),
            Some(Value::Integer(999)),
        ])
    );
}

#[tokio::test]
async fn test_array_length_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_length_uc = ArrayLengthUseCase::new(array_repo.clone());
    let array_append_uc = ArrayAppendUseCase::new(array_repo.clone());

    let key = Key::from_static("test_array_length");

    let length_result = array_length_uc.execute(ArrayLengthInput::new(key.clone())).await;
    assert_eq!(length_result, Ok(None));

    let values = vec![
        Value::String(b"item1".to_vec()),
        Value::String(b"item2".to_vec()),
    ];
    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), values)).await;
    assert_eq!(set_result, Ok(()));

    let length_result2 = array_length_uc.execute(ArrayLengthInput::new(key.clone())).await;
    assert_eq!(length_result2, Ok(Some(2)));

    let append_values = vec![
        Value::Integer(42),
        Value::String(b"item4".to_vec()),
    ];
    let append_result = array_append_uc.execute(ArrayAppendInput::new(key.clone(), append_values)).await;
    assert_eq!(append_result, Ok(4));

    let length_result3 = array_length_uc.execute(ArrayLengthInput::new(key.clone())).await;
    assert_eq!(length_result3, Ok(Some(4)));
}

#[tokio::test]
async fn test_array_operations_on_wrong_type() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());
    let array_append_uc = ArrayAppendUseCase::new(array_repo.clone());
    let array_slice_uc = ArraySliceUseCase::new(array_repo.clone());
    let array_update_uc = ArrayUpdateUseCase::new(array_repo.clone());
    let array_length_uc = ArrayLengthUseCase::new(array_repo.clone());

    let key = Key::from_static("wrong_type_key");

    basic_repo.set(&key, Value::String(b"string_value".to_vec())).await.unwrap();

    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), vec![0])).await;
    assert_eq!(get_result, Err(DomainError::WrongType));

    let append_result = array_append_uc.execute(ArrayAppendInput::new(key.clone(), vec![Value::String(b"new".to_vec())])).await;
    assert_eq!(append_result, Err(DomainError::WrongType));

    let slice_result = array_slice_uc.execute(ArraySliceInput::new(key.clone(), 0, Some(1))).await;
    assert_eq!(slice_result, Err(DomainError::WrongType));

    let update_result = array_update_uc.execute(ArrayUpdateInput::new(key.clone(), vec![(0, Value::String(b"updated".to_vec()))])).await;
    assert_eq!(update_result, Err(DomainError::WrongType));

    let length_result = array_length_uc.execute(ArrayLengthInput::new(key.clone())).await;
    assert_eq!(length_result, Err(DomainError::WrongType));
}

#[tokio::test]
async fn test_array_complex_operations() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let array_repo: Arc<dyn ArrayRepository> = storage;

    let array_set_uc = ArraySetUseCase::new(array_repo.clone());
    let array_get_uc = ArrayGetUseCase::new(array_repo.clone());
    let array_update_uc = ArrayUpdateUseCase::new(array_repo.clone());
    let array_append_uc = ArrayAppendUseCase::new(array_repo.clone());
    let array_length_uc = ArrayLengthUseCase::new(array_repo.clone());

    let key = Key::from_static("complex_array");

    let values = vec![
        Value::String(b"string_value".to_vec()),
        Value::Integer(42),
        Value::Hash(std::collections::HashMap::from([
            ("field1".to_string(), b"hash_value1".to_vec()),
            ("field2".to_string(), b"hash_value2".to_vec()),
        ])),
        Value::List(vec![b"list_item1".to_vec(), b"list_item2".to_vec()]),
    ];

    let set_result = array_set_uc.execute(ArraySetInput::new(key.clone(), values.clone())).await;
    assert_eq!(set_result, Ok(()));

    let indices = vec![0, 1, 2, 3];
    let get_result = array_get_uc.execute(ArrayGetInput::new(key.clone(), indices)).await;
    assert_eq!(
        get_result,
        Ok(vec![
            Some(values[0].clone()),
            Some(values[1].clone()),
            Some(values[2].clone()),
            Some(values[3].clone()),
        ])
    );

    let updates = vec![(1, Value::Integer(100))];
    let update_result = array_update_uc.execute(ArrayUpdateInput::new(key.clone(), updates)).await;
    assert_eq!(update_result, Ok(1));

    let get_single = array_get_uc.execute(ArrayGetInput::new(key.clone(), vec![1])).await;
    assert_eq!(
        get_single,
        Ok(vec![Some(Value::Integer(100))])
    );

    let append_values = vec![
        Value::String(b"appended_string".to_vec()),
        Value::Integer(200),
    ];
    let append_result = array_append_uc.execute(ArrayAppendInput::new(key.clone(), append_values)).await;
    assert_eq!(append_result, Ok(6));

    let length_result = array_length_uc.execute(ArrayLengthInput::new(key.clone())).await;
    assert_eq!(length_result, Ok(Some(6)));
}
