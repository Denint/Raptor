use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

#[cfg(not(miri))]
#[tokio::test]
async fn test_ttl_set_get_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_ttl_key";

    let set_body = json!({
        "value": {"type": "String", "value": "ttl_value".as_bytes()},
        "ttl_seconds": 2
    });
    let set_response = client
        .post(format!("{}/single_key/set_with_ttl/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .expect("Failed to execute SET_WITH_TTL request");

    assert_eq!(set_response.status(), StatusCode::OK);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request");
    assert_eq!(get_response.status(), StatusCode::OK);

    let ttl_response = client
        .get(format!("{}/single_key/ttl/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute TTL request");
    assert_eq!(ttl_response.status(), StatusCode::OK);
    let ttl_result: Value = ttl_response.json().await.unwrap();
    assert!(ttl_result["ttl"].as_i64().unwrap() > 0);

    sleep(Duration::from_millis(2100)).await;

    let get_expired_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request for expired key");
    assert_eq!(get_expired_response.status(), StatusCode::NOT_FOUND);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_ttl_persist_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_persist_key";

    let set_body = json!({
        "value": {"type": "String", "value": "persist_value".as_bytes()},
        "ttl_seconds": 2
    });
    client
        .post(format!("{}/single_key/set_with_ttl/{}", app.address, key))
        .json(&set_body)
        .send()
        .await
        .unwrap();

    let persist_response = client
        .post(format!("{}/single_key/persist/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute PERSIST request");

    assert_eq!(persist_response.status(), StatusCode::OK);
    let persist_result: Value = persist_response.json().await.unwrap();
    assert_eq!(persist_result["persisted"], true);

    let ttl_response = client
        .get(format!("{}/single_key/ttl/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute TTL request");
    assert_eq!(ttl_response.status(), StatusCode::OK);
    let ttl_result: Value = ttl_response.json().await.unwrap();
    assert_eq!(ttl_result["ttl"], -1);

    sleep(Duration::from_secs(3)).await;
    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request after persist");
    assert_eq!(get_response.status(), StatusCode::OK);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_ttl_expire_zero_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "test_expire_zero_key";

    client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("zero_ttl_value")
        .send()
        .await
        .unwrap();

    let expire_response = client
        .post(format!("{}/single_key/expire/{}", app.address, key))
        .json(&json!({ "ttl_seconds": 0 }))
        .send()
        .await
        .expect("Failed to execute EXPIRE request with 0 TTL");

    assert_eq!(expire_response.status(), StatusCode::OK);
    let expire_result: Value = expire_response.json().await.unwrap();
    assert_eq!(expire_result["expired"], true);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .expect("Failed to execute GET request for zero TTL key");
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}
