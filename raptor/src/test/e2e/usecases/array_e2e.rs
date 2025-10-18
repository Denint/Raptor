use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_set_get_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_array";

    let set_body = json!({
        "values": [
            {"type": "String", "value": "value1".as_bytes()},
            {"type": "Integer", "value": 42},
            {"type": "String", "value": "value3".as_bytes()}
        ]
    });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let get_body = json!({
        "indices": [0, 1, 2, 3]
    });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_result: serde_json::Value = get_response.json().await.unwrap();

    assert_eq!(get_result["values"].as_array().unwrap().len(), 4);
    assert_eq!(get_result["values"][0]["type"], "String");
    let value1_bytes: Vec<u8> = get_result["values"][0]["value"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    assert_eq!(String::from_utf8(value1_bytes).unwrap(), "value1");
    assert_eq!(get_result["values"][1]["type"], "Integer");
    assert_eq!(get_result["values"][1]["value"].as_i64().unwrap(), 42);
    assert_eq!(get_result["values"][2]["type"], "String");
    let value3_bytes: Vec<u8> = get_result["values"][2]["value"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    assert_eq!(String::from_utf8(value3_bytes).unwrap(), "value3");
    assert!(get_result["values"][3].is_null());
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_set_empty_array_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_empty_array";

    let set_body = json!({
        "values": []
    });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request with empty array");

    assert_eq!(set_response.status(), StatusCode::OK);

    let length_response = client
        .get(format!("{}/single_key/array/len/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ARRAY_LENGTH request");

    assert_eq!(length_response.status(), StatusCode::OK);
    let length_result: serde_json::Value = length_response.json().await.unwrap();
    assert_eq!(length_result["length"], 0);
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_get_nonexistent_key_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "nonexistent_array_key";

    let get_body = json!({
        "indices": [0, 1]
    });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request for nonexistent key");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_result: serde_json::Value = get_response.json().await.unwrap();

    assert_eq!(get_result["values"].as_array().unwrap().len(), 2);
    assert!(get_result["values"][0].is_null());
    assert!(get_result["values"][1].is_null());
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_append_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_array_append";

    let set_body = json!({
        "values": [
            {"type": "String", "value": "initial".as_bytes()}
        ]
    });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute initial ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let append_body = json!({
        "values": [
            {"type": "Integer", "value": 100},
            {"type": "String", "value": "appended".as_bytes()}
        ]
    });
    let append_response = client
        .put(format!("{}/single_key/array/append/{}", app.address, key))
        .json(&append_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_APPEND request");

    assert_eq!(append_response.status(), StatusCode::OK);
    let append_result: serde_json::Value = append_response.json().await.unwrap();
    assert_eq!(append_result["new_length"], 3);

    let get_body = json!({
        "indices": [0, 1, 2]
    });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_result: serde_json::Value = get_response.json().await.unwrap();

    assert_eq!(get_result["values"][0]["type"], "String");
    assert_eq!(get_result["values"][1]["type"], "Integer");
    assert_eq!(get_result["values"][1]["value"], 100);
    assert_eq!(get_result["values"][2]["type"], "String");
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_slice_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_array_slice";

    let values: Vec<serde_json::Value> = (0..5)
        .map(|i| json!({"type": "String", "value": format!("value{}", i).as_bytes()}))
        .collect();

    let set_body = json!({ "values": values });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let slice_body = json!({
        "start": 1,
        "end": 4
    });
    let slice_response = client
        .post(format!("{}/single_key/array/slice/{}", app.address, key))
        .json(&slice_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SLICE request");

    assert_eq!(slice_response.status(), StatusCode::OK);
    let slice_result: serde_json::Value = slice_response.json().await.unwrap();

    let values = slice_result["values"].as_array().unwrap();
    assert_eq!(values.len(), 3);
    for (i, value) in values.iter().enumerate().take(3) {
        assert_eq!(value["type"], "String");
        let expected_value = format!("value{}", i + 1);
        let actual_bytes: Vec<u8> = value["value"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u8)
            .collect();
        let actual_value = String::from_utf8(actual_bytes).unwrap();
        assert_eq!(actual_value, expected_value);
    }

    let slice_body2 = json!({
        "start": 3
    });
    let slice_response2 = client
        .post(format!("{}/single_key/array/slice/{}", app.address, key))
        .json(&slice_body2)
        .send()
        .await
        .expect("Failed to execute ARRAY_SLICE request without end");

    assert_eq!(slice_response2.status(), StatusCode::OK);
    let slice_result2: serde_json::Value = slice_response2.json().await.unwrap();

    let values2 = slice_result2["values"].as_array().unwrap();
    assert_eq!(values2.len(), 2);

    let slice_body3 = json!({
        "start": 4,
        "end": 2
    });
    let slice_response3 = client
        .post(format!("{}/single_key/array/slice/{}", app.address, key))
        .json(&slice_body3)
        .send()
        .await
        .expect("Failed to execute ARRAY_SLICE request for start > end");

    assert_eq!(slice_response3.status(), StatusCode::OK);
    let slice_result3: serde_json::Value = slice_response3.json().await.unwrap();
    assert!(slice_result3["values"].as_array().unwrap().is_empty());

    let slice_body4 = json!({
        "start": 10,
        "end": 12
    });
    let slice_response4 = client
        .post(format!("{}/single_key/array/slice/{}", app.address, key))
        .json(&slice_body4)
        .send()
        .await
        .expect("Failed to execute ARRAY_SLICE request for start out of bounds");

    assert_eq!(slice_response4.status(), StatusCode::OK);
    let slice_result4: serde_json::Value = slice_response4.json().await.unwrap();
    assert!(slice_result4["values"].as_array().unwrap().is_empty());

    let empty_key = "empty_array_for_slice";
    let set_empty_body = json!({ "values": [] });
    client
        .post(format!("{}/single_key/array/{}", app.address, empty_key))
        .json(&set_empty_body)
        .send()
        .await
        .expect("Failed to set empty array");
    let slice_body5 = json!({
        "start": 0,
        "end": 10
    });
    let slice_response5 = client
        .post(format!("{}/single_key/array/slice/{}", app.address, empty_key))
        .json(&slice_body5)
        .send()
        .await
        .expect("Failed to execute ARRAY_SLICE request on empty array");
    assert_eq!(slice_response5.status(), StatusCode::OK);
    let slice_result5: serde_json::Value = slice_response5.json().await.unwrap();
    assert!(slice_result5["values"].as_array().unwrap().is_empty());
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_update_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_array_update";

    let set_body = json!({
        "values": [
            {"type": "String", "value": "original1".as_bytes()},
            {"type": "String", "value": "original2".as_bytes()},
            {"type": "String", "value": "original3".as_bytes()}
        ]
    });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let update_body = json!({
        "updates": [
            {"index": 0, "value": {"type": "String", "value": "updated1".as_bytes()}},
            {"index": 2, "value": {"type": "Integer", "value": 999}},
            {"index": 5, "value": {"type": "String", "value": "out_of_bounds".as_bytes()}}
        ]
    });
    let update_response = client
        .put(format!("{}/single_key/array/update/{}", app.address, key))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_UPDATE request");

    assert_eq!(update_response.status(), StatusCode::OK);
    let update_result: serde_json::Value = update_response.json().await.unwrap();
    assert_eq!(update_result["updated_count"], 2);

    let get_body = json!({
        "indices": [0, 1, 2]
    });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_result: serde_json::Value = get_response.json().await.unwrap();

    assert_eq!(get_result["values"][0]["type"], "String");
    assert_eq!(get_result["values"][1]["type"], "String");
    assert_eq!(get_result["values"][2]["type"], "Integer");
    assert_eq!(get_result["values"][2]["value"], 999);

    let update_body2 = json!({ "updates": [] });
    let update_response2 = client
        .put(format!("{}/single_key/array/update/{}", app.address, key))
        .json(&update_body2)
        .send()
        .await
        .expect("Failed to execute ARRAY_UPDATE request with empty updates");

    assert_eq!(update_response2.status(), StatusCode::OK);
    let update_result2: serde_json::Value = update_response2.json().await.unwrap();
    assert_eq!(update_result2["updated_count"], 0);

    let get_body2 = json!({ "indices": [0, 1, 2] });
    let get_response2 = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body2)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request");

    assert_eq!(get_response2.status(), StatusCode::OK);
    let get_result2: serde_json::Value = get_response2.json().await.unwrap();

    assert_eq!(get_result2["values"][0]["type"], "String");
    let val0_bytes: Vec<u8> = get_result2["values"][0]["value"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    assert_eq!(String::from_utf8(val0_bytes).unwrap(), "updated1");
    assert_eq!(get_result2["values"][1]["type"], "String");
    let val1_bytes: Vec<u8> = get_result2["values"][1]["value"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    assert_eq!(String::from_utf8(val1_bytes).unwrap(), "original2");
    assert_eq!(get_result2["values"][2]["type"], "Integer");
    assert_eq!(get_result2["values"][2]["value"], 999);
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_length_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_array_length";

    let length_response1 = client
        .get(format!("{}/single_key/array/len/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ARRAY_LENGTH request for non-existent key");

    assert_eq!(length_response1.status(), StatusCode::OK);
    let length_result1: serde_json::Value = length_response1.json().await.unwrap();
    assert!(length_result1["length"].is_null());

    let set_body = json!({
        "values": [
            {"type": "String", "value": "item1".as_bytes()},
            {"type": "Integer", "value": 42}
        ]
    });
    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let length_response2 = client
        .get(format!("{}/single_key/array/len/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ARRAY_LENGTH request");

    assert_eq!(length_response2.status(), StatusCode::OK);
    let length_result2: serde_json::Value = length_response2.json().await.unwrap();
    assert_eq!(length_result2["length"], 2);
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_operations_wrong_type_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "wrong_type_key";

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let get_body = json!({ "indices": [0] });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request on wrong type");

    assert!(!get_response.status().is_success());

    let append_body = json!({
        "values": [{"type": "String", "value": "new_value".as_bytes()}]
    });
    let append_response = client
        .put(format!("{}/single_key/array/append/{}", app.address, key))
        .json(&append_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_APPEND request on wrong type");

    assert!(!append_response.status().is_success());

    let length_response = client
        .get(format!("{}/single_key/array/len/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ARRAY_LENGTH request on wrong type");

    assert!(!length_response.status().is_success());

    let append_wrong_type_body = json!({
        "values": [{"type": "String", "value": "new_value".as_bytes()}]
    });
    let append_wrong_type_response = client
        .put(format!("{}/single_key/array/append/{}", app.address, key))
        .json(&append_wrong_type_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_APPEND request on wrong type");

    assert_eq!(append_wrong_type_response.status(), StatusCode::BAD_REQUEST);
}

#[cfg(not(miri))]
#[cfg(not(miri))]
#[tokio::test]
async fn test_array_complex_operations_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "complex_array";

    let set_body = json!({
        "values": [
            {"type": "String", "value": "string_value".as_bytes()},
            {"type": "Integer", "value": 42},
            {
                "type": "Hash",
                "value": {
                    "field1": "hash_value1".as_bytes(),
                    "field2": "hash_value2".as_bytes()
                }
            },
            {
                "type": "List",
                "value": ["list_item1".as_bytes(), "list_item2".as_bytes()]
            }
        ]
    });

    let set_response = client
        .post(format!("{}/single_key/array/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_SET request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let get_body = json!({ "indices": [0, 1, 2, 3] });
    let get_response = client
        .post(format!("{}/single_key/array/get/{}", app.address, key))
        .json(&get_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_GET request");

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_result: serde_json::Value = get_response.json().await.unwrap();

    let values = get_result["values"].as_array().unwrap();
    assert_eq!(values.len(), 4);
    assert_eq!(values[0]["type"], "String");
    assert_eq!(values[1]["type"], "Integer");
    assert_eq!(values[1]["value"], 42);
    assert_eq!(values[2]["type"], "Hash");
    assert_eq!(values[3]["type"], "List");

    let update_body = json!({
        "updates": [
            {"index": 1, "value": {"type": "Integer", "value": 100}}
        ]
    });

    let update_response = client
        .put(format!("{}/single_key/array/update/{}", app.address, key))
        .json(&update_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_UPDATE request");

    assert_eq!(update_response.status(), StatusCode::OK);

    let append_body = json!({
        "values": [
            {"type": "String", "value": "appended1".as_bytes()},
            {"type": "Integer", "value": 200}
        ]
    });

    let append_response = client
        .put(format!("{}/single_key/array/append/{}", app.address, key))
        .json(&append_body)
        .send()
        .await
        .expect("Failed to execute ARRAY_APPEND request");

    assert_eq!(append_response.status(), StatusCode::OK);
    let append_result: serde_json::Value = append_response.json().await.unwrap();
    assert_eq!(append_result["new_length"], 6);

    let length_response = client
        .get(format!("{}/single_key/array/len/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ARRAY_LENGTH request");

    assert_eq!(length_response.status(), StatusCode::OK);
    let length_result: serde_json::Value = length_response.json().await.unwrap();
    assert_eq!(length_result["length"], 6);
}
