use crate::{
    application::use_cases::counter::operations::{
        decr_counter_use_case::{DecrCounterUseCase, DecrCounterInput},
        incr_counter_use_case::{IncrCounterUseCase, IncrCounterInput},
        reset_counter_use_case::{ResetCounterUseCase, ResetCounterInput},
    },
    domain::{
        errors::DomainError,
        repositories::{BasicRepository, CounterRepository},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_counter_use_cases_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let counter_repo: Arc<dyn CounterRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let incr_uc = IncrCounterUseCase::new(counter_repo.clone());
    let decr_uc = DecrCounterUseCase::new(counter_repo.clone());

    let key = Key::from_static("test_counter");


    let result = incr_uc.execute(IncrCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, 1);

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, Some(Value::Integer(1)));


    let result = incr_uc.execute(IncrCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, 2);

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, Some(Value::Integer(2)));


    let result = decr_uc.execute(DecrCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, 1);

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, Some(Value::Integer(1)));
}

#[tokio::test]
async fn test_counter_reset_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let counter_repo: Arc<dyn CounterRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let incr_uc = IncrCounterUseCase::new(counter_repo.clone());
    let reset_uc = ResetCounterUseCase::new(counter_repo.clone());

    let key = Key::from_static("test_reset");


    for _ in 0..50 {
        incr_uc.execute(IncrCounterInput::new(key.clone())).await.unwrap();
    }

    let result = reset_uc.execute(ResetCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, Some(50));

    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, Some(Value::Integer(0)));

    let nonexistent_key = Key::from_static("nonexistent_reset");
    let result = reset_uc.execute(ResetCounterInput::new(nonexistent_key.clone())).await.unwrap();
    assert_eq!(result, Some(0));
}


#[tokio::test]
async fn test_counter_wrong_type_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let counter_repo: Arc<dyn CounterRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let incr_uc = IncrCounterUseCase::new(counter_repo.clone());
    let reset_uc = ResetCounterUseCase::new(counter_repo.clone());

    let key = Key::from_static("wrong_type_counter");

    basic_repo.set(&key, Value::String(b"not_an_integer".to_vec())).await.unwrap();

    let incr_result = incr_uc.execute(IncrCounterInput::new(key.clone())).await;
    assert_eq!(incr_result, Err(DomainError::WrongType));

    let decr_uc = DecrCounterUseCase::new(counter_repo.clone());
    let decr_result = decr_uc.execute(DecrCounterInput::new(key.clone())).await;
    assert_eq!(decr_result, Err(DomainError::WrongType));

    let reset_result = reset_uc.execute(ResetCounterInput::new(key.clone())).await;
    assert_eq!(reset_result, Err(DomainError::WrongType));
}

#[tokio::test]
async fn test_counter_decr_negative_prevention_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());
    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let counter_repo: Arc<dyn CounterRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;

    let incr_uc = IncrCounterUseCase::new(counter_repo.clone());
    let decr_uc = DecrCounterUseCase::new(counter_repo.clone());

    let key = Key::from_static("test_decr_negative");


    let result = incr_uc.execute(IncrCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, 1);


    let result = decr_uc.execute(DecrCounterInput::new(key.clone())).await.unwrap();
    assert_eq!(result, 0);


    let result = decr_uc.execute(DecrCounterInput::new(key.clone())).await;
    assert!(matches!(result, Err(DomainError::NegativeCounterValue)));


    let retrieved = basic_repo.get(&key).await.unwrap();
    assert_eq!(retrieved, Some(Value::Integer(0)));
}
