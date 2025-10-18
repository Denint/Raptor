use crate::{
    application::use_cases::hash::operations::{
        hdel_use_case::{HDelUseCase, HDelInput},
        hexists_use_case::{HExistsUseCase, HExistsInput},
        hget_use_case::{HGetUseCase, HGetInput},
        hkeys_use_case::{HKeysUseCase, HKeysInput},
        hlen_use_case::{HLenUseCase, HLenInput},
        hset_use_case::{HSetUseCase, HSetInput},
        hvals_use_case::{HValsUseCase, HValsInput},
    },
    domain::{
        errors::DomainError,
        repositories::{BasicRepository, HashRepository},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_hash_hdel_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hdel_uc = HDelUseCase::new(hash_repo.clone());
    let hexists_uc = HExistsUseCase::new(hash_repo.clone());
    let hlen_uc = HLenUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_hdel");

    let result = hset_uc.execute(HSetInput::new(key.clone(), "field1".to_string(), b"value1".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = hset_uc.execute(HSetInput::new(key.clone(), "field2".to_string(), b"value2".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = hset_uc.execute(HSetInput::new(key.clone(), "field3".to_string(), b"value3".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = hdel_uc.execute(HDelInput::new(key.clone(), vec!["field2".to_string()])).await;
    assert_eq!(result, Ok(1));

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "field2".to_string())).await;
    assert_eq!(result, Ok(false));

    let result = hdel_uc.execute(HDelInput::new(key.clone(), vec!["field1".to_string(), "nonexistent".to_string()])).await;
    assert_eq!(result, Ok(1));

    let result = hdel_uc.execute(HDelInput::new(key.clone(), vec!["field3".to_string()])).await;
    assert_eq!(result, Ok(1));

    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));
}

#[tokio::test]
async fn test_hash_hexists_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hexists_uc = HExistsUseCase::new(hash_repo.clone());
    let hset_uc = HSetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_hexists");

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "nonexistent".to_string())).await;
    assert_eq!(result, Ok(false));

    let result = hset_uc.execute(HSetInput::new(key.clone(), "existing".to_string(), b"value".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "existing".to_string())).await;
    assert_eq!(result, Ok(true));

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "nonexistent".to_string())).await;
    assert_eq!(result, Ok(false));
}

#[tokio::test]
async fn test_hash_hkeys_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hkeys_uc = HKeysUseCase::new(hash_repo.clone());
    let hset_uc = HSetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_hkeys");

    let result = hkeys_uc.execute(HKeysInput::new(key.clone())).await;
    assert_eq!(result, Ok(vec![]));

    hset_uc.execute(HSetInput::new(key.clone(), "key1".to_string(), b"value1".to_vec())).await.unwrap();
    hset_uc.execute(HSetInput::new(key.clone(), "key2".to_string(), b"value2".to_vec())).await.unwrap();
    hset_uc.execute(HSetInput::new(key.clone(), "key3".to_string(), b"value3".to_vec())).await.unwrap();

    let result = hkeys_uc.execute(HKeysInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 3);
    assert!(result.as_ref().unwrap().contains(&"key1".to_string()));
    assert!(result.as_ref().unwrap().contains(&"key2".to_string()));
    assert!(result.as_ref().unwrap().contains(&"key3".to_string()));
}

#[tokio::test]
async fn test_hash_hvals_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hvals_uc = HValsUseCase::new(hash_repo.clone());
    let hset_uc = HSetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_hvals");

    let result = hvals_uc.execute(HValsInput::new(key.clone())).await;
    assert_eq!(result, Ok(vec![]));

    hset_uc.execute(HSetInput::new(key.clone(), "key1".to_string(), b"value1".to_vec())).await.unwrap();
    hset_uc.execute(HSetInput::new(key.clone(), "key2".to_string(), b"value2".to_vec())).await.unwrap();
    hset_uc.execute(HSetInput::new(key.clone(), "key3".to_string(), b"value3".to_vec())).await.unwrap();

    let result = hvals_uc.execute(HValsInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 3);
    assert!(result.as_ref().unwrap().contains(&b"value1".to_vec()));
    assert!(result.as_ref().unwrap().contains(&b"value2".to_vec()));
    assert!(result.as_ref().unwrap().contains(&b"value3".to_vec()));
}

#[tokio::test]
async fn test_hash_hlen_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hlen_uc = HLenUseCase::new(hash_repo.clone());
    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hdel_uc = HDelUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_hlen");

    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));

    hset_uc.execute(HSetInput::new(key.clone(), "field1".to_string(), b"value1".to_vec())).await.unwrap();
    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(1));

    hset_uc.execute(HSetInput::new(key.clone(), "field2".to_string(), b"value2".to_vec())).await.unwrap();
    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(2));

    hdel_uc.execute(HDelInput::new(key.clone(), vec!["field1".to_string()])).await.unwrap();
    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(1));
}

#[tokio::test]
async fn test_hash_complex_operations_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hlen_uc = HLenUseCase::new(hash_repo.clone());
    let hexists_uc = HExistsUseCase::new(hash_repo.clone());
    let hkeys_uc = HKeysUseCase::new(hash_repo.clone());
    let hvals_uc = HValsUseCase::new(hash_repo.clone());
    let hdel_uc = HDelUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_complex");

    let fields = vec![
        ("user_id", "12345"),
        ("username", "johndoe"),
        ("email", "john@example.com"),
        ("role", "admin"),
        ("last_login", "2024-01-15"),
    ];

    for (field, value) in &fields {
        hset_uc.execute(HSetInput::new(key.clone(), field.to_string(), value.as_bytes().to_vec())).await.unwrap();
    }

    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(5));

    for (field, _) in &fields {
        let result = hexists_uc.execute(HExistsInput::new(key.clone(), field.to_string())).await;
        assert_eq!(result, Ok(true));
    }

    let result = hkeys_uc.execute(HKeysInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 5);
    for (field, _) in &fields {
        assert!(result.as_ref().unwrap().contains(&field.to_string()));
    }

    let result = hvals_uc.execute(HValsInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 5);
    for (_, expected_value) in &fields {
        assert!(result.as_ref().unwrap().contains(&expected_value.as_bytes().to_vec()));
    }

    let result = hdel_uc.execute(HDelInput::new(key.clone(), vec!["role".to_string(), "last_login".to_string()])).await;
    assert_eq!(result, Ok(2));

    let result = hlen_uc.execute(HLenInput::new(key.clone())).await;
    assert_eq!(result, Ok(3));

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "role".to_string())).await;
    assert_eq!(result, Ok(false));

    let result = hexists_uc.execute(HExistsInput::new(key.clone(), "last_login".to_string())).await;
    assert_eq!(result, Ok(false));

    let remaining_fields = vec!["user_id", "username", "email"];
    for field in &remaining_fields {
        let result = hexists_uc.execute(HExistsInput::new(key.clone(), field.to_string())).await;
        assert_eq!(result, Ok(true));
    }
}

#[tokio::test]
async fn test_hash_hset_empty_field_and_value_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage;

    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hget_uc = HGetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_empty_field_value");

    let result = hset_uc.execute(HSetInput::new(key.clone(), "".to_string(), b"value".to_vec())).await;
    assert_eq!(result, Ok(true));
    let retrieved = hget_uc.execute(HGetInput::new(key.clone(), "".to_string())).await;
    assert_eq!(retrieved, Ok(Some(b"value".to_vec())));

    let result = hset_uc.execute(HSetInput::new(key.clone(), "field_empty_val".to_string(), Vec::new())).await;
    assert_eq!(result, Ok(true));
    let retrieved = hget_uc.execute(HGetInput::new(key.clone(), "field_empty_val".to_string())).await;
    assert_eq!(retrieved, Ok(Some(Vec::new())));

    let result = hset_uc.execute(HSetInput::new(key.clone(), "".to_string(), Vec::new())).await;
    assert_eq!(result, Ok(false));
    let retrieved = hget_uc.execute(HGetInput::new(key.clone(), "".to_string())).await;
    assert_eq!(retrieved, Ok(Some(Vec::new())));
}

#[tokio::test]
async fn test_hash_hget_after_type_conversion_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hget_uc = HGetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hash_type_conversion");

    hset_uc.execute(HSetInput::new(key.clone(), "field1".to_string(), b"value1".to_vec())).await.unwrap();

    basic_repo.set(&key, Value::String(b"not_a_hash".to_vec())).await.unwrap();

    let result = hget_uc.execute(HGetInput::new(key.clone(), "field1".to_string())).await;
    assert_eq!(result, Err(DomainError::WrongType));
}

#[tokio::test]
async fn test_hash_hset_updates_access_time_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let hash_repo: Arc<dyn HashRepository> = storage.clone();
    let hset_uc = HSetUseCase::new(hash_repo.clone());
    let hget_uc = HGetUseCase::new(hash_repo.clone());

    let key = Key::from_static("test_hset_access_time");

    hset_uc.execute(HSetInput::new(key.clone(), "field1".to_string(), b"value1".to_vec())).await.unwrap();

    let initial_access = storage.store.get(&key).unwrap().last_accessed;

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    hget_uc.execute(HGetInput::new(key.clone(), "field1".to_string())).await.unwrap();

    let updated_access = storage.store.get(&key).unwrap().last_accessed;
    assert!(updated_access >= initial_access);

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let intermediate_access = storage.store.get(&key).unwrap().last_accessed;

    hget_uc.execute(HGetInput::new(key.clone(), "field1".to_string())).await.unwrap();

    let final_access = storage.store.get(&key).unwrap().last_accessed;
    assert!(final_access >= intermediate_access);
}
