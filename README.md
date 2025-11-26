<h1 align="center">
  <br>
  AGENTDB
  <br>
</h1>

<h4 align="center">
  Database Abstraction Layer for AI Agents
</h4>

<p align="center">
  <a href="https://crates.io/crates/agentdb" target="_blank">
    <img src="https://img.shields.io/crates/v/agentdb" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/agentdb" target="_blank">
    <img src="https://img.shields.io/crates/d/agentdb" alt="Downloads"/>
  </a>
  <a href="https://docs.rs/agentdb" target="_blank">
    <img src="https://docs.rs/agentdb/badge.svg" alt="Documentation"/>
  </a>
  <a href="LICENSE" target="_blank">
    <img src="https://img.shields.io/github/license/cryptopatrick/agentdb.svg" alt="License"/>
  </a>
</p>

<b>Author's bio:</b> 👋😀 Hi, I'm CryptoPatrick! I'm currently enrolled as an
Undergraduate student in Mathematics, at Chalmers & the University of Gothenburg, Sweden. <br>
If you have any questions or need more info, then please <a href="https://discord.gg/T8EWmJZpCB">join my Discord Channel: AiMath</a>

---

<p align="center">
  <a href="#-what-is-agentdb">What is AgentDB</a> •
  <a href="#-features">Features</a> •
  <a href="#-architecture">Architecture</a> •
  <a href="#-how-to-use">How To Use</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-license">License</a>
</p>

## 🛎 Important Notices
* This is a **trait-based abstraction layer**, not a complete database
* Requires concrete implementations (see [agentsql](../agentsql))
* Designed for AI agent persistence needs

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-agentdb">What is AgentDB</a></li>
    <li><a href="#-features">Features</a></li>
      <ul>
        <li><a href="#-core-traits">Core Traits</a></li>
        <li><a href="#-value-system">Value System</a></li>
        <li><a href="#-error-handling">Error Handling</a></li>
      </ul>
    <li><a href="#-architecture">Architecture</a></li>
    <li><a href="#-how-to-use">How to Use</a></li>
    <li><a href="#-examples">Examples</a></li>
    <li><a href="#-testing">Testing</a></li>
    <li><a href="#-documentation">Documentation</a></li>
    <li><a href="#-author">Author</a></li>
    <li><a href="#-support">Support</a></li>
    <li><a href="#-license">License</a></li>
  </ol>
</details>

## 🤔 What is AgentDB

`agentdb` is a unified database abstraction layer that provides a common interface for AI agent storage operations across multiple backend families (SQL, Key-Value, Graph databases). It serves as the middleware between high-level agent operations and concrete database implementations.

Built to enable database flexibility without code changes, AgentDB provides a single trait that works seamlessly across SQLite, PostgreSQL, MySQL, and future backends.

### Use Cases

- **Multi-Backend Support**: Write agent code once, deploy on any supported backend
- **Database Flexibility**: Switch between SQLite, PostgreSQL, MySQL without code changes
- **Type Safety**: Rust's type system ensures correctness across database operations
- **Future-Proof**: Easy to add new database backends without changing agent code
- **Testing**: Mock implementations for unit testing agent logic
- **Custom Backends**: Implement your own backend for specialized storage needs

## 📷 Features

`agentdb` provides a complete abstraction layer for database operations with type safety and flexibility:

### 🔧 Core Traits

#### **AgentDB Trait**
- **CRUD Operations**: Put, get, delete, and scan key-value pairs
- **Query Interface**: Execute SQL queries with result sets
- **Transaction Support**: Optional transactional operations
- **Capability Discovery**: Runtime feature detection
- **Async-First**: All operations use async/await

#### **Capabilities Trait**
- **Transaction Support**: Check if transactions are available
- **Index Support**: Determine if indexes are supported
- **Backend Family**: Identify backend type (SQL, KV, Graph)
- **Feature Flags**: Query specific backend capabilities

#### **Transaction Trait**
- **ACID Guarantees**: Atomic commit and rollback operations
- **Nested Transactions**: Support for savepoints (backend-dependent)
- **Error Handling**: Comprehensive error types

### 💾 Value System
- **Type-Safe Values**: Strong typing with `Value` wrapper
- **Byte-Level Storage**: Efficient binary data handling
- **Zero-Copy Operations**: Minimize allocations where possible
- **Conversion Traits**: Easy conversion to/from Rust types

### 🔒 Error Handling
- **Unified Error Types**: Single error type across all backends
- **Context Preservation**: Detailed error messages with context
- **Result Type**: Standard Rust `Result<T, AgentDbError>`
- **Backend Errors**: Wrapped backend-specific errors

## 📐 Architecture

1. 🏛 **Overall Architecture**

```diagram
┌────────────────────────────────────────────────────┐
│              AgentFS High-Level APIs               │
│   (FileSystem, KvStore, ToolRecorder)              │
└───────────────────────┬────────────────────────────┘
                        │
┌───────────────────────▼────────────────────────────┐
│                  AgentDB Trait                     │
│  • put(key, value) → Result<()>                    │
│  • get(key) → Result<Option<Value>>                │
│  • delete(key) → Result<()>                        │
│  • scan(prefix) → Result<ScanResult>               │
│  • query(sql, params) → Result<QueryResult>        │
│  • capabilities() → &dyn Capabilities              │
└───────────────────────┬────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│   AgentSQL   │ │  AgentKV    │ │ AgentGraph │
│   (SQLx)     │ │  (Future)   │ │  (Future)  │
└───────┬──────┘ └─────────────┘ └────────────┘
        │
┌───────┼────────┬────────────┐
│       │        │            │
▼       ▼        ▼            ▼
SQLite  Postgres MySQL    MariaDB
```

2. 🔄 **Data Flow**

```diagram
┌──────────────────────────────────────────────┐
│      Application calls AgentDB::put()       │
└────────────────────┬─────────────────────────┘
                     │
            ┌────────▼────────┐
            │   AgentDB Trait │
            │   Dispatches to │
            │   Implementation│
            └────────┬────────┘
                     │
     ┌───────────────┼───────────────┐
     │               │               │
┌────▼─────┐  ┌──────▼──────┐  ┌────▼──────┐
│ SQLite   │  │ PostgreSQL  │  │   MySQL   │
│ Backend  │  │  Backend    │  │  Backend  │
└────┬─────┘  └──────┬──────┘  └────┬──────┘
     │               │               │
     └───────────────┼───────────────┘
                     │
            ┌────────▼────────┐
            │  Actual Storage │
            │   (Database)    │
            └─────────────────┘
```

3. 💾 **Trait System**

```diagram
┌────────────────────────────────────────────────┐
│              AgentDB Trait                     │
│  • Core database operations                    │
│  • Required for all implementations            │
└────────────────────┬───────────────────────────┘
                     │
     ┌───────────────┴───────────────┐
     │                               │
┌────▼──────────────┐   ┌────────────▼──────────┐
│  Capabilities     │   │    Transaction        │
│  • Runtime flags  │   │    • commit()         │
│  • Feature detect │   │    • rollback()       │
└───────────────────┘   └───────────────────────┘
```

## 🚙 How to Use

### Installation

Add `agentdb` to your `Cargo.toml`:

```toml
[dependencies]
agentdb = "0.1"
# Also add a concrete implementation:
agentsql = "0.1"
```

Or install with cargo:

```bash
cargo add agentdb
cargo add agentsql
```

### Basic Example

```rust
use agentdb::{AgentDB, Capabilities};

async fn example(db: impl AgentDB) -> Result<(), Box<dyn std::error::Error>> {
    // Check capabilities
    if db.capabilities().supports_transactions() {
        println!("✓ Transactions supported");
    }

    // Key-value operations
    db.put("user:123", b"Alice".to_vec().into()).await?;

    if let Some(value) = db.get("user:123").await? {
        println!("User: {}", String::from_utf8_lossy(value.as_bytes()));
    }

    // Scan with prefix
    let result = db.scan("user:").await?;
    println!("Found {} users", result.keys.len());

    // SQL query (if supported)
    let query_result = db.query(
        "SELECT * FROM users WHERE active = 1",
        vec![]
    ).await?;

    for row in query_result.rows {
        // Process rows...
    }

    Ok(())
}
```

### Advanced: Implementing a Custom Backend

```rust
use agentdb::{AgentDB, Capabilities, QueryResult, Row, ScanResult, Value};
use async_trait::async_trait;

struct MyCustomBackend {
    // Your backend state...
}

#[async_trait]
impl AgentDB for MyCustomBackend {
    fn capabilities(&self) -> &dyn Capabilities {
        // Return your capabilities
        unimplemented!()
    }

    async fn put(&self, key: &str, value: Value) -> agentdb::Result<()> {
        // Implement put operation
        unimplemented!()
    }

    async fn get(&self, key: &str) -> agentdb::Result<Option<Value>> {
        // Implement get operation
        unimplemented!()
    }

    async fn delete(&self, key: &str) -> agentdb::Result<()> {
        // Implement delete operation
        unimplemented!()
    }

    async fn scan(&self, prefix: &str) -> agentdb::Result<ScanResult> {
        // Implement scan operation
        unimplemented!()
    }

    async fn query(&self, sql: &str, params: Vec<Value>) -> agentdb::Result<QueryResult> {
        // Implement query operation
        unimplemented!()
    }

    // ... implement other required methods
}
```

## 🧪 Examples

See the [agentsql](../agentsql) and [agentfs](../agentfs) crates for complete examples of using AgentDB in practice.

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## 📚 Documentation

Comprehensive documentation is available at [docs.rs/agentdb](https://docs.rs/agentdb), including:
- Complete API reference for all traits and types
- Guide to implementing custom backends
- Error handling best practices
- Performance optimization tips

## 🖊 Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a>

Keybase Verification:
https://keybase.io/cryptopatrick/sigs/8epNh5h2FtIX1UNNmf8YQ-k33M8J-Md4LnAN

## 🐣 Support

Leave a ⭐ if you think this project is cool.

## 🗄 License

This project is licensed under MIT. See [LICENSE](LICENSE) for details.
