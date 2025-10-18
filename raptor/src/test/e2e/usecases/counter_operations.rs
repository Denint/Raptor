use super::common::TestApp;
use reqwest::{Client, StatusCode};
use serde_json::json;

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_incr_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_counter_incr";

    let response = client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": 1})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_multiple_incr() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_counter_multiple_incr";


    client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();


    let response = client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": 2})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_decr_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_counter_decr";


    client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();


    let response = client
        .post(format!("{}/counter/decr/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": 0})
    );

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body, serde_json::json!({"type": "Integer", "value": 0}));
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_overflow_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key_overflow = "test_counter_overflow";


    let set_body = json!({"type": "Integer", "value": i64::MAX});
    client
        .post(format!("{}/single_key/set/{}", app.address, key_overflow))
        .json(&set_body)
        .send()
        .await
        .unwrap();

    let response_overflow = client
        .post(format!("{}/counter/incr/{}", app.address, key_overflow))
        .send()
        .await
        .unwrap();
    assert_eq!(response_overflow.status(), StatusCode::OK);
    assert_eq!(
        response_overflow.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": i64::MIN})
    );
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_operations_wrong_type_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();
    let key = "wrong_type_counter";

    client
        .post(format!("{}/single_key/set/{}", app.address, key))
        .body("string_value")
        .send()
        .await
        .unwrap();

    let incr_response = client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(incr_response.status(), StatusCode::BAD_REQUEST);

    let decr_response = client
        .post(format!("{}/counter/decr/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(decr_response.status(), StatusCode::BAD_REQUEST);

    let reset_response = client
        .post(format!("{}/counter/reset/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(reset_response.status(), StatusCode::BAD_REQUEST);
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_decr_negative_prevention_e2e() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_counter_decr_negative";


    client
        .post(format!("{}/counter/incr/{}", app.address, key))
        .send()
        .await
        .unwrap();


    let decr_response = client
        .post(format!("{}/counter/decr/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(decr_response.status(), StatusCode::OK);
    assert_eq!(
        decr_response.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": 0})
    );


    let decr_response2 = client
        .post(format!("{}/counter/decr/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(decr_response2.status(), StatusCode::BAD_REQUEST);


    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body, serde_json::json!({"type": "Integer", "value": 0}));
}


#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_reset_operation() {
    let app = TestApp::spawn().await;
    let client = Client::new();

    let key = "test_counter_reset";


    for _ in 0..10 {
        client
            .post(format!("{}/counter/incr/{}", app.address, key))
            .send()
            .await
            .unwrap();
    }

    let response = client
        .post(format!("{}/counter/reset/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<serde_json::Value>().await.unwrap(),
        serde_json::json!({"value": 10})
    );

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, key))
        .send()
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body, serde_json::json!({"type": "Integer", "value": 0}));
}
