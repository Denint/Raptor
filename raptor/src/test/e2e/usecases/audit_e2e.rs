use super::common::TestApp;
use reqwest::{Client, StatusCode};
use std::fs;
use std::path::Path;

fn get_audit_file_path(base_path: &str) -> String {
    base_path.to_string()
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_basic_operations_audit_logging() {
    let temp_dir = std::env::temp_dir();
    let audit_log_path = temp_dir.join("test_audit_basic.log").to_string_lossy().to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, "audit_test_key"))
        .body("audit_test_value")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, "audit_test_key"))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(get_response.status(), StatusCode::OK);

    let delete_response = client
        .delete(format!("{}/single_key/del/{}", app.address, "audit_test_key"))
        .send()
        .await
        .expect("Failed to execute DELETE request");

    assert_eq!(delete_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    if !Path::new(&audit_file_path).exists() {
        panic!("Audit log file was not created: {}", audit_file_path);
    }

    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_file_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut found_set = false;
    let mut found_get = false;
    let mut found_delete = false;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("set") => {
                    found_set = true;
                    assert_eq!(log_entry["fields"]["key"], "audit_test_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("get") => {
                    found_get = true;
                    assert_eq!(log_entry["fields"]["key"], "audit_test_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("delete") => {
                    found_delete = true;
                    assert_eq!(log_entry["fields"]["key"], "audit_test_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                _ => {}
            }
        }
    }

    assert!(found_set, "SET operation should be logged");
    assert!(found_get, "GET operation should be logged");
    assert!(found_delete, "DELETE operation should be logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_hash_operations_audit_logging() {
    let audit_log_path = "/tmp/test_audit_hash.log".to_string();

    if let Some(parent) = Path::new(&audit_log_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();


    let hset_response1 = client
        .post(format!("{}/hash/hset/{}/{}", app.address, "audit_hash_key", "field1"))
        .body("value1")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response1.status(), StatusCode::OK);

    let hset_response2 = client
        .post(format!("{}/hash/hset/{}/{}", app.address, "audit_hash_key", "field2"))
        .body("value2")
        .send()
        .await
        .expect("Failed to execute HSET request");

    assert_eq!(hset_response2.status(), StatusCode::OK);

    let hget_response = client
        .get(format!("{}/hash/hget/{}/{}", app.address, "audit_hash_key", "field1"))
        .send()
        .await
        .expect("Failed to execute HGET request");

    assert_eq!(hget_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");


    let mut found_hset = false;
    let mut found_hget = false;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("hset") => {
                    if log_entry["fields"]["key"] == "audit_hash_key" && log_entry["fields"]["success"] == true {
                        found_hset = true;
                    }
                }
                Some("hget") => {
                    if log_entry["fields"]["key"] == "audit_hash_key" && log_entry["fields"]["success"] == true {
                        found_hget = true;
                    }
                }
                _ => {}
            }
        }
    }

    assert!(found_hset, "Should have HSET operations logged");
    assert!(found_hget, "Should have HGET operations logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_counter_operations_audit_logging() {
    let temp_dir = std::env::temp_dir();
    let audit_log_path = temp_dir.join("test_audit_counter.log").to_string_lossy().to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    let incr_response = client
        .post(format!("{}/counter/incr/{}", app.address, "audit_counter_key"))
        .json(&serde_json::json!({"value": 5}))
        .send()
        .await
        .expect("Failed to execute INCR request");

    assert_eq!(incr_response.status(), StatusCode::OK);

    let incr_response2 = client
        .post(format!("{}/counter/incr/{}", app.address, "audit_counter_key"))
        .json(&serde_json::json!({"value": 10}))
        .send()
        .await
        .expect("Failed to execute INCR request");

    assert_eq!(incr_response2.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut incr_count = 0;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op"))
            && op.as_str() == Some("incr") {
                incr_count += 1;
                assert_eq!(log_entry["fields"]["key"], "audit_counter_key");
                assert_eq!(log_entry["fields"]["success"], true);
            }
    }

    assert!(incr_count >= 2, "Should have at least 2 INCR operations logged");
    assert!(incr_count <= 2 + 5, "Should not have too many INCR operations logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_list_operations_audit_logging() {
    let audit_log_path = "/tmp/test_audit_list.log".to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    let lpush_response1 = client
        .post(format!("{}/list/lpush/{}", app.address, "audit_list_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"values": ["aXRlbTE="]}"#)
        .send()
        .await
        .expect("Failed to execute LPUSH request");

    assert_eq!(lpush_response1.status(), StatusCode::OK);

    let lpush_response2 = client
        .post(format!("{}/list/lpush/{}", app.address, "audit_list_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"values": ["aXRlbTI="]}"#)
        .send()
        .await
        .expect("Failed to execute LPUSH request");

    assert_eq!(lpush_response2.status(), StatusCode::OK);

    let lrange_response = client
        .post(format!("{}/list/lrange/{}", app.address, "audit_list_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"start": 0, "stop": -1}"#)
        .send()
        .await
        .expect("Failed to execute LRANGE request");

    assert_eq!(lrange_response.status(), StatusCode::OK);

    let lpop_response = client
        .post(format!("{}/list/lpop/{}", app.address, "audit_list_key"))
        .send()
        .await
        .expect("Failed to execute LPOP request");

    assert_eq!(lpop_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut lpush_count = 0;
    let mut lrange_count = 0;
    let mut lpop_count = 0;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("lpush") => {
                    lpush_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_list_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("lrange") => {
                    lrange_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_list_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("lpop") => {
                    lpop_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_list_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                _ => {}
            }
        }
    }

    assert!(lpush_count >= 2, "Should have at least 2 LPUSH operations logged");
    assert!(lpush_count <= 2 + 5, "Should not have too many LPUSH operations logged");
    assert_eq!(lrange_count, 1, "Should have 1 LRANGE operation logged");
    assert_eq!(lpop_count, 1, "Should have 1 LPOP operation logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_set_operations_audit_logging() {
    let audit_log_path = "/tmp/test_audit_set.log".to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    let sadd_response1 = client
        .post(format!("{}/set/sadd/{}", app.address, "audit_set_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"members": ["bWVtYmVyMQ=="]}"#)
        .send()
        .await
        .expect("Failed to execute SADD request");

    assert_eq!(sadd_response1.status(), StatusCode::OK);

    let sadd_response2 = client
        .post(format!("{}/set/sadd/{}", app.address, "audit_set_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"members": ["bWVtYmVyMg=="]}"#)
        .send()
        .await
        .expect("Failed to execute SADD request");

    assert_eq!(sadd_response2.status(), StatusCode::OK);

    let smembers_response = client
        .get(format!("{}/set/smembers/{}", app.address, "audit_set_key"))
        .send()
        .await
        .expect("Failed to execute SMEMBERS request");

    assert_eq!(smembers_response.status(), StatusCode::OK);

    let sismember_response = client
        .post(format!("{}/set/sismember/{}", app.address, "audit_set_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"member": "bWVtYmVyMQ=="}"#)
        .send()
        .await
        .expect("Failed to execute SISMEMBER request");

    assert_eq!(sismember_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut sadd_count = 0;
    let mut smembers_count = 0;
    let mut sismember_count = 0;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("sadd") => {
                    sadd_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_set_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("smembers") => {
                    smembers_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_set_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("sismember") => {
                    sismember_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_set_key");
                    assert_eq!(log_entry["fields"]["member"], "member1");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                _ => {}
            }
        }
    }

    assert!(sadd_count >= 2, "Should have at least 2 SADD operations logged");
    assert!(sadd_count <= 2 + 5, "Should not have too many SADD operations logged");
    assert_eq!(smembers_count, 1, "Should have 1 SMEMBERS operation logged");
    assert_eq!(sismember_count, 1, "Should have 1 SISMEMBER operation logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_sorted_set_operations_audit_logging() {
    let audit_log_path = "/tmp/test_audit_sorted_set.log".to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    let zadd_response1 = client
        .post(format!("{}/sorted_set/zadd/{}", app.address, "audit_zset_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"member": "bWVtYmVyMQ==", "score": 1.0}"#)
        .send()
        .await
        .expect("Failed to execute ZADD request");

    assert_eq!(zadd_response1.status(), StatusCode::OK);

    let zadd_response2 = client
        .post(format!("{}/sorted_set/zadd/{}", app.address, "audit_zset_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"member": "bWVtYmVyMg==", "score": 2.0}"#)
        .send()
        .await
        .expect("Failed to execute ZADD request");

    assert_eq!(zadd_response2.status(), StatusCode::OK);

    let zrange_response = client
        .post(format!("{}/sorted_set/zrange/{}", app.address, "audit_zset_key"))
        .header("Content-Type", "application/json")
        .body(r#"{"start": 0, "stop": -1}"#)
        .send()
        .await
        .expect("Failed to execute ZRANGE request");

    assert_eq!(zrange_response.status(), StatusCode::OK);

    let zscore_response = client
        .get(format!("{}/sorted_set/zscore/{}/{}", app.address, "audit_zset_key", "bWVtYmVyMQ=="))
        .send()
        .await
        .expect("Failed to execute ZSCORE request");

    assert_eq!(zscore_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut zadd_count = 0;
    let mut zrange_count = 0;
    let mut zscore_count = 0;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("zadd") => {
                    zadd_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_zset_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("zrange") => {
                    zrange_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_zset_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("zscore") => {
                    zscore_count += 1;
                    assert_eq!(log_entry["fields"]["key"], "audit_zset_key");
                    assert_eq!(log_entry["fields"]["member"], "member1");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                _ => {}
            }
        }
    }

    assert!(zadd_count >= 2, "Should have at least 2 ZADD operations logged");
    assert!(zadd_count <= 2 + 5, "Should not have too many ZADD operations logged");
    assert_eq!(zrange_count, 1, "Should have 1 ZRANGE operation logged");
    assert_eq!(zscore_count, 1, "Should have 1 ZSCORE operation logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_error_operations_audit_logging() {
    let temp_dir = std::env::temp_dir();
    let audit_log_path = temp_dir.join("test_audit_error.log").to_string_lossy().to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    let set_response = client
        .post(format!("{}/single_key/set/{}", app.address, "audit_error_key"))
        .body("not_a_number")
        .send()
        .await
        .expect("Failed to execute SET request");

    assert_eq!(set_response.status(), StatusCode::CREATED);

    let incr_response = client
        .post(format!("{}/counter/incr/{}", app.address, "audit_error_key"))
        .json(&serde_json::json!({"value": 1}))
        .send()
        .await
        .expect("Failed to execute INCR request");

    assert_eq!(incr_response.status(), StatusCode::BAD_REQUEST);

    let get_response = client
        .get(format!("{}/single_key/get/{}", app.address, "non_existent_key"))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut found_error_incr = false;
    let mut found_successful_set = false;
    let mut found_get_nonexistent = false;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("set") => {
                    found_successful_set = true;
                    assert_eq!(log_entry["fields"]["key"], "audit_error_key");
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("incr") => {
                    if log_entry["fields"]["success"] == false {
                        found_error_incr = true;
                        assert_eq!(log_entry["fields"]["key"], "audit_error_key");
                    }
                }
                Some("get") => {
                    if log_entry["fields"]["key"] == "non_existent_key" {
                        found_get_nonexistent = true;
                        assert_eq!(log_entry["fields"]["success"], true);
                    }
                }
                _ => {}
            }
        }
    }

    assert!(found_successful_set, "SET operation should be logged successfully");
    assert!(found_error_incr, "Failed INCR operation should be logged with success=false");
    assert!(found_get_nonexistent, "GET on non-existent key should be logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}

#[cfg(not(miri))]
#[tokio::test]
async fn test_multi_key_operations_audit_logging() {
    let temp_dir = std::env::temp_dir();
    let audit_log_path = temp_dir.join("test_audit_multi.log").to_string_lossy().to_string();

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }

    let app = TestApp::spawn_with_env(&[("AUDIT_LOG_PATH", &audit_log_path)]).await;
    let client = Client::new();

    client
        .post(format!("{}/single_key/set/{}", app.address, "multi_key1"))
        .body("value1")
        .send()
        .await
        .expect("Failed to set key1");

    client
        .post(format!("{}/single_key/set/{}", app.address, "multi_key2"))
        .body("value2")
        .send()
        .await
        .expect("Failed to set key2");

    client
        .post(format!("{}/single_key/set/{}", app.address, "multi_key3"))
        .body("value3")
        .send()
        .await
        .expect("Failed to set key3");

    let mget_response = client
        .post(format!("{}/multi_key/mget", app.address))
        .json(&serde_json::json!({"keys": ["multi_key1", "multi_key2", "multi_key3"]}))
        .send()
        .await
        .expect("Failed to execute MGET request");

    assert_eq!(mget_response.status(), StatusCode::OK);

    let mset_data = serde_json::json!({
        "pairs": [
            {"key": "multi_key4", "value": {"type": "String", "value": [118, 97, 108, 117, 101, 52]}},
            {"key": "multi_key5", "value": {"type": "String", "value": [118, 97, 108, 117, 101, 53]}}
        ]
    });

    let mset_response = client
        .post(format!("{}/multi_key/mset", app.address))
        .json(&mset_data)
        .send()
        .await
        .expect("Failed to execute MSET request");

    assert_eq!(mset_response.status(), StatusCode::OK);

    let mdel_response = client
        .post(format!("{}/multi_key/mdel", app.address))
        .json(&serde_json::json!({"keys": ["multi_key1", "multi_key2"]}))
        .send()
        .await
        .expect("Failed to execute MDEL request");

    assert_eq!(mdel_response.status(), StatusCode::OK);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let audit_file_path = get_audit_file_path(&audit_log_path);
    let log_content = fs::read_to_string(&audit_file_path)
        .unwrap_or_else(|_| panic!("Failed to read audit log file: {}", audit_log_path));

    let log_lines: Vec<&str> = log_content.lines().collect();
    assert!(!log_lines.is_empty(), "Audit log should contain entries");

    let mut set_count = 0;
    let mut mget_count = 0;
    let mut mset_count = 0;
    let mut mdel_count = 0;

    for line in log_lines {
        if line.trim().is_empty() {
            continue;
        }

        let log_entry: serde_json::Value = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Failed to parse JSON log entry '{}': {}", line, e));

        if let Some(op) = log_entry.get("fields").and_then(|f| f.get("op")) {
            match op.as_str() {
                Some("set") => set_count += 1,
                Some("mget") => {
                    mget_count += 1;
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("mset") => {
                    mset_count += 1;
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                Some("mdel") => {
                    mdel_count += 1;
                    assert_eq!(log_entry["fields"]["success"], true);
                }
                _ => {}
            }
        }
    }

    assert!(set_count >= 3, "Should have at least 3 SET operations logged");
    assert!(set_count <= 3 + 5, "Should not have too many SET operations logged");
    assert_eq!(mget_count, 1, "Should have 1 MGET operation logged");
    assert_eq!(mset_count, 1, "Should have 1 MSET operation logged");
    assert_eq!(mdel_count, 1, "Should have 1 MDEL operation logged");

    if Path::new(&audit_log_path).exists() {
        let _ = fs::remove_file(&audit_log_path);
    }
}
