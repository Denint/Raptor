use super::common::TestApp;
use reqwest::{Client, StatusCode};

#[cfg(not(miri))]
#[tokio::test]
async fn test_lru_eviction_integration() {
    let app = TestApp::spawn_with_env(&[
        ("LRU_ENABLED", "true"),
        ("MAX_MEMORY_BYTES", "1000"),
    ]).await;
    let client = Client::new();

    for i in 0..20 {
        let key = format!("lru_key_{:02}", i);
        let value = "x".repeat(100);

        let response = client
            .post(format!("{}/single_key/set/{}", app.address, key))
            .body(value)
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let mut remaining_keys = 0;
    let mut evicted_keys = 0;
    for i in 0..20 {
        let key = format!("lru_key_{:02}", i);
        let response = client
            .get(format!("{}/single_key/get/{}", app.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        if response.status() == StatusCode::OK {
            remaining_keys += 1;
        } else if response.status() == StatusCode::NOT_FOUND {
            evicted_keys += 1;
        }
    }

    assert!(remaining_keys < 20, "Expected some keys to be evicted");
    assert!(remaining_keys > 0, "Expected some keys to remain");
    assert!(evicted_keys > 0, "Expected some keys to be evicted");
    assert!((5..=10).contains(&remaining_keys), "Expected 5-10 keys to remain, but {} remained", remaining_keys);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_lru_disabled_integration() {
    let app = TestApp::spawn_with_env(&[]).await;
    let client = Client::new();

    for i in 0..10 {
        let key = format!("no_lru_key_{:02}", i);
        let response = client
            .post(format!("{}/single_key/set/{}", app.address, key))
            .body("value")
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    for i in 0..10 {
        let key = format!("no_lru_key_{:02}", i);
        let response = client
            .get(format!("{}/single_key/get/{}", app.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Key {} should still exist when LRU is disabled",
            key
        );
    }
}
