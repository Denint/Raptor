use crate::{
    application::use_cases::sorted_set::operations::{
        zadd_use_case::{ZAddUseCase, ZAddInput},
        zcard_use_case::{ZCardUseCase, ZCardInput},
        zrange_use_case::{ZRangeUseCase, ZRangeInput},
        zrem_use_case::{ZRemUseCase, ZRemInput},
        zscore_use_case::{ZScoreUseCase, ZScoreInput},
    },
    domain::{
        errors::DomainError,
        repositories::{BasicRepository, SortedSetRepository},
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_sorted_set_zadd_zrange_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;

    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_sorted_set");

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 2.5, b"member2".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 1.0, b"member1".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 3.0, b"member3".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await;
    assert_eq!(
        result,
        Ok(vec![
            b"member1".to_vec(),
            b"member2".to_vec(),
            b"member3".to_vec()
        ])
    );

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 1, 2)).await;
    assert_eq!(result, Ok(vec![b"member2".to_vec(), b"member3".to_vec()]));
}

#[tokio::test]
async fn test_sorted_set_zadd_min_max_nan_scores_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_zset_min_max_nan");

    zadd_uc.execute(ZAddInput::new(key.clone(), f64::MAX, b"max_member".to_vec())).await.unwrap();
    zadd_uc.execute(ZAddInput::new(key.clone(), f64::MIN, b"min_member".to_vec())).await.unwrap();
    zadd_uc.execute(ZAddInput::new(key.clone(), 0.0, b"zero_member".to_vec())).await.unwrap();

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await.unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], b"min_member".to_vec());
    assert_eq!(result[1], b"zero_member".to_vec());
    assert_eq!(result[2], b"max_member".to_vec());

    zadd_uc.execute(ZAddInput::new(key.clone(), f64::NAN, b"nan_member".to_vec())).await.unwrap();
    let result_nan = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await.unwrap();
    assert_eq!(result_nan.len(), 4);
    assert!(result_nan.contains(&b"nan_member".to_vec()));
}

#[tokio::test]
async fn test_sorted_set_zrem_empty_members_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zcard_uc = ZCardUseCase::new(sorted_set_repo.clone());
    let zrem_uc = ZRemUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_zrem_empty");

    zadd_uc.execute(ZAddInput::new(key.clone(), 1.0, b"initial".to_vec())).await.unwrap();
    let initial_card = zcard_uc.execute(ZCardInput::new(key.clone())).await.unwrap();

    let result = zrem_uc.execute(ZRemInput::new(key.clone(), vec![])).await;
    assert_eq!(result, Ok(0));

    let current_card = zcard_uc.execute(ZCardInput::new(key.clone())).await.unwrap();
    assert_eq!(current_card, initial_card);
}

#[tokio::test]
async fn test_sorted_set_zrange_single_element_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_zrange_single");
    zadd_uc.execute(ZAddInput::new(key.clone(), 1.0, b"member1".to_vec())).await.unwrap();

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, 0)).await.unwrap();
    assert_eq!(result, vec![b"member1".to_vec()]);

    let result_neg = zrange_uc.execute(ZRangeInput::new(key.clone(), -1, -1)).await.unwrap();
    assert_eq!(result_neg, vec![b"member1".to_vec()]);
}

#[tokio::test]
async fn test_sorted_set_zrange_after_type_conversion_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage.clone();
    let basic_repo: Arc<dyn BasicRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_zrange_type_conversion");

    zadd_uc.execute(ZAddInput::new(key.clone(), 1.0, b"member1".to_vec())).await.unwrap();

    basic_repo.set(&key, Value::String(b"not_a_zset".to_vec())).await.unwrap();

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await;
    assert_eq!(result, Err(DomainError::WrongType));
}

#[tokio::test]
async fn test_sorted_set_zrem_zcard_zscore_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zcard_uc = ZCardUseCase::new(sorted_set_repo.clone());
    let zscore_uc = ZScoreUseCase::new(sorted_set_repo.clone());
    let zrem_uc = ZRemUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_sorted_set_ops");

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 1.5, b"member1".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zcard_uc.execute(ZCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(1));

    let result = zscore_uc.execute(ZScoreInput::new(key.clone(), b"member1".to_vec())).await;
    assert_eq!(result, Ok(Some(1.5)));

    let result = zscore_uc.execute(ZScoreInput::new(key.clone(), b"nonexistent".to_vec())).await;
    assert_eq!(result, Ok(None));

    let result = zrem_uc.execute(ZRemInput::new(key.clone(), vec![b"member1".to_vec()])).await;
    assert_eq!(result, Ok(1));

    let result = zcard_uc.execute(ZCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));
}

#[tokio::test]
async fn test_sorted_set_update_score_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());
    let zscore_uc = ZScoreUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_sorted_set_update");

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 1.0, b"member1".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 2.0, b"member2".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 3.0, b"member3".to_vec())).await;
    assert_eq!(result, Ok(true));

    let result = zadd_uc.execute(ZAddInput::new(key.clone(), 2.5, b"member1".to_vec())).await;
    assert_eq!(result, Ok(false));

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await;
    assert_eq!(
        result,
        Ok(vec![
            b"member2".to_vec(),
            b"member1".to_vec(),
            b"member3".to_vec()
        ])
    );

    let result = zscore_uc.execute(ZScoreInput::new(key.clone(), b"member1".to_vec())).await;
    assert_eq!(result, Ok(Some(2.5)));
}

#[tokio::test]
async fn test_sorted_set_negative_ranges_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;
    let zadd_uc = ZAddUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_sorted_set_ranges");

    for i in 1..=5 {
        let member = format!("member{}", i);
        let result = zadd_uc.execute(ZAddInput::new(key.clone(), i as f64, member.as_bytes().to_vec())).await;
        assert_eq!(result, Ok(true));
    }

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), -2, -1)).await;
    assert_eq!(result, Ok(vec![b"member4".to_vec(), b"member5".to_vec()]));

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 1, -1)).await;
    assert_eq!(
        result,
        Ok(vec![
            b"member2".to_vec(),
            b"member3".to_vec(),
            b"member4".to_vec(),
            b"member5".to_vec()
        ])
    );
}

#[tokio::test]
async fn test_sorted_set_empty_operations_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let sorted_set_repo: Arc<dyn SortedSetRepository> = storage;

    let zcard_uc = ZCardUseCase::new(sorted_set_repo.clone());
    let zrange_uc = ZRangeUseCase::new(sorted_set_repo.clone());
    let zscore_uc = ZScoreUseCase::new(sorted_set_repo.clone());
    let zrem_uc = ZRemUseCase::new(sorted_set_repo.clone());

    let key = Key::from_static("test_empty_sorted_set");

    let result = zcard_uc.execute(ZCardInput::new(key.clone())).await;
    assert_eq!(result, Ok(0));

    let result = zrange_uc.execute(ZRangeInput::new(key.clone(), 0, -1)).await;
    assert_eq!(result, Ok(vec![]));

    let result = zscore_uc.execute(ZScoreInput::new(key.clone(), b"nonexistent".to_vec())).await;
    assert_eq!(result, Ok(None));

    let result = zrem_uc.execute(ZRemInput::new(key.clone(), vec![b"nonexistent".to_vec()])).await;
    assert_eq!(result, Ok(0));
}
