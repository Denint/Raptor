use crate::{
    application::use_cases::multi_key::operations::{
        mdel_use_case::{MDelUseCase, MDelInput},
        mget_use_case::{MGetUseCase, MGetInput},
        mset_use_case::{MSetUseCase, MSetInput},
    },
    domain::{
        repositories::MultiRepository,
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

#[tokio::test]
async fn test_multi_key_mset_mget_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let multi_repo: Arc<dyn MultiRepository> = storage;

    let mset_uc = MSetUseCase::new(multi_repo.clone());
    let mget_uc = MGetUseCase::new(multi_repo.clone());

    let key1 = Key::from_static("mkey1");
    let key2 = Key::from_static("mkey2");
    let key3 = Key::from_static("mkey3");

    let pairs = vec![
        (key1.clone(), Value::String(b"value1".to_vec())),
        (key2.clone(), Value::Integer(100)),
    ];

    let mset_result = mset_uc.execute(MSetInput::new(pairs.clone())).await;
    assert_eq!(mset_result, Ok(()));

    let keys_to_get = vec![key1.clone(), key2.clone(), key3.clone()];
    let mget_result = mget_uc.execute(MGetInput::new(keys_to_get)).await;
    assert_eq!(
        mget_result,
        Ok(vec![
            Some(Value::String(b"value1".to_vec())),
            Some(Value::Integer(100)),
            None,
        ])
    );
}

#[tokio::test]
async fn test_multi_key_mdel_integration() {
    let config = Arc::new(crate::infrastructure::config::create_test_config());

    let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));
    let multi_repo: Arc<dyn MultiRepository> = storage;

    let mset_uc = MSetUseCase::new(multi_repo.clone());
    let mdel_uc = MDelUseCase::new(multi_repo.clone());
    let mget_uc = MGetUseCase::new(multi_repo.clone());

    let key1 = Key::from_static("mdel_key1");
    let key2 = Key::from_static("mdel_key2");

    mset_uc.execute(MSetInput::new(vec![
        (key1.clone(), Value::String(b"val1".to_vec())),
        (key2.clone(), Value::String(b"val2".to_vec())),
    ])).await.unwrap();

    let keys_to_del = vec![key1.clone(), Key::from_static("nonexistent_key")];
    let mdel_result = mdel_uc.execute(MDelInput::new(keys_to_del)).await;
    assert_eq!(
        mdel_result,
        Ok(vec![
            Some(Value::String(b"val1".to_vec())),
            None,
        ])
    );

    let get_result = mget_uc.execute(MGetInput::new(vec![key1.clone()])).await;
    assert_eq!(get_result, Ok(vec![None]));

    let get_result2 = mget_uc.execute(MGetInput::new(vec![key2.clone()])).await;
    assert_eq!(get_result2, Ok(vec![Some(Value::String(b"val2".to_vec()))]));
}
