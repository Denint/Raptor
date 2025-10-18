use super::common::TestApp;
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_lpush_rpop() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_list";

    let lpush_body = json!({
        "values": [
            general_purpose::STANDARD.encode("value1"),
            general_purpose::STANDARD.encode("value2"),
            general_purpose::STANDARD.encode("value3")
        ]
    });
    let lpush_response = client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body)
        .send()
        .await
        .expect("Failed to execute LPUSH request");

    assert_eq!(lpush_response.status(), StatusCode::OK);
    let lpush_result: serde_json::Value = lpush_response.json().await.unwrap();
    assert_eq!(lpush_result["new_length"], 3);

    let rpop_response1 = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request");

    assert_eq!(rpop_response1.status(), StatusCode::OK);
    let rpop_result1: serde_json::Value = rpop_response1.json().await.unwrap();
    let decoded_value1 = String::from_utf8(
        general_purpose::STANDARD
            .decode(rpop_result1["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value1, "value1");

    let rpop_response2 = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request");

    assert_eq!(rpop_response2.status(), StatusCode::OK);
    let rpop_result2: serde_json::Value = rpop_response2.json().await.unwrap();
    let decoded_value2 = String::from_utf8(
        general_purpose::STANDARD
            .decode(rpop_result2["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value2, "value2");

    let rpop_response3 = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request");

    assert_eq!(rpop_response3.status(), StatusCode::OK);
    let rpop_result3: serde_json::Value = rpop_response3.json().await.unwrap();
    let decoded_value3 = String::from_utf8(
        general_purpose::STANDARD
            .decode(rpop_result3["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value3, "value3");

    let rpop_response4 = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request");

    assert_eq!(rpop_response4.status(), StatusCode::OK);
    let rpop_result4: serde_json::Value = rpop_response4.json().await.unwrap();
    assert!(rpop_result4["value"].is_null());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_multiple_lpush() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_list_multi";

    let lpush_body1 = json!({
        "values": [general_purpose::STANDARD.encode("first")]
    });
    let lpush_response1 = client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body1)
        .send()
        .await
        .expect("Failed to execute first LPUSH request");

    assert_eq!(lpush_response1.status(), StatusCode::OK);
    let lpush_result1: serde_json::Value = lpush_response1.json().await.unwrap();
    assert_eq!(lpush_result1["new_length"], 1);

    let lpush_body2 = json!({
        "values": [
            general_purpose::STANDARD.encode("second"),
            general_purpose::STANDARD.encode("third")
        ]
    });
    let lpush_response2 = client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body2)
        .send()
        .await
        .expect("Failed to execute second LPUSH request");

    assert_eq!(lpush_response2.status(), StatusCode::OK);
    let lpush_result2: serde_json::Value = lpush_response2.json().await.unwrap();
    assert_eq!(lpush_result2["new_length"], 3);

    let rpop_response = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request");

    assert_eq!(rpop_response.status(), StatusCode::OK);
    let rpop_result: serde_json::Value = rpop_response.json().await.unwrap();
    let decoded_value = String::from_utf8(
        general_purpose::STANDARD
            .decode(rpop_result["value"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(decoded_value, "first");
}




#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_wrong_type() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "wrong_type_list";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .unwrap();

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let lpush_body = json!({
        "values": [general_purpose::STANDARD.encode("list_value")]
    });
    let lpush_response = client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body)
        .send()
        .await
        .expect("Failed to execute LPUSH request on wrong type");

    assert_eq!(lpush_response.status(), StatusCode::BAD_REQUEST);

    let rpop_response = client
        .post(format!("{}/list/rpop/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute RPOP request on wrong type");

    assert_eq!(rpop_response.status(), StatusCode::BAD_REQUEST);

    let lrange_response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .expect("Failed to execute LRANGE request on wrong type");

    assert_eq!(lrange_response.status(), StatusCode::BAD_REQUEST);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_lrange_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_lrange";

    let lpush_body = json!({
        "values": [
            general_purpose::STANDARD.encode("one"),
            general_purpose::STANDARD.encode("two"),
            general_purpose::STANDARD.encode("three"),
            general_purpose::STANDARD.encode("four"),
            general_purpose::STANDARD.encode("five")
        ]
    });
    client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body)
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": 2 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members: Vec<String> = body["values"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| String::from_utf8(general_purpose::STANDARD.decode(m.as_str().unwrap()).unwrap()).unwrap())
        .collect();
    assert_eq!(members, vec!["five", "four", "three"]);

    let response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": -3, "stop": -1 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members: Vec<String> = body["values"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| String::from_utf8(general_purpose::STANDARD.decode(m.as_str().unwrap()).unwrap()).unwrap())
        .collect();
    assert_eq!(members, vec!["three", "two", "one"]);

    let response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": 100 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members: Vec<String> = body["values"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| String::from_utf8(general_purpose::STANDARD.decode(m.as_str().unwrap()).unwrap()).unwrap())
        .collect();
    assert_eq!(members, vec!["five", "four", "three", "two", "one"]);

    let response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 3, "stop": 1 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body["values"].as_array().unwrap();
    assert!(members.is_empty());

    let empty_key = "empty_list_lrange";
    client
        .post(format!("{}/list/lpush/{}", app.address, empty_key))
        .json(&json!({ "values": [] }))
        .send()
        .await
        .unwrap();
    let response = client
        .post(format!("{}/list/lrange/{}", app.address, empty_key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body["values"].as_array().unwrap();
    assert!(members.is_empty());
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_empty_values_vec_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key_lpush = "empty_lpush_e2e";
    let key_rpush = "empty_rpush_e2e";

    let lpush_body = json!({ "values": [] });
    let lpush_response = client
        .post(format!("{}/list/lpush/{}", app.address, key_lpush))
        .json(&lpush_body)
        .send()
        .await
        .unwrap();
    assert_eq!(lpush_response.status(), StatusCode::OK);
    let lpush_result: serde_json::Value = lpush_response.json().await.unwrap();
    assert_eq!(lpush_result["new_length"], 0);

    let rpush_body = json!({ "values": [] });
    let rpush_response = client
        .post(format!("{}/list/rpush/{}", app.address, key_rpush))
        .json(&rpush_body)
        .send()
        .await
        .unwrap();
    assert_eq!(rpush_response.status(), StatusCode::OK);
    let rpush_result: serde_json::Value = rpush_response.json().await.unwrap();
    assert_eq!(rpush_result["new_length"], 0);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_empty_then_refill_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_refill_list";

    let lpush_body = json!({
        "values": [
            general_purpose::STANDARD.encode("first"),
            general_purpose::STANDARD.encode("second")
        ]
    });
    client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body)
        .send()
        .await
        .unwrap();

    client.post(format!("{}/list/rpop/{}", app.address, key)).send().await.unwrap();
    client.post(format!("{}/list/rpop/{}", app.address, key)).send().await.unwrap();

    let get_response = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .unwrap();
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert!(get_body["values"].as_array().unwrap().is_empty());

    let lpush_body2 = json!({
        "values": [
            general_purpose::STANDARD.encode("third"),
            general_purpose::STANDARD.encode("fourth")
        ]
    });
    client
        .post(format!("{}/list/lpush/{}", app.address, key))
        .json(&lpush_body2)
        .send()
        .await
        .unwrap();

    let get_response2 = client
        .post(format!("{}/list/lrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .unwrap();
    let get_body2: serde_json::Value = get_response2.json().await.unwrap();
    let members: Vec<String> = get_body2["values"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| String::from_utf8(general_purpose::STANDARD.decode(m.as_str().unwrap()).unwrap()).unwrap())
        .collect();
    assert_eq!(members, vec!["fourth", "third"]);
}

