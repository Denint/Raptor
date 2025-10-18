use crate::application::use_cases::ttl::operations::expire_use_case::{ExpireUseCase, ExpireInput};
use crate::application::use_cases::ttl::operations::persist_use_case::{PersistUseCase, PersistInput};
use crate::application::use_cases::ttl::operations::set_with_ttl_use_case::{SetWithTtlUseCase, SetWithTtlInput};
use crate::application::use_cases::ttl::operations::ttl_use_case::{TtlUseCase, TtlInput};
use crate::domain::{
    repositories::{BasicRepository, TtlRepository},
    value_objects::{key::Key, value::Value},
};
use crate::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_expire_command() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let expire_uc = ExpireUseCase::new(ttl_repo.clone());
    let ttl_uc = TtlUseCase::new(ttl_repo.clone());

    let key = Key::from_static("expire_test_key");

    let value = Value::String(b"expire_test_value".to_vec());
    basic_repo.set(&key, value).await.unwrap();

    let ttl_before = ttl_uc.execute(TtlInput::new(key.clone())).await.unwrap();
    assert_eq!(ttl_before, Some(-1));

    let expire_result = expire_uc.execute(ExpireInput::new(key.clone(), 5)).await.unwrap();
    assert!(expire_result);

    let ttl_after = ttl_uc.execute(TtlInput::new(key.clone())).await.unwrap();
    assert!(ttl_after.unwrap() <= 5 * 1000 && ttl_after.unwrap() > 0);

    let get_result = basic_repo.get(&key).await.unwrap();
    assert!(get_result.is_some());
}

#[tokio::test]
async fn test_ttl_set_with_ttl_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_with_ttl_uc = SetWithTtlUseCase::new(ttl_repo.clone());
    let ttl_uc = TtlUseCase::new(ttl_repo.clone());

    let key = Key::from_static("set_with_ttl_key");
    let value = Value::String(b"ttl_value".to_vec());

    set_with_ttl_uc.execute(SetWithTtlInput::new(key.clone(), value.clone(), 2)).await.unwrap();
    let ttl_result = ttl_uc.execute(TtlInput::new(key.clone())).await.unwrap();
    assert!(ttl_result.unwrap() > 0 && ttl_result.unwrap() <= 2 * 1000);
    tokio::time::sleep(Duration::from_millis(2100)).await;
    assert_eq!(basic_repo.get(&key).await.unwrap(), None);

    let key_ms = Key::from_static("set_with_ttl_ms_key");
    ttl_repo.set_with_ttl_ms(&key_ms, value.clone(), 2000).await.unwrap();
    let ttl_result_ms = ttl_uc.execute(TtlInput::new(key_ms.clone())).await.unwrap();
    assert!(ttl_result_ms.unwrap() > 0 && ttl_result_ms.unwrap() <= 2000);
    tokio::time::sleep(Duration::from_millis(2100)).await;
    assert_eq!(basic_repo.get(&key_ms).await.unwrap(), None);
}

#[tokio::test]
async fn test_ttl_persist_expired_key_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_with_ttl_uc = SetWithTtlUseCase::new(ttl_repo.clone());
    let persist_uc = PersistUseCase::new(ttl_repo.clone());

    let key = Key::from_static("expired_persist_key_integration");
    let value = Value::String(b"test_value".to_vec());

    set_with_ttl_uc.execute(SetWithTtlInput::new(key.clone(), value.clone(), 1)).await.unwrap();

    sleep(Duration::from_millis(1100)).await;

    let result = persist_uc.execute(PersistInput::new(key.clone())).await.unwrap();
    assert!(!result);

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_ttl_lazy_cleanup_on_operations() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_with_ttl_uc = SetWithTtlUseCase::new(ttl_repo.clone());
    let ttl_uc = TtlUseCase::new(ttl_repo.clone());

    let key = Key::from_static("lazy_cleanup_key");

    let value = Value::String(b"lazy_cleanup_value".to_vec());
    set_with_ttl_uc.execute(SetWithTtlInput::new(key.clone(), value, 1)).await.unwrap();

    sleep(Duration::from_millis(1100)).await;

    let ttl_result = ttl_uc.execute(TtlInput::new(key.clone())).await.unwrap();
    assert_eq!(ttl_result, Some(-2));

    let get_result = basic_repo.get(&key).await.unwrap();
    assert!(get_result.is_none());
}

#[tokio::test]
async fn test_ttl_expire_with_zero_ttl_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let expire_uc = ExpireUseCase::new(ttl_repo.clone());

    let key = Key::from_static("zero_ttl_key_integration");
    let value = Value::String(b"test_value".to_vec());

    basic_repo.set(&key, value.clone()).await.unwrap();

    let result = expire_uc.execute(ExpireInput::new(key.clone(), 0)).await.unwrap();
    assert!(result);

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_ttl_after_type_conversion_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let ttl_repo: Arc<dyn TtlRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_with_ttl_uc = SetWithTtlUseCase::new(ttl_repo.clone());
    let ttl_uc = TtlUseCase::new(ttl_repo.clone());

    let key = Key::from_static("converted_ttl_key");

    set_with_ttl_uc.execute(SetWithTtlInput::new(key.clone(), Value::String(b"initial".to_vec()), 10)).await.unwrap();

    basic_repo.set(&key, Value::String(b"not_a_ttl_key".to_vec())).await.unwrap();

    let result = ttl_uc.execute(TtlInput::new(key.clone())).await;
    assert_eq!(result, Ok(Some(-1)));
}
