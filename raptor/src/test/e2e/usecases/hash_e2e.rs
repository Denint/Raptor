use base64::{Engine as _, engine::general_purpose};
use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_operations_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_e2e";

    let response = client
        .post(format!("{}/hash/hset/{}/name", app.address, key))
        .body("Alice Johnson")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["is_new_entry"], true);

    let response = client
        .post(format!("{}/hash/hset/{}/age", app.address, key))
        .body("28")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .post(format!("{}/hash/hset/{}/city", app.address, key))
        .body("San Francisco")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .get(format!("{}/hash/hget/{}/name", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let decoded_value = String::from_utf8(
        general_purpose::STANDARD
            .decode(body["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value, "Alice Johnson");

    let response = client
        .get(format!("{}/hash/hexists/{}/name", app.address, key))
        .send()
        .await
        .expect("Failed to execute HEXISTS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["exists"], true);

    let response = client
        .get(format!("{}/hash/hexists/{}/nonexistent", app.address, key))
        .send()
        .await
        .expect("Failed to execute HEXISTS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["exists"], false);

    let response = client
        .get(format!("{}/hash/hkeys/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HKEYS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let keys = body["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&json!("name")));
    assert!(keys.contains(&json!("age")));
    assert!(keys.contains(&json!("city")));

    let response = client
        .get(format!("{}/hash/hvals/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HVALS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let values = body["values"].as_array().unwrap();
    assert_eq!(values.len(), 3);

    for value in values {
        let decoded = String::from_utf8(
            general_purpose::STANDARD
                .decode(value.as_str().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert!(["Alice Johnson", "28", "San Francisco"].contains(&decoded.as_str()));
    }

    let response = client
        .get(format!("{}/hash/hlen/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HLEN request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["len"], 3);

    let response = client
        .post(format!("{}/hash/hdel/{}", app.address, key))
        .json(&json!({
            "fields": ["age", "city"]
        }))
        .send()
        .await
        .expect("Failed to execute HDEL request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["deleted_count"], 2);

    let response = client
        .get(format!("{}/hash/hlen/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HLEN request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["len"], 1);

    let response = client
        .post(format!("{}/hash/hdel/{}", app.address, key))
        .json(&json!({
            "fields": ["nonexistent"]
        }))
        .send()
        .await
        .expect("Failed to execute HDEL request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["deleted_count"], 0);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_hset_empty_field_and_value_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_empty_field_value";

    let response = client
        .post(format!("{}/hash/hset/{}/empty_field", app.address, key))
        .body("value")
        .send()
        .await
        .expect("Failed to execute HSET request with empty field");

    assert_eq!(response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/hash/hget/{}/empty_field", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request for empty field");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    let decoded_value = String::from_utf8(
        general_purpose::STANDARD
            .decode(get_body["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value, "value");

    let response = client
        .post(format!("{}/hash/hset/{}/field_empty_val", app.address, key))
        .body("")
        .send()
        .await
        .expect("Failed to execute HSET request with empty value");

    assert_eq!(response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/hash/hget/{}/field_empty_val", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request for empty value");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    let decoded_value = String::from_utf8(
        general_purpose::STANDARD
            .decode(get_body["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value, "");
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_hdel_empty_fields_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_hdel_empty";

    client
        .post(format!("{}/hash/hset/{}/field1", app.address, key))
        .body("value1")
        .send()
        .await
        .expect("Failed to execute HSET request");

    let response = client
        .post(format!("{}/hash/hdel/{}", app.address, key))
        .json(&json!({
            "fields": []
        }))
        .send()
        .await
        .expect("Failed to execute HDEL request with empty fields");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["deleted_count"], 0);

    let get_response = client
        .get(format!("{}/hash/hget/{}/field1", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    let decoded_value = String::from_utf8(
        general_purpose::STANDARD
            .decode(get_body["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value, "value1");
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_operations_wrong_type_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_wrong_type";

    let response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(response.status(), StatusCode::CREATED);

    let response = client
        .post(format!("{}/hash/hset/{}/test", app.address, key))
        .body("value")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .get(format!("{}/hash/hexists/{}/test", app.address, key))
        .send()
        .await
        .expect("Failed to execute HEXISTS request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .get(format!("{}/hash/hkeys/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HKEYS request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let hget_response = client
        .get(format!("{}/hash/hget/{}/some_field", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request on wrong type");
    assert_eq!(hget_response.status(), StatusCode::BAD_REQUEST);

    let hlen_response = client
        .get(format!("{}/hash/hlen/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HLEN request on wrong type");
    assert_eq!(hlen_response.status(), StatusCode::BAD_REQUEST);

    let hdel_response = client
        .post(format!("{}/hash/hdel/{}", app.address, key))
        .json(&json!({ "fields": ["field1"] }))
        .send()
        .await
        .expect("Failed to execute HDEL request on wrong type");
    assert_eq!(hdel_response.status(), StatusCode::BAD_REQUEST);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_operations_empty_hash_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_empty";

    let response = client
        .get(format!("{}/hash/hkeys/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HKEYS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let keys = body["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 0);

    let response = client
        .get(format!("{}/hash/hvals/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HVALS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let values = body["values"].as_array().unwrap();
    assert_eq!(values.len(), 0);

    let response = client
        .get(format!("{}/hash/hlen/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute HLEN request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["len"], 0);

    let response = client
        .get(format!("{}/hash/hexists/{}/nonexistent", app.address, key))
        .send()
        .await
        .expect("Failed to execute HEXISTS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["exists"], false);
}
