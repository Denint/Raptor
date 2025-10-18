# Raptor

**High-performance in-memory key-value store written in Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.89+-000000.svg)](https://www.rust-lang.org/)

Raptor is a high-performance in-memory data structure store written in Rust. It supports all major data types with a focus on performance, reliability, and modern software architecture.

## 🚀 Features

### Data Structures
- **Strings** - Basic string values with binary safety
- **Hashes** - Field-value maps within keys
- **Lists** - Ordered collections with push/pop operations
- **Sets** - Unordered unique collections
- **Sorted Sets** - Ordered sets with scores
- **TTL (Time-To-Live)** - Automatic key expiration
- **Arrays** - Dynamic arrays with indexing operations

### Performance & Reliability
- **High Performance** - Optimized for concurrent workloads using DashMap
- **Smart LRU Cache** - Intelligent memory management with TTL-aware eviction and score-based prioritization
- **Snapshots** - Persistent storage with configurable intervals
- **Graceful Shutdown** - Clean termination with final snapshot saving
- **Comprehensive Benchmarks** - Extensive performance testing suite

### Developer Experience
- **Clean Architecture** - Hexagonal architecture with clear separation of concerns
- **Async/Await** - Modern asynchronous programming
- **Comprehensive Testing** - Unit, integration, and E2E tests
- **Docker Support** - Ready-to-deploy containerization

# Startup in Docker
```bash
# Build and run with Docker
docker-compose up --build

# Or run directly
docker run -p 3000:3000 raptor
```

## 🔧 Configuration

Raptor can be configured via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | HTTP server port |
| `LOG_FORMAT` | `json` | Log format (json/text) |
| `MAX_MEMORY_BYTES` | `System RAM * 0.9` | Memory limit for LRU |
| `SNAPSHOT_INTERVAL_SECONDS` | `10` | Snapshot interval |
| `SMART_LRU_EVICTION_THRESHOLD` | `0.8` | Memory threshold for smart eviction (0.1-0.95) |
| `SMART_LRU_CHECK_INTERVAL_SECONDS` | `30` | Interval between memory checks |
| `SMART_LRU_MIN_EVICTION_RATIO` | `0.05` | Min entries to evict (0.01-0.5) |
| `SMART_LRU_MAX_EVICTION_RATIO` | `0.2` | Max entries to evict (0.05-0.8) |
| `SMART_LRU_TTL_WEIGHT` | `0.4` | TTL priority weight (0.0-1.0) |
| `SMART_LRU_ACCESS_WEIGHT` | `0.4` | Access time priority weight (0.0-1.0) |
| `SMART_LRU_SIZE_WEIGHT` | `0.2` | Size priority weight (0.0-1.0) |
| `FEATURE_INCR_COMMAND` | `true` | Enable INCR command |

## 🎯 Usage Examples

### Basic Operations
```bash
# Set a key
curl -X POST "http://localhost:3000/single_key/set/mykey" \
  -H "Content-Type: application/json" \
  -d '{"value": {"type": "String", "value": "SGVsbG8gV29ybGQ="}}'

# Get a key
curl "http://localhost:3000/single_key/get/mykey"

# Delete a key
curl -X DELETE "http://localhost:3000/single_key/delete/mykey"
```

### Hash Operations
```bash
# Set hash field
curl -X POST "http://localhost:3000/hash/hset/myhash/myfield" \
  -H "Content-Type: application/octet-stream" \
  -d "SGVsbG8="

# Get hash field
curl "http://localhost:3000/hash/hget/myhash/myfield"
```

### List Operations
```bash
# Push to list
curl -X POST "http://localhost:3000/list/lpush/mylist" \
  -H "Content-Type: application/json" \
  -d '{"values": ["aXRlbTE=", "aXRlbTI="]}'

# Get list range
curl -X POST "http://localhost:3000/list/lrange/mylist" \
  -H "Content-Type: application/json" \
  -d '{"start": 0, "stop": -1}'
```

### TTL Operations
```bash
# Set with TTL
curl -X POST "http://localhost:3000/single_key/set/key_with_ttl" \
  -H "Content-Type: application/json" \
  -d '{"value": {"type": "String", "value": "RXhwaXJpbmc="}, "ttl_seconds": 300}'

# Check TTL
curl "http://localhost:3000/single_key/ttl/key_with_ttl"

# Remove TTL
curl -X POST "http://localhost:3000/single_key/persist/key_with_ttl"
```

## 📚 Data Types & Base64 Encoding

**Important:** All binary data must be base64-encoded in JSON requests. The API returns base64-encoded data.

### Supported Types
- `String` - Binary-safe strings
- `Integer` - 64-bit integers (returned as base64-encoded strings)
- `Hash` - Field-value maps
- `List` - Ordered arrays
- `Set` - Unique value sets
- `SortedSet` - Score-sorted sets
- `Array` - Dynamic arrays

## 🧪 Testing & Benchmarks

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test integration    # Integration tests
cargo test --test e2e           # End-to-end tests

# Run benchmarks
cargo bench

# Run tests
cargo test

# Run with custom config
PORT=8080 LOG_FORMAT=text cargo run
``` 

## 📈 Performance

Raptor is optimized for high-performance scenarios:

- **Concurrent Access**: Lock-free operations with DashMap
- **Smart LRU Caching**: Intelligent TTL-aware memory management with score-based eviction
- **Snapshot Optimization**: Efficient serialization
- **Sub-millisecond responses**: For basic operations

## 📄 License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
