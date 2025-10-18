use crate::{
    application::use_cases::list::operations::{
        lpop_use_case::{LPopUseCase, LPopInput},
        lpush_use_case::{LPushUseCase, LPushInput},
        lrange_use_case::{LRangeUseCase, LRangeInput},
        rpop_use_case::{RPopUseCase, RPopInput},
        rpush_use_case::{RPushUseCase, RPushInput},
    },
    domain::{
        errors::DomainError,
        repositories::{BasicRepository, ListRepository},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_list_lpush_rpop_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("test_list");

    let values = vec![b"value1".to_vec(), b"value2".to_vec(), b"value3".to_vec()];
    let push_result = lpush_uc.execute(LPushInput::new(key.clone(), values)).await;
    assert_eq!(push_result, Ok(3));

    let pop1 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop1, Ok(Some(b"value1".to_vec())));

    let pop2 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop2, Ok(Some(b"value2".to_vec())));

    let pop3 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop3, Ok(Some(b"value3".to_vec())));

    let pop4 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop4, Ok(None));
}

#[tokio::test]
async fn test_list_multiple_lpush() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("test_list");

    let values1 = vec![b"first".to_vec()];
    let result1 = lpush_uc.execute(LPushInput::new(key.clone(), values1)).await;
    assert_eq!(result1, Ok(1));

    let values2 = vec![b"second".to_vec(), b"third".to_vec()];
    let result2 = lpush_uc.execute(LPushInput::new(key.clone(), values2)).await;
    assert_eq!(result2, Ok(3));

    let pop1 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop1, Ok(Some(b"first".to_vec())));

    let pop2 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop2, Ok(Some(b"second".to_vec())));
}

#[tokio::test]
async fn test_list_empty_list_operations() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("empty_list");

    let pop_result = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop_result, Ok(None));

    let values = vec![b"single_value".to_vec()];
    let push_result = lpush_uc.execute(LPushInput::new(key.clone(), values)).await;
    assert_eq!(push_result, Ok(1));

    let pop_result2 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop_result2, Ok(Some(b"single_value".to_vec())));

    let pop_result3 = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop_result3, Ok(None));
}

#[tokio::test]
async fn test_list_wrong_type_error() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("wrong_type_key");

    basic_repo.set(&key, Value::String(b"string_value".to_vec())).await.unwrap();

    let values = vec![b"list_value".to_vec()];
    let push_result = lpush_uc.execute(LPushInput::new(key.clone(), values)).await;
    assert_eq!(push_result, Err(DomainError::WrongType));

    let pop_result = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(pop_result, Err(DomainError::WrongType));
}

#[tokio::test]
async fn test_list_large_list_operations() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("large_list");

    for i in 0..100 {
        let value = format!("value_{}", i).into_bytes();
        lpush_uc.execute(LPushInput::new(key.clone(), vec![value])).await.unwrap();
    }

    for i in 0..10 {
        let expected = format!("value_{}", i).into_bytes();
        let pop_result = rpop_uc.execute(RPopInput::new(key.clone())).await;
        assert_eq!(pop_result, Ok(Some(expected)));
    }

    let pop_result = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert!(pop_result.is_ok());
    assert!(pop_result.unwrap().is_some());
}

#[tokio::test]
async fn test_list_lrange_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let rpush_uc = RPushUseCase::new(list_repo.clone());
    let lrange_uc = LRangeUseCase::new(list_repo.clone());

    let key = Key::from_static("lrange_test_list");
    rpush_uc.execute(RPushInput::new(key.clone(), vec![
        b"one".to_vec(),
        b"two".to_vec(),
        b"three".to_vec(),
        b"four".to_vec(),
        b"five".to_vec(),
    ])).await.unwrap();

    let result = lrange_uc.execute(LRangeInput::new(key.clone(), 0, 2)).await.unwrap();
    assert_eq!(result, vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()]);

    let result = lrange_uc.execute(LRangeInput::new(key.clone(), -3, -1)).await.unwrap();
    assert_eq!(result, vec![b"three".to_vec(), b"four".to_vec(), b"five".to_vec()]);

    let result = lrange_uc.execute(LRangeInput::new(key.clone(), 0, 100)).await.unwrap();
    assert_eq!(
        result,
        vec![
            b"one".to_vec(),
            b"two".to_vec(),
            b"three".to_vec(),
            b"four".to_vec(),
            b"five".to_vec(),
        ]
    );

    let result = lrange_uc.execute(LRangeInput::new(key.clone(), 3, 1)).await.unwrap();
    assert!(result.is_empty());

    let empty_key = Key::from_static("empty_lrange");
    let lpush_uc = LPushUseCase::new(list_repo.clone());
    lpush_uc.execute(LPushInput::new(empty_key.clone(), vec![])).await.unwrap();
    let result = lrange_uc.execute(LRangeInput::new(empty_key.clone(), 0, -1)).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_list_empty_values_vec_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let rpush_uc = RPushUseCase::new(list_repo.clone());
    let lrange_uc = LRangeUseCase::new(list_repo.clone());

    let key_lpush = Key::from_static("empty_lpush");
    lpush_uc.execute(LPushInput::new(key_lpush.clone(), vec![b"initial".to_vec()])).await.unwrap();
    let initial_len_lpush = lrange_uc.execute(LRangeInput::new(key_lpush.clone(), 0, -1)).await.unwrap().len();
    let result_lpush = lpush_uc.execute(LPushInput::new(key_lpush.clone(), vec![])).await.unwrap();
    assert_eq!(result_lpush, initial_len_lpush);
    assert_eq!(lrange_uc.execute(LRangeInput::new(key_lpush.clone(), 0, -1)).await.unwrap().len(), initial_len_lpush);

    let key_rpush = Key::from_static("empty_rpush");
    rpush_uc.execute(RPushInput::new(key_rpush.clone(), vec![b"initial".to_vec()])).await.unwrap();
    let initial_len_rpush = lrange_uc.execute(LRangeInput::new(key_rpush.clone(), 0, -1)).await.unwrap().len();
    let result_rpush = rpush_uc.execute(RPushInput::new(key_rpush.clone(), vec![])).await.unwrap();
    assert_eq!(result_rpush, initial_len_rpush);
    assert_eq!(lrange_uc.execute(LRangeInput::new(key_rpush.clone(), 0, -1)).await.unwrap().len(), initial_len_rpush);
}

#[tokio::test]
async fn test_list_lpop_rpop_empty_then_refill_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage;

    let rpush_uc = RPushUseCase::new(list_repo.clone());
    let lpop_uc = LPopUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());
    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let lrange_uc = LRangeUseCase::new(list_repo.clone());

    let key = Key::from_static("refill_list_integration");

    rpush_uc.execute(RPushInput::new(key.clone(), vec![b"A".to_vec(), b"B".to_vec()])).await.unwrap();
    assert_eq!(lrange_uc.execute(LRangeInput::new(key.clone(), 0, -1)).await.unwrap().len(), 2);

    assert_eq!(lpop_uc.execute(LPopInput::new(key.clone())).await.unwrap(), Some(b"A".to_vec()));
    assert_eq!(lpop_uc.execute(LPopInput::new(key.clone())).await.unwrap(), Some(b"B".to_vec()));
    assert_eq!(lpop_uc.execute(LPopInput::new(key.clone())).await.unwrap(), None);
    assert!(lrange_uc.execute(LRangeInput::new(key.clone(), 0, -1)).await.unwrap().is_empty());

    lpush_uc.execute(LPushInput::new(key.clone(), vec![b"C".to_vec(), b"D".to_vec()])).await.unwrap();
    assert_eq!(lrange_uc.execute(LRangeInput::new(key.clone(), 0, -1)).await.unwrap().len(), 2);
    assert_eq!(lrange_uc.execute(LRangeInput::new(key.clone(), 0, -1)).await.unwrap(), vec![b"D".to_vec(), b"C".to_vec()]);

    assert_eq!(rpop_uc.execute(RPopInput::new(key.clone())).await.unwrap(), Some(b"C".to_vec()));
    assert_eq!(rpop_uc.execute(RPopInput::new(key.clone())).await.unwrap(), Some(b"D".to_vec()));
    assert_eq!(rpop_uc.execute(RPopInput::new(key.clone())).await.unwrap(), None);
}

#[tokio::test]
async fn test_list_pop_after_type_conversion_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let list_repo: Arc<dyn ListRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let lpush_uc = LPushUseCase::new(list_repo.clone());
    let lpop_uc = LPopUseCase::new(list_repo.clone());
    let rpop_uc = RPopUseCase::new(list_repo.clone());

    let key = Key::from_static("converted_list_key");

    lpush_uc.execute(LPushInput::new(key.clone(), vec![b"initial".to_vec()])).await.unwrap();

    basic_repo.set(&key, Value::String(b"not_a_list".to_vec())).await.unwrap();

    let lpop_result = lpop_uc.execute(LPopInput::new(key.clone())).await;
    assert_eq!(lpop_result, Err(DomainError::WrongType));

    let rpop_result = rpop_uc.execute(RPopInput::new(key.clone())).await;
    assert_eq!(rpop_result, Err(DomainError::WrongType));
}
