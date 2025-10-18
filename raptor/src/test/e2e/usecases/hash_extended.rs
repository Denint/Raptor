use super::common::TestApp;
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_operations_extended() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_extended";

    let hset_response1 = client
        .post(format!("{}/hash/hset/{}/name", app.address, key))
        .body("John Doe")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response1.status(), StatusCode::OK);

    let hset_response2 = client
        .post(format!("{}/hash/hset/{}/age", app.address, key))
        .body("30")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response2.status(), StatusCode::OK);

    let hset_response3 = client
        .post(format!("{}/hash/hset/{}/city", app.address, key))
        .body("New York")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response3.status(), StatusCode::OK);

    let response = client
        .get(format!("{}/hash/hexists/{}/name", app.address, key))
        .send()
        .await
        .expect("Failed to execute HEXISTS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["exists"], true);

    let response = client
        .get(format!("{}/hash/hexists/{}/country", app.address, key))
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
            "fields": ["age"]
        }))
        .send()
        .await
        .expect("Failed to execute HDEL request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["deleted_count"], 1);

    let response = client
        .post(format!("{}/hash/hdel/{}", app.address, key))
        .json(&json!({
            "fields": ["name", "city", "nonexistent"]
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
    assert_eq!(body["len"], 0);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_hset_empty_field_and_value_extended_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_empty_field_value_extended";

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
async fn test_hash_hdel_empty_fields_extended_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash_hdel_empty_extended";

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
async fn test_hash_operations_with_regular_key_conflict() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "conflict_key";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("regular_string_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let hset_response = client
        .post(format!("{}/hash/hset/{}/field1", app.address, key))
        .body("value1")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert!(hset_response.status().is_client_error());

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
async fn test_hash_operations() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_hash";

    let hset_response1 = client
        .post(format!("{}/hash/hset/{}/name", app.address, key))
        .body("John Doe")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response1.status(), StatusCode::OK);
    let hset_result1: serde_json::Value = hset_response1.json().await.unwrap();
    assert_eq!(hset_result1["is_new_entry"], true);

    let hset_response2 = client
        .post(format!("{}/hash/hset/{}/age", app.address, key))
        .body("30")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response2.status(), StatusCode::OK);
    let hset_result2: serde_json::Value = hset_response2.json().await.unwrap();
    assert_eq!(hset_result2["is_new_entry"], true);

    let hget_response1 = client
        .get(format!("{}/hash/hget/{}/name", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(hget_response1.status(), StatusCode::OK);
    let hget_result1: serde_json::Value = hget_response1.json().await.unwrap();
    assert_eq!(
        String::from_utf8(
            general_purpose::STANDARD
                .decode(hget_result1["value"].as_str().unwrap())
                .unwrap()
        )
        .unwrap(),
        "John Doe"
    );

    let hget_response2 = client
        .get(format!("{}/hash/hget/{}/age", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(hget_response2.status(), StatusCode::OK);
    let hget_result2: serde_json::Value = hget_response2.json().await.unwrap();
    assert_eq!(
        String::from_utf8(
            general_purpose::STANDARD
                .decode(hget_result2["value"].as_str().unwrap())
                .unwrap()
        )
        .unwrap(),
        "30"
    );

    let hget_response3 = client
        .get(format!("{}/hash/hget/{}/nonexistent", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(hget_response3.status(), StatusCode::OK);
    let hget_result3: serde_json::Value = hget_response3.json().await.unwrap();
    assert!(hget_result3["value"].is_null());

    let hset_response3 = client
        .post(format!("{}/hash/hset/{}/age", app.address, key))
        .body("31")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response3.status(), StatusCode::OK);
    let hset_result3: serde_json::Value = hset_response3.json().await.unwrap();
    assert_eq!(hset_result3["is_new_entry"], false);

    let hget_response4 = client
        .get(format!("{}/hash/hget/{}/age", app.address, key))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(hget_response4.status(), StatusCode::OK);
    let hget_result4: serde_json::Value = hget_response4.json().await.unwrap();
    assert_eq!(
        String::from_utf8(
            general_purpose::STANDARD
                .decode(hget_result4["value"].as_str().unwrap())
                .unwrap()
        )
        .unwrap(),
        "31"
    );
}
