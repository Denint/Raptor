use super::common::TestApp;
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_zadd_zrange() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_sorted_set";

    let members_data = vec![
        ("member1", 1.5),
        ("member2", 2.0),
        ("member3", 0.5),
        ("member4", 3.0),
    ];

    for (member, score) in &members_data {
        let member_b64 = general_purpose::STANDARD.encode(member.as_bytes());
        let response = client
            .post(format!("{}/sorted_set/zadd/{}", app.address, key))
            .json(&json!({
                "score": score,
                "member": member_b64
            }))
            .send()
            .await
            .expect("Failed to execute ZADD request");

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = response.json().await.unwrap();
        assert!(body["is_new_entry"].as_bool().unwrap());
    }

    let response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({
            "start": 0,
            "stop": -1
        }))
        .send()
        .await
        .expect("Failed to execute ZRANGE request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body.as_array().unwrap();

    let decoded_members: Vec<String> = members
        .iter()
        .map(|m| {
            let b64 = m.as_str().unwrap();
            String::from_utf8(general_purpose::STANDARD.decode(b64).unwrap()).unwrap()
        })
        .collect();

    assert_eq!(
        decoded_members,
        vec!["member3", "member1", "member2", "member4"]
    );

    let response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({
            "start": 1,
            "stop": 2
        }))
        .send()
        .await
        .expect("Failed to execute ZRANGE request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body.as_array().unwrap();

    let decoded_members: Vec<String> = members
        .iter()
        .map(|m| {
            let b64 = m.as_str().unwrap();
            String::from_utf8(general_purpose::STANDARD.decode(b64).unwrap()).unwrap()
        })
        .collect();

    assert_eq!(decoded_members, vec!["member1", "member2"]);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_zadd_min_max_nan_scores_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_zset_min_max_nan";

    let member_max_b64 = general_purpose::STANDARD.encode("max_member");
    client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": f64::MAX,
            "member": member_max_b64
        }))
        .send()
        .await
        .unwrap();

    let member_min_b64 = general_purpose::STANDARD.encode("min_member");
    client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": f64::MIN,
            "member": member_min_b64
        }))
        .send()
        .await
        .unwrap();

    let member_nan_b64 = general_purpose::STANDARD.encode("nan_member");
    client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": f64::NAN,
            "member": member_nan_b64
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body.as_array().unwrap();

    let decoded_members: Vec<String> = members
        .iter()
        .map(|m| String::from_utf8(general_purpose::STANDARD.decode(m.as_str().unwrap()).unwrap()).unwrap())
        .collect();

    assert!(decoded_members.len() >= 2);
    assert!(decoded_members.contains(&"min_member".to_string()));
    assert!(decoded_members.contains(&"max_member".to_string()));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_zrem_empty_members_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_zrem_empty";

    let member_b64 = general_purpose::STANDARD.encode("initial");
    client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": 1.0,
            "member": member_b64
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/sorted_set/zrem/{}", app.address, key))
        .json(&json!({
            "members": []
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["removed_count"], 0);

    let zcard_response = client
        .get(format!("{}/sorted_set/zcard/{}", app.address, key))
        .send()
        .await
        .unwrap();
    let zcard_body: serde_json::Value = zcard_response.json().await.unwrap();
    assert_eq!(zcard_body["card"], 1);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_zrange_single_element_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_zrange_single";

    let member_b64 = general_purpose::STANDARD.encode("single_member");
    client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": 1.0,
            "member": member_b64
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": 0 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body.as_array().unwrap();
    let decoded_member = String::from_utf8(general_purpose::STANDARD.decode(members[0].as_str().unwrap()).unwrap()).unwrap();
    assert_eq!(decoded_member, "single_member");

    let response_neg = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({ "start": -1, "stop": -1 }))
        .send()
        .await
        .unwrap();
    assert_eq!(response_neg.status(), StatusCode::OK);
    let body_neg: serde_json::Value = response_neg.json().await.unwrap();
    let members_neg = body_neg.as_array().unwrap();
    let decoded_member_neg = String::from_utf8(general_purpose::STANDARD.decode(members_neg[0].as_str().unwrap()).unwrap()).unwrap();
    assert_eq!(decoded_member_neg, "single_member");
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_zrem_zcard_zscore() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_sorted_set_ops";

    let member_b64 = general_purpose::STANDARD.encode("member1");
    let response = client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": 1.5,
            "member": member_b64
        }))
        .send()
        .await
        .expect("Failed to execute ZADD request");

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .get(format!("{}/sorted_set/zcard/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ZCARD request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["card"].as_u64().unwrap(), 1);

    let member_b64 = general_purpose::STANDARD.encode("member1");
    let response = client
        .get(format!("{}/sorted_set/zscore/{}/{}", app.address, key, member_b64))
        .send()
        .await
        .expect("Failed to execute ZSCORE request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["score"].as_f64().unwrap(), 1.5);

    let member_b64 = general_purpose::STANDARD.encode("member1");
    let response = client
        .post(format!("{}/sorted_set/zrem/{}", app.address, key))
        .json(&json!({
            "members": [member_b64]
        }))
        .send()
        .await
        .expect("Failed to execute ZREM request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["removed_count"].as_u64().unwrap(), 1);

    let response = client
        .get(format!("{}/sorted_set/zcard/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute ZCARD request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["card"].as_u64().unwrap(), 0);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_wrong_type() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "wrong_type_key";

    let response = client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(response.status(), StatusCode::CREATED);

    let member_b64 = general_purpose::STANDARD.encode("member1");
    let response = client
        .post(format!("{}/sorted_set/zadd/{}", app.address, key))
        .json(&json!({
            "score": 1.5,
            "member": member_b64
        }))
        .send()
        .await
        .expect("Failed to execute ZADD request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, key))
        .json(&json!({ "start": 0, "stop": -1 }))
        .send()
        .await
        .expect("Failed to execute ZRANGE request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .post(format!("{}/sorted_set/zrem/{}", app.address, key))
        .json(&json!({ "members": [general_purpose::STANDARD.encode("member1")] }))
        .send()
        .await
        .expect("Failed to execute ZREM request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .get(format!("{}/sorted_set/zscore/{}/{}", app.address, key, general_purpose::STANDARD.encode("member1")))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = client
        .get(format!("{}/sorted_set/zcard/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
