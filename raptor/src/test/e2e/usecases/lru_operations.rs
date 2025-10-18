use super::common::TestApp;
use reqwest::{Client, StatusCode};

#[cfg(not(miri))]
#[tokio::test]
async fn test_lru_eviction_e2e() {
    let app = TestApp::spawn_with_env(&[
        ("LRU_ENABLED", "true"),
        ("MAX_MEMORY_BYTES", "300"),
    ])
    .await;
    let client = Client::new();

    for i in 0..15 {
        let key = format!("lru_key_{:02}", i);
        let value = "x".repeat(80);

        let response = client
            .post(format!("{}/single_key/set/{}", app.address, key))
            .body(value)
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let mut remaining_keys = 0;
    for i in 0..15 {
        let key = format!("lru_key_{:02}", i);
        let response = client
            .get(format!("{}/single_key/get/{}", app.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        if response.status() == StatusCode::OK {
            remaining_keys += 1;
        }
    }

    assert!(
        remaining_keys < 15,
        "Expected some keys to be evicted, but all {} remain",
        remaining_keys
    );
    assert!(
        remaining_keys > 0,
        "All keys were evicted, {} remain",
        remaining_keys
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_lru_access_pattern_e2e() {
    let app =
        TestApp::spawn_with_env(&[("LRU_ENABLED", "true"), ("MAX_MEMORY_BYTES", "800")]).await;
    let client = Client::new();

    for i in 1..=4 {
        let key = format!("access_key{}", i);
        let response = client
            .post(format!("{}/single_key/set/{}", app.address, key))
            .body("medium_value_data")
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let response = client
        .get(format!("{}/single_key/get/access_key1", app.address))
        .send()
        .await
        .expect("Failed to execute GET request for key1");

    assert_eq!(response.status(), StatusCode::OK);

    for i in 5..=12 {
        let key = format!("access_key{}", i);
        let response = client
            .post(format!("{}/single_key/set/{}", app.address, key))
            .body("x".repeat(40))
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let response = client
        .get(format!("{}/single_key/get/access_key1", app.address))
        .send()
        .await
        .expect("Failed to execute GET request for key1");

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "key1 should still exist as most recently accessed"
    );

    let mut remaining_old_keys = 0;
    for i in 2..=4 {
        let key = format!("access_key{}", i);
        let response = client
            .get(format!("{}/single_key/get/{}", app.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        if response.status() == StatusCode::OK {
            remaining_old_keys += 1;
        }
    }

    assert!(
        remaining_old_keys <= 3,
        "Some old keys should potentially be evicted"
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_no_memory_limit_e2e() {
    let app = TestApp::spawn_with_env(&[])
    .await;
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
            "Key {} should still exist when no memory limit is set",
            key
        );
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_lru_memory_calculation_e2e() {
    let app = TestApp::spawn_with_env(&[
        ("LRU_ENABLED", "true"),
        ("MAX_MEMORY_BYTES", "80"),
    ])
    .await;
    let client = Client::new();

    let response1 = client
        .post(format!("{}/single_key/set/{}", app.address, "small_key"))
        .body("a")
        .send()
        .await
        .expect("Failed to execute SET request");
    assert_eq!(response1.status(), StatusCode::CREATED);

    let response2 = client
        .post(format!("{}/single_key/set/{}", app.address, "medium_key"))
        .body("medium_value_here")
        .send()
        .await
        .expect("Failed to execute SET request");
    assert_eq!(response2.status(), StatusCode::CREATED);

    let response3 = client
        .post(format!("{}/single_key/set/{}", app.address, "large_key"))
        .body("x".repeat(50))
        .send()
        .await
        .expect("Failed to execute SET request");
    assert_eq!(response3.status(), StatusCode::CREATED);

    let mut remaining_count = 0;
    let test_keys = ["small_key", "medium_key", "large_key"];

    for &key in &test_keys {
        let response = client
            .get(format!("{}/single_key/get/{}", app.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        if response.status() == StatusCode::OK {
            remaining_count += 1;
        }
    }

    assert!(
        remaining_count < test_keys.len(),
        "Memory limit should cause some eviction"
    );
    assert!(remaining_count > 0, "At least one key should remain");
}
