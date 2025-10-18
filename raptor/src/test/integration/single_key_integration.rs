use crate::{
    application::use_cases::basic::operations::{
        delete_key_use_case::{DeleteKeyUseCase, DeleteKeyInput},
        get_key_use_case::{GetKeyUseCase, GetKeyInput},
        set_key_use_case::{SetKeyUseCase, SetKeyInput},
    },
    domain::{
        repositories::BasicRepository,
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_single_key_get_set_delete_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_uc = SetKeyUseCase::new(basic_repo.clone());
    let get_uc = GetKeyUseCase::new(basic_repo.clone());
    let delete_uc = DeleteKeyUseCase::new(basic_repo.clone());

    let key = Key::from_static("test_key");
    let value = Value::String(b"test_value".to_vec());

    let set_result = set_uc.execute(SetKeyInput::new(key.clone(), value.clone())).await;
    assert_eq!(set_result, Ok(true));

    let get_result = get_uc.execute(GetKeyInput::new(key.clone())).await;
    assert_eq!(get_result, Ok(Some(value.clone())));

    let delete_result = delete_uc.execute(DeleteKeyInput::new(key.clone())).await;
    assert_eq!(delete_result, Ok(Some(value.clone())));

    let get_after_delete = get_uc.execute(GetKeyInput::new(key.clone())).await;
    assert_eq!(get_after_delete, Ok(None));

    let nonexistent_key = Key::from_static("nonexistent_key");
    let get_nonexistent = get_uc.execute(GetKeyInput::new(nonexistent_key.clone())).await;
    assert_eq!(get_nonexistent, Ok(None));
}

#[tokio::test]
async fn test_single_key_set_overwrite_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let set_uc = SetKeyUseCase::new(basic_repo.clone());
    let get_uc = GetKeyUseCase::new(basic_repo.clone());

    let key = Key::from_static("overwrite_key");

    let res1 = set_uc.execute(SetKeyInput::new(key.clone(), Value::String(b"value1".to_vec()))).await;
    assert_eq!(res1, Ok(true));

    let res2 = set_uc.execute(SetKeyInput::new(key.clone(), Value::String(b"value2".to_vec()))).await;
    assert_eq!(res2, Ok(false));

    let retrieved = get_uc.execute(GetKeyInput::new(key.clone())).await.unwrap();
    assert_eq!(retrieved, Some(Value::String(b"value2".to_vec())));
}

#[tokio::test]
async fn test_single_key_delete_nonexistent_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let delete_uc = DeleteKeyUseCase::new(basic_repo.clone());

    let key = Key::from_static("nonexistent_delete_key");

    let delete_result = delete_uc.execute(DeleteKeyInput::new(key.clone())).await;
    assert_eq!(delete_result, Ok(None));
}
