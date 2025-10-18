use crate::{
    application::use_cases::set::operations::{
        sadd_use_case::{SAddUseCase, SAddInput},
        scard_use_case::{SCardUseCase, SCardInput},
        sismember_use_case::{SIsMemberUseCase, SIsMemberInput},
        smembers_use_case::{SMembersUseCase, SMembersInput},
        srem_use_case::{SRemUseCase, SRemInput},
    },
    domain::{
        errors::DomainError,
        repositories::SetRepository,
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_set_sadd_smembers_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let smembers_uc = SMembersUseCase::new(set_repo.clone());

    let key = Key::from_static("test_set");

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"member1".to_vec(), b"member2".to_vec()])).await;
    assert_eq!(result, Ok(2));

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"member2".to_vec(), b"member3".to_vec()])).await;
    assert_eq!(result, Ok(1));

    let result = smembers_uc.execute(SMembersInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 3);
    assert!(result.as_ref().unwrap().contains(&b"member1".to_vec()));
    assert!(result.as_ref().unwrap().contains(&b"member2".to_vec()));
    assert!(result.as_ref().unwrap().contains(&b"member3".to_vec()));
}

#[tokio::test]
async fn test_set_srem_sismember_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let sismember_uc = SIsMemberUseCase::new(set_repo.clone());
    let srem_uc = SRemUseCase::new(set_repo.clone());

    let key = Key::from_static("test_set_ops");

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"alice".to_vec(), b"bob".to_vec(), b"charlie".to_vec()])).await;
    assert_eq!(result, Ok(3));

    let result = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"alice".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"dave".to_vec())).await;
    assert_eq!(result, Ok(false));

    let result = srem_uc.execute(SRemInput::new(key.clone(), vec![b"alice".to_vec(), b"nonexistent".to_vec()])).await;
    assert_eq!(result, Ok(1));

    let result = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"alice".to_vec())).await;
    assert_eq!(result, Ok(false));

    let result = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"bob".to_vec())).await;
    assert_eq!(result, Ok(true));
}

#[tokio::test]
async fn test_set_scard_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let scard_uc = SCardUseCase::new(set_repo.clone());
    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let srem_uc = SRemUseCase::new(set_repo.clone());

    let key = Key::from_static("test_set_card");

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"one".to_vec(), b"two".to_vec()])).await;
    assert_eq!(result, Ok(2));

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(2));

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"three".to_vec()])).await;
    assert_eq!(result, Ok(1));

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(3));

    let result = srem_uc.execute(SRemInput::new(key.clone(), vec![b"one".to_vec(), b"two".to_vec()])).await;
    assert_eq!(result, Ok(2));

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(1));
}

#[tokio::test]
async fn test_set_empty_operations_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let smembers_uc = SMembersUseCase::new(set_repo.clone());
    let scard_uc = SCardUseCase::new(set_repo.clone());
    let sismember_uc = SIsMemberUseCase::new(set_repo.clone());
    let srem_uc = SRemUseCase::new(set_repo.clone());

    let key = Key::from_static("empty_set");

    let result = smembers_uc.execute(SMembersInput::new(key.clone())).await;
    assert_eq!(result, Ok(vec![]));

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));

    let result = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"nonexistent".to_vec())).await;
    assert_eq!(result, Ok(false));

    let result = srem_uc.execute(SRemInput::new(key.clone(), vec![b"nonexistent".to_vec()])).await;
    assert_eq!(result, Ok(0));
}

#[tokio::test]
async fn test_set_duplicates_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let scard_uc = SCardUseCase::new(set_repo.clone());
    let smembers_uc = SMembersUseCase::new(set_repo.clone());

    let key = Key::from_static("test_duplicates");

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![b"same".to_vec(), b"same".to_vec(), b"different".to_vec()])).await;
    assert_eq!(result, Ok(2));

    let result = scard_uc.execute(SCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(2));

    let result = smembers_uc.execute(SMembersInput::new(key.clone())).await;
    assert_eq!(result.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_set_sadd_empty_members_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let scard_uc = SCardUseCase::new(set_repo.clone());

    let key = Key::from_static("test_sadd_empty");

    sadd_uc.execute(SAddInput::new(key.clone(), vec![b"initial".to_vec()])).await.unwrap();
    let initial_card = scard_uc.execute(SCardInput::new(key.clone())).await.unwrap();

    let result = sadd_uc.execute(SAddInput::new(key.clone(), vec![])).await;
    assert_eq!(result, Ok(0));

    let current_card = scard_uc.execute(SCardInput::new(key.clone())).await.unwrap();
    assert_eq!(current_card, initial_card);
}

#[tokio::test]
async fn test_set_srem_empty_members_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let scard_uc = SCardUseCase::new(set_repo.clone());
    let srem_uc = SRemUseCase::new(set_repo.clone());

    let key = Key::from_static("test_srem_empty");

    sadd_uc.execute(SAddInput::new(key.clone(), vec![b"initial".to_vec()])).await.unwrap();
    let initial_card = scard_uc.execute(SCardInput::new(key.clone())).await.unwrap();

    let result = srem_uc.execute(SRemInput::new(key.clone(), vec![])).await;
    assert_eq!(result, Ok(0));

    let current_card = scard_uc.execute(SCardInput::new(key.clone())).await.unwrap();
    assert_eq!(current_card, initial_card);
}

#[tokio::test]
async fn test_set_sismember_empty_member_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let sismember_uc = SIsMemberUseCase::new(set_repo.clone());

    let key = Key::from_static("test_sismember_empty");

    sadd_uc.execute(SAddInput::new(key.clone(), vec![b"nonempty".to_vec(), Vec::new()])).await.unwrap();

    let result_empty = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"".to_vec())).await;
    assert_eq!(result_empty, Ok(true));

    let result_nonempty = sismember_uc.execute(SIsMemberInput::new(key.clone(), b"nonempty".to_vec())).await;
    assert_eq!(result_nonempty, Ok(true));

    let nonexistent_key = Key::from_static("nonexistent_set");
    let result_nonexistent_empty = sismember_uc.execute(SIsMemberInput::new(nonexistent_key.clone(), b"".to_vec())).await;
    assert_eq!(result_nonexistent_empty, Ok(false));
}

#[tokio::test]
async fn test_set_smembers_after_type_conversion_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let set_repo: Arc<dyn SetRepository> = storage.clone();
    let basic_repo: Arc<dyn crate::domain::repositories::BasicRepository> = storage;

    let sadd_uc = SAddUseCase::new(set_repo.clone());
    let smembers_uc = SMembersUseCase::new(set_repo.clone());

    let key = Key::from_static("test_set_type_conversion");

    sadd_uc.execute(SAddInput::new(key.clone(), vec![b"member1".to_vec()])).await.unwrap();

    basic_repo.set(&key, Value::String(b"not_a_set".to_vec())).await.unwrap();

    let result = smembers_uc.execute(SMembersInput::new(key.clone())).await;
    assert_eq!(result, Err(DomainError::WrongType));
}
