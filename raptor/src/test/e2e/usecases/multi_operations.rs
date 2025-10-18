use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_mset_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let mset_body = serde_json::json!({
        "pairs": [
            {"key": "test_mset_key1", "value": {"type": "String", "value": "test_value1".as_bytes()}},
            {"key": "test_mset_key2", "value": {"type": "Integer", "value": 123}}
        ]
    });

    let response = client
        .post(format!("{}/multi_key/mset", app.address))
        .json(&mset_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_mget_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let mset_body = serde_json::json!({
        "pairs": [
            {"key": "test_mget_key1", "value": {"type": "String", "value": "test_value1".as_bytes()}},
            {"key": "test_mget_key2", "value": {"type": "Integer", "value": 456}}
        ]
    });

    client
        .post(format!("{}/multi_key/mset", app.address))
        .json(&mset_body)
        .send()
        .await
        .unwrap();

    let mget_body = serde_json::json!({"keys": ["test_mget_key1", "test_mget_key2", "nonexistent_key"]});

    let response = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&mget_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let result: serde_json::Value = response.json().await.unwrap();
    let values = result["values"].as_array().unwrap();
    assert_eq!(values.len(), 3);

    assert_eq!(values[0], json!({"type": "String", "value": "test_value1".as_bytes()}));

    assert_eq!(values[1], json!({"type": "Integer", "value": 456}));

    assert!(values[2].is_null());

    let mget_body_empty = serde_json::json!({"keys": []});
    let response_empty = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&mget_body_empty)
        .send()
        .await
        .unwrap();

    assert_eq!(response_empty.status(), StatusCode::OK);
    let result_empty: serde_json::Value = response_empty.json().await.unwrap();
    assert!(result_empty["values"].as_array().unwrap().is_empty());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_mdel_empty_keys_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let mdel_body = serde_json::json!({"keys": []});
    let response = client
        .post(format!("{}/multi_key/mdel", app.address))
        .json(&mdel_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let result: serde_json::Value = response.json().await.unwrap();
    assert!(result["values"].as_array().unwrap().is_empty());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_mset_overwrite_type_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_mset_overwrite_type";

    client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("initial_string")
        .send()
        .await
        .unwrap();

    let mset_body = json!({
        "pairs": [
            {"key": key, "value": {"type": "Integer", "value": 456}}
        ]
    });
    let response = client
        .post(format!("{}/multi_key/mset", app.address))
        .json(&mset_body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let get_body = serde_json::json!({"keys": [key]});
    let get_response = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&get_body)
        .send()
        .await
        .unwrap();
    let get_result: serde_json::Value = get_response.json().await.unwrap();
    let values = get_result["values"].as_array().unwrap();
    assert_eq!(values[0], json!({"type": "Integer", "value": 456}));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_mget_after_type_conversion_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key1 = "mget_conv_key1";
    let key2 = "mget_conv_key2";

    client.post(format!("{}/single_key/set/{}", app.address, key1)).body("string_val").send().await.unwrap();
    client.post(format!("{}/single_key/set/{}", app.address, key2)).body("123").send().await.unwrap();

    let array_body = json!({"values": [{"type": "String", "value": "arr_val".as_bytes()}]});
    client.post(format!("{}/single_key/array/{}", app.address, key1)).json(&array_body).send().await.unwrap();

    let mget_body = serde_json::json!({"keys": [key1, key2]});
    let response = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&mget_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let result: serde_json::Value = response.json().await.unwrap();
    let values = result["values"].as_array().unwrap();
    assert_eq!(values[0]["type"], "Array");
    assert!(values[0]["value"].is_array());
    assert_eq!(values[1], json!({"type": "String", "value": "123".as_bytes()}));
}
