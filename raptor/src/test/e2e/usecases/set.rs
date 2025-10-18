use super::common::TestApp;
use base64::{Engine as _, engine::general_purpose};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::collections::HashSet;

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_sadd_smembers() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_set";

    let response = client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [
                general_purpose::STANDARD.encode("alice"),
                general_purpose::STANDARD.encode("bob")
            ]
        }))
        .send()
        .await
        .expect("Failed to execute SADD request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["added_count"], 2);

    let response = client
        .get(format!("{}/set/smembers/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute SMEMBERS request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    let members = body["members"].as_array().unwrap();

    let decoded_members: HashSet<String> = members
        .iter()
        .map(|m| {
            String::from_utf8(
                general_purpose::STANDARD
                    .decode(m.as_str().unwrap())
                    .unwrap(),
            )
            .unwrap()
        })
        .collect();

    assert_eq!(decoded_members.len(), 2);
    assert!(decoded_members.contains("alice"));
    assert!(decoded_members.contains("bob"));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_sadd_empty_members_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_sadd_empty";

    client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [general_purpose::STANDARD.encode("initial")]
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": []
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["added_count"], 0);

    let scard_response = client
        .get(format!("{}/set/scard/{}", app.address, key))
        .send()
        .await
        .unwrap();
    let scard_body: serde_json::Value = scard_response.json().await.unwrap();
    assert_eq!(scard_body["card"], 1);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_srem_empty_members_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_srem_empty";

    client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [general_purpose::STANDARD.encode("initial")]
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/set/srem/{}", app.address, key))
        .json(&json!({
            "members": []
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["removed_count"], 0);

    let scard_response = client
        .get(format!("{}/set/scard/{}", app.address, key))
        .send()
        .await
        .unwrap();
    let scard_body: serde_json::Value = scard_response.json().await.unwrap();
    assert_eq!(scard_body["card"], 1);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_sismember_empty_member_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_sismember_empty";

    client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [
                general_purpose::STANDARD.encode("nonempty"),
                general_purpose::STANDARD.encode("")
            ]
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/set/sismember/{}", app.address, key))
        .json(&json!({
            "member": general_purpose::STANDARD.encode("")
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["is_member"], true);

    let nonexistent_key = "nonexistent_set_sismember";
    let response = client
        .post(format!("{}/set/sismember/{}", app.address, nonexistent_key))
        .json(&json!({
            "member": general_purpose::STANDARD.encode("")
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["is_member"], false);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_srem_sismember_scard() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_set_ops";

    let response = client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [
                general_purpose::STANDARD.encode("alice"),
                general_purpose::STANDARD.encode("bob"),
                general_purpose::STANDARD.encode("charlie")
            ]
        }))
        .send()
        .await
        .expect("Failed to execute SADD request");

    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .get(format!("{}/set/scard/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute SCARD request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["card"], 3);

    let response = client
        .post(format!("{}/set/sismember/{}", app.address, key))
        .json(&json!({
            "member": general_purpose::STANDARD.encode("alice")
        }))
        .send()
        .await
        .expect("Failed to execute SISMEMBER request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["is_member"], true);

    let response = client
        .post(format!("{}/set/sismember/{}", app.address, key))
        .json(&json!({
            "member": general_purpose::STANDARD.encode("dave")
        }))
        .send()
        .await
        .expect("Failed to execute SISMEMBER request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["is_member"], false);

    let response = client
        .post(format!("{}/set/srem/{}", app.address, key))
        .json(&json!({
            "members": [general_purpose::STANDARD.encode("alice")]
        }))
        .send()
        .await
        .expect("Failed to execute SREM request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["removed_count"], 1);

    let response = client
        .get(format!("{}/set/scard/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute SCARD request");

    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["card"], 2);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_wrong_type() {
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

    let response = client
        .post(format!("{}/set/sadd/{}", app.address, key))
        .json(&json!({
            "members": [general_purpose::STANDARD.encode("member1")]
        }))
        .send()
        .await
        .expect("Failed to execute SADD request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
