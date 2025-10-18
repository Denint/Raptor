use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_get_delete_flow() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_key";
    let value = "test_value";
    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body(value)
        .send()
        .await
        .expect("Failed to execute SET request.");
    assert_eq!(set_response.status(), StatusCode::CREATED);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request.");
    assert_eq!(get_response.status(), StatusCode::OK);

    let get_body: Value = get_response.json().await.unwrap();
    assert_eq!(get_body, json!({"type": "String", "value": value.as_bytes()}));

    let delete_response = client
        .delete(format!("{}/single_key/del/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute DELETE request.");
    assert_eq!(delete_response.status(), StatusCode::OK);
    let delete_body = delete_response.text().await.unwrap();
    assert_eq!(delete_body, "Key deleted");

    let get_after_delete_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request.");
    assert_eq!(get_after_delete_response.status(), StatusCode::NOT_FOUND);

    let nonexistent_key = "nonexistent_delete_key";
    let delete_nonexistent_response = client
        .delete(format!("{}/single_key/del/{}", app.address, nonexistent_key))
        .send()
        .await
        .expect("Failed to execute DELETE request for nonexistent key.");
    assert_eq!(delete_nonexistent_response.status(), StatusCode::OK);
    let delete_nonexistent_body: Value = delete_nonexistent_response.json().await.unwrap();
    assert!(delete_nonexistent_body.is_null());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_overwrite_with_different_type_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_overwrite_type";

    let set_response1 = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("initial_string")
        .send()
        .await
        .expect("Failed to execute initial SET request");
    assert_eq!(set_response1.status(), StatusCode::CREATED);

    let set_body2 = json!({
        "type": "Integer",
        "value": 123
    });
    let set_response2 = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .json(&set_body2)
        .send()
        .await
        .expect("Failed to execute SET request with Integer");
    assert_eq!(set_response2.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: Value = get_response.json().await.unwrap();
    assert_eq!(get_body, json!({"type": "Integer", "value": 123}));

    let set_body3 = json!({
        "type": "String",
        "value": "new_string".as_bytes()
    });
    let set_response3 = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .json(&set_body3)
        .send()
        .await
        .expect("Failed to execute SET request with String");
    assert_eq!(set_response3.status(), StatusCode::OK);

    let get_response2 = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");
    assert_eq!(get_response2.status(), StatusCode::OK);
    let get_body2: Value = get_response2.json().await.unwrap();
    assert_eq!(get_body2, json!({"type": "String", "value": "new_string".as_bytes()}));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_empty_value_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_empty_value";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("")
        .send()
        .await
        .expect("Failed to execute SET request with empty value.");
    assert_eq!(set_response.status(), StatusCode::CREATED);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request for empty value.");
    assert_eq!(get_response.status(), StatusCode::OK);

    let get_body: Value = get_response.json().await.unwrap();
    assert_eq!(get_body, json!({"type": "String", "value": "".as_bytes()}));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_new_commands_flow() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "counter";

    let incr_response = client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(incr_response.status(), StatusCode::OK);
    assert_eq!(
        incr_response.json::<Value>().await.unwrap(),
        json!({"value": 1})
    );

    let decr_response = client
        .post(format!("{}/counter/decr/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(decr_response.status(), StatusCode::OK);
    assert_eq!(
        decr_response.json::<Value>().await.unwrap(),
        json!({"value": 0})
    );


    for _ in 0..4 {
        client
            .post(format!("{}/counter/incr/{}", app.address, key))
            .send()
            .await
            .unwrap();
    }

    let incr_response2 = client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(incr_response2.status(), StatusCode::OK);
    assert_eq!(
        incr_response2.json::<Value>().await.unwrap(),
        json!({"value": 5})
    );

    let reset_response = client
        .post(format!("{}/counter/reset/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(reset_response.status(), StatusCode::OK);
    assert_eq!(
        reset_response.json::<Value>().await.unwrap(),
        json!({"value": 5})
    );

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: Value = get_response.json().await.unwrap();
    assert_eq!(get_body, json!({"type": "Integer", "value": 0}));

    let mset_body = json!({
        "pairs": [
            {"key": "key1", "value": {"type": "String", "value": "value1".as_bytes()}},
            {"key": "key2", "value": {"type": "Integer", "value": 123}}
        ]
    });
    let mset_response = client
        .post(format!("{}/multi_key/mset", app.address))
        .json(&mset_body)
        .send()
        .await
        .unwrap();
    assert_eq!(mset_response.status(), StatusCode::OK);

    let mget_body = serde_json::json!({"keys": ["key1", "key2", "missing_key"]});
    let mget_response = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&mget_body)
        .send()
        .await
        .unwrap();
    assert_eq!(mget_response.status(), StatusCode::OK);
    let mget_result: Value = mget_response.json().await.unwrap();
    let values = mget_result["values"].as_array().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], json!({"type": "String", "value": "value1".as_bytes()}));
    assert_eq!(values[1], json!({"type": "Integer", "value": 123}));
    assert!(values[2].is_null());

    let mdel_body = serde_json::json!({"keys": ["key1", "key2"]});
    let mdel_response = client
        .post(format!("{}/multi_key/mdel", app.address))
        .json(&mdel_body)
        .send()
        .await
        .unwrap();
    assert_eq!(mdel_response.status(), StatusCode::OK);
    let mdel_result: Value = mdel_response.json().await.unwrap();
    let deleted_values = mdel_result["values"].as_array().unwrap();

    assert_eq!(deleted_values.len(), 2);
    assert_eq!(deleted_values[0], json!({"type": "String", "value": "value1".as_bytes()}));
    assert_eq!(deleted_values[1], json!({"type": "Integer", "value": 123}));
}
