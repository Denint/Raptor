use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

#[cfg(not(miri))]
#[tokio::test]
async fn test_single_key_set_get_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_single_key_set_get";

    let set_string_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .expect("Failed to execute SET request for string");
    assert_eq!(set_string_response.status(), StatusCode::CREATED);

    let get_string_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request for string");
    assert_eq!(get_string_response.status(), StatusCode::OK);
    let get_string_body: Value = get_string_response.json().await.unwrap();
    assert_eq!(get_string_body, json!({"type": "String", "value": "string_value".as_bytes()}));

    let key_int = "test_single_key_int";
    let set_int_body = json!({
        "type": "Integer",
        "value": 123
    });
    let set_int_response = client
        .post(format!("{}/single_key/set/{}", app.address, key_int))
        .json(&set_int_body)
        .send()
        .await
        .expect("Failed to execute SET request for integer");
    assert_eq!(set_int_response.status(), StatusCode::CREATED);

    let get_int_response = client
        .get(format!("{}/single_key/get/{}", app.address, key_int))
        .send()
        .await
        .expect("Failed to execute GET request for integer");
    assert_eq!(get_int_response.status(), StatusCode::OK);
    let get_int_body: Value = get_int_response.json().await.unwrap();
    assert_eq!(get_int_body, json!({"type": "Integer", "value": 123}));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_single_key_delete_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_single_key_delete";

    client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("value_to_delete")
        .send()
        .await
        .unwrap();

    let delete_response = client
        .delete(format!("{}/single_key/del/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute DELETE request");
    assert_eq!(delete_response.status(), StatusCode::OK);
    let delete_body = delete_response.text().await.unwrap();
    assert_eq!(delete_body, "Key deleted");

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}
