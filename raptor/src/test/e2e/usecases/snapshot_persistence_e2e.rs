use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::{self, json};
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};

#[cfg(not(miri))]
#[tokio::test]
async fn test_snapshot_persistence_e2e() {
    let snapshot_path = "test_snapshot_e2e.bin";

    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }

    let app1 = TestApp::spawn_with_env(&[
        ("SNAPSHOT_INTERVAL_SECONDS", "1"),
        ("SNAPSHOT_PATH", snapshot_path),
    ]).await;

    let client = Client::new();

    let test_data = vec![
        ("key1", "value1"),
        ("key2", "value2"),
        ("string_key", "string_value"),
    ];

    for (key, value) in &test_data {
        let response = client
            .post(format!("{}/single_key/set/{}", app1.address, key))
            .body(*value)
            .send()
            .await
            .expect("Failed to execute SET request");

        assert_eq!(response.status(), StatusCode::CREATED);
    }


    for _ in 0..5 {
        let incr_response = client
            .post(format!("{}/counter/incr/counter_key", app1.address))
            .send()
            .await
            .unwrap();
        let status = incr_response.status();
        if status != StatusCode::OK {
            let error_text = incr_response.text().await.unwrap();
            panic!("Counter incr failed with status {}: {}", status, error_text);
        }
    }

    let hset_response = client
        .post(format!("{}/hash/hset/hash_key/hash_field", app1.address))
        .body("hash_value")
        .send()
        .await
        .unwrap();
    assert_eq!(hset_response.status(), StatusCode::OK);

    let array_data = json!({
        "values": [
            {"type": "String", "value": "array_item1".as_bytes()},
            {"type": "Integer", "value": 100}
        ]
    });
    let array_response = client
        .post(format!("{}/single_key/array/array_key", app1.address))
        .json(&array_data)
        .send()
        .await
        .unwrap();
    assert_eq!(array_response.status(), StatusCode::OK);

    sleep(Duration::from_millis(2000)).await;

    assert!(Path::new(snapshot_path).exists(), "Snapshot file should exist");

    drop(app1);

    let app2 = TestApp::spawn_with_env(&[
        ("SNAPSHOT_INTERVAL_SECONDS", "1"),
        ("SNAPSHOT_PATH", snapshot_path),
    ]).await;

    sleep(Duration::from_millis(500)).await;

    for (key, expected_value) in &test_data {
        let response = client
            .get(format!("{}/single_key/get/{}", app2.address, key))
            .send()
            .await
            .expect("Failed to execute GET request");

        assert_eq!(response.status(), StatusCode::OK, "Key {} should exist", key);

        let body: serde_json::Value = response.json().await.unwrap();
        let actual_value = String::from_utf8(
            body["value"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect::<Vec<u8>>(),
        )
        .unwrap();

        assert_eq!(actual_value, *expected_value, "Value for key {} should match", key);
    }

    let counter_response = client
        .get(format!("{}/single_key/get/counter_key", app2.address))
        .send()
        .await
        .unwrap();
    assert_eq!(counter_response.status(), StatusCode::OK);
    let counter_body: serde_json::Value = counter_response.json().await.unwrap();
    assert_eq!(counter_body["value"], json!(5));

    let hash_response = client
        .get(format!("{}/hash/hget/hash_key/hash_field", app2.address))
        .send()
        .await
        .unwrap();
    assert_eq!(hash_response.status(), StatusCode::OK);

    let array_get_response = client
        .post(format!("{}/single_key/array/get/array_key", app2.address))
        .json(&json!({"indices": [0, 1]}))
        .send()
        .await
        .unwrap();
    assert_eq!(array_get_response.status(), StatusCode::OK);

    let array_body: serde_json::Value = array_get_response.json().await.unwrap();
    let values = array_body["values"].as_array().unwrap();
    assert_eq!(values.len(), 2);

    drop(app2);
    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_snapshot_persistence_no_existing_file_e2e() {
    let snapshot_path = "test_nonexistent_snapshot.bin";

    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }

    let app = TestApp::spawn_with_env(&[
        ("SNAPSHOT_INTERVAL_SECONDS", "1"),
        ("SNAPSHOT_PATH", snapshot_path),
    ]).await;

    let client = Client::new();

    let response = client
        .post(format!("{}/single_key/set/test_key", app.address))
        .body("test_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(response.status(), StatusCode::CREATED);

    drop(app);
    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_snapshot_persistence_complex_data_e2e() {
    let snapshot_path = "test_complex_snapshot.bin";

    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }

    let app1 = TestApp::spawn_with_env(&[
        ("SNAPSHOT_INTERVAL_SECONDS", "1"),
        ("SNAPSHOT_PATH", snapshot_path),
    ]).await;

    let client = Client::new();

    let complex_data = vec![
        ("string_key", "string_value"),
        ("unicode_key", "привет мир 🌍"),
        ("empty_key", ""),
        ("number_key", "12345"),
    ];

    for (key, value) in &complex_data {
        client
            .post(format!("{}/single_key/set/{}", app1.address, key))
            .body(*value)
            .send()
            .await
            .unwrap();
    }


    client
        .post(format!("{}/single_key/set/neg_counter", app1.address))
        .json(&json!({"type": "Integer", "value": -50}))
        .send()
        .await
        .unwrap();

    let large_value = "x".repeat(10000);
    client
        .post(format!("{}/single_key/set/large_key", app1.address))
        .body(large_value)
        .send()
        .await
        .unwrap();

    sleep(Duration::from_millis(1500)).await;

    drop(app1);

    let app2 = TestApp::spawn_with_env(&[
        ("SNAPSHOT_INTERVAL_SECONDS", "1"),
        ("SNAPSHOT_PATH", snapshot_path),
    ]).await;

    for (key, expected) in &complex_data {
        let response = client
            .get(format!("{}/single_key/get/{}", app2.address, key))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = response.json().await.unwrap();
        let actual = String::from_utf8(
            body["value"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as u8)
                .collect(),
        )
        .unwrap();
        assert_eq!(actual, *expected);
    }

    let counter_response = client
        .get(format!("{}/single_key/get/neg_counter", app2.address))
        .send()
        .await
        .unwrap();
    let counter_body: serde_json::Value = counter_response.json().await.unwrap();
    assert_eq!(counter_body["value"], json!(-50));

    let large_response = client
        .get(format!("{}/single_key/get/large_key", app2.address))
        .send()
        .await
        .unwrap();
    assert_eq!(large_response.status(), StatusCode::OK);

    drop(app2);
    if Path::new(snapshot_path).exists() {
        fs::remove_file(snapshot_path).unwrap();
    }
}
