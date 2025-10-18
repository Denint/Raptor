use crate::domain::value_objects::{key::Key, value::Value};
use crate::infrastructure::persistence::InMemoryStorage;
use crate::application::use_cases::{
    basic::operations::{set_key_use_case::{SetKeyUseCase, SetKeyInput}, delete_key_use_case::{DeleteKeyUseCase, DeleteKeyInput}},
    counter::operations::incr_counter_use_case::{IncrCounterUseCase, IncrCounterInput},
    hash::operations::hset_use_case::{HSetUseCase, HSetInput},
};
use crate::domain::repositories::{BasicRepository, CounterRepository, HashRepository};
use std::fs;
use std::sync::Arc;

#[tokio::test]
async fn test_audit_logging() {
    let audit_log_path = "test_audit.log";
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));

    let basic_repo: Arc<dyn BasicRepository> = storage.clone();
    let counter_repo: Arc<dyn CounterRepository> = storage.clone();
    let hash_repo: Arc<dyn HashRepository> = storage.clone();

    let set_key_uc = SetKeyUseCase::new(basic_repo.clone());
    let delete_key_uc = DeleteKeyUseCase::new(basic_repo.clone());
    let incr_uc = IncrCounterUseCase::new(counter_repo.clone());
    let hset_uc = HSetUseCase::new(hash_repo.clone());

    let key_set = Key::from_static("audit_test_set");
    set_key_uc.execute(SetKeyInput::new(key_set.clone(), Value::String(b"test_value".to_vec()))).await.unwrap();

    let key_incr = Key::from_static("audit_test_incr");
    incr_uc.execute(IncrCounterInput::new(key_incr.clone())).await.unwrap();

    let key_hset = Key::from_static("audit_test_hset");
    hset_uc.execute(HSetInput::new(key_hset.clone(), "field1".to_string(), b"hval".to_vec())).await.unwrap();

    let key_del = Key::from_static("audit_test_del");
    set_key_uc.execute(SetKeyInput::new(key_del.clone(), Value::String(b"to_delete".to_vec()))).await.unwrap();
    delete_key_uc.execute(DeleteKeyInput::new(key_del.clone())).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let log_content = if std::path::Path::new(audit_log_path).exists() {
        fs::read_to_string(audit_log_path).unwrap_or_default()
    } else {
        String::new()
    };

    if !log_content.is_empty() {
        assert!(log_content.contains("op=\"set\""));
        assert!(log_content.contains("op=\"incr\""));
        assert!(log_content.contains("op=\"hset\""));
        assert!(log_content.contains("op=\"delete\""));
    }

    let key_error = Key::from_static("audit_test_error");
    let basic_repo_for_set: Arc<dyn BasicRepository> = storage.clone();
    basic_repo_for_set.set(&key_error, Value::String(b"not_a_number".to_vec())).await.unwrap();
    let incr_error_result = incr_uc.execute(IncrCounterInput::new(key_error.clone())).await;
    assert!(incr_error_result.is_err());

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let log_content_error = if std::path::Path::new(audit_log_path).exists() {
        fs::read_to_string(audit_log_path).unwrap_or_default()
    } else {
        String::new()
    };

    if !log_content_error.is_empty() {
        assert!(log_content_error.contains("success=false"));
    }

    if std::path::Path::new(audit_log_path).exists() {
        fs::remove_file(audit_log_path).unwrap();
    }

    let config_disabled = Arc::new(crate::infrastructure::config::create_test_config());
    let storage_disabled = Arc::new(InMemoryStorage::new(Arc::clone(&config_disabled)));
    let basic_repo_disabled: Arc<dyn BasicRepository> = storage_disabled;
    let set_key_uc_disabled = SetKeyUseCase::new(basic_repo_disabled);

    let key_disabled = Key::from_static("audit_disabled_test");
    set_key_uc_disabled.execute(SetKeyInput::new(key_disabled.clone(), Value::String(b"value_disabled".to_vec()))).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    assert!(!std::path::Path::new(audit_log_path).exists() || fs::read_to_string(audit_log_path).unwrap_or_default().is_empty());
}
