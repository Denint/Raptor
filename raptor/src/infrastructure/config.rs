use anyhow::Result;
use sysinfo::System;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub log_format: LogFormat,
    pub max_memory_bytes: u64,
    pub snapshot_interval_seconds: Option<u64>,
    pub audit_log_path: Option<String>,
    pub snapshot_path: Option<String>,

    pub smart_lru_eviction_threshold: f64,
    pub smart_lru_check_interval_seconds: u64,
    pub smart_lru_min_eviction_ratio: f64,
    pub smart_lru_max_eviction_ratio: f64,
    pub smart_lru_ttl_weight: f64,
    pub smart_lru_access_weight: f64,
    pub smart_lru_size_weight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogFormat {
    Json,
    Text,
}

impl Config {
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("PORT must be a valid number: {}", e))?;

        let log_format = std::env::var("LOG_FORMAT")
            .map(|s| {
                if s.eq_ignore_ascii_case("text") {
                    LogFormat::Text
                } else {
                    LogFormat::Json
                }
            })
            .unwrap_or(LogFormat::Json);

        let max_memory_bytes = std::env::var("MAX_MEMORY_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                let mut system = System::new();
                system.refresh_memory();
                let total_memory = system.total_memory();
                (total_memory as f64 * 0.9) as u64
            });

        let snapshot_interval_seconds = std::env::var("SNAPSHOT_INTERVAL_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let audit_log_path = std::env::var("AUDIT_LOG_PATH").ok();

        let snapshot_path = std::env::var("SNAPSHOT_PATH")
            .unwrap_or_else(|_| "./snapshots/raptor_snapshot.bin".to_string());

        let smart_lru_eviction_threshold = std::env::var("SMART_LRU_EVICTION_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.8);

        let smart_lru_check_interval_seconds = std::env::var("SMART_LRU_CHECK_INTERVAL_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        let smart_lru_min_eviction_ratio = std::env::var("SMART_LRU_MIN_EVICTION_RATIO")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.05);

        let smart_lru_max_eviction_ratio = std::env::var("SMART_LRU_MAX_EVICTION_RATIO")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.2);

        let smart_lru_ttl_weight = std::env::var("SMART_LRU_TTL_WEIGHT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.4);

        let smart_lru_access_weight = std::env::var("SMART_LRU_ACCESS_WEIGHT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.4);

        let smart_lru_size_weight = std::env::var("SMART_LRU_SIZE_WEIGHT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.2);

        Ok(Self {
            port,
            log_format,
            max_memory_bytes,
            snapshot_interval_seconds: Some(snapshot_interval_seconds),
            audit_log_path,
            snapshot_path: Some(snapshot_path),
            smart_lru_eviction_threshold,
            smart_lru_check_interval_seconds,
            smart_lru_min_eviction_ratio,
            smart_lru_max_eviction_ratio,
            smart_lru_ttl_weight,
            smart_lru_access_weight,
            smart_lru_size_weight,
        })
    }
}

pub fn create_test_config() -> Config {
    Config {
        port: 3000,
        log_format: LogFormat::Json,
        max_memory_bytes: 1024 * 1024,
        snapshot_interval_seconds: Some(10),
        audit_log_path: None,
        snapshot_path: Some("./test_snapshot.bin".to_string()),
        smart_lru_eviction_threshold: 0.8,
        smart_lru_check_interval_seconds: 30,
        smart_lru_min_eviction_ratio: 0.05,
        smart_lru_max_eviction_ratio: 0.2,
        smart_lru_ttl_weight: 0.4,
        smart_lru_access_weight: 0.4,
        smart_lru_size_weight: 0.2,
    }
}
