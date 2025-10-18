use super::common::TestApp;
use reqwest::{Client, StatusCode};

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_set_key";
    let _value = "test_set_value";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("test_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let response2 = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("new_value")
        .send()
        .await
        .expect("Failed to execute SET request for overwrite");

    assert_eq!(response2.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request after overwrite");

    assert_eq!(get_response.status(), StatusCode::OK);
    let body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(
        body,
        serde_json::json!({"type": "String", "value": "new_value".as_bytes()})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_delete_nonexistent_key_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let _key = "nonexistent_delete_key";

    let response = client
        .delete(format!("{}/single_key/del/{}", app.address, "non_existent_key"))
        .send()
        .await
        .expect("Failed to execute DELETE request for nonexistent key");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.is_null());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_empty_value_basic_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_empty_value_basic";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("")
        .send()
        .await
        .expect("Failed to execute SET request with empty value");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(
        body,
        serde_json::json!({"type": "String", "value": "".as_bytes()})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_get_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_get_key";
    let value = "test_get_value";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body(value)
        .send()
        .await
        .expect("Failed to set value");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(
        body,
        serde_json::json!({"type": "String", "value": value.as_bytes()})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_delete_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_delete_key";
    let value = "test_delete_value";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body(value)
        .send()
        .await
        .expect("Failed to set value");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let delete_response = client
        .delete(format!("{}/single_key/del/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute DELETE request");

    assert_eq!(delete_response.status(), StatusCode::OK);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_get_nonexistent_key() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "nonexistent_key";

    let response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_hget_nonexistent_field() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_hash_nonexistent";
    let existing_field = "existing_field";
    let nonexistent_field = "nonexistent_field";

    let set_response = client
        .post(format!("{}/hash/hset/{}/{}", app.address, key, existing_field))
        .body("value")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/hash/hget/{}/{}", app.address, key, nonexistent_field))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(body["field"], nonexistent_field);
    assert!(body["value"].is_null());
}
