//! # AgentDB - Database Abstraction Layer for AI Agents
//!
//! AgentDB provides a unified interface for agent storage operations across
//! multiple backend families (SQL, KV, Graph). It serves as the middleware
//! between high-level agent operations (AgentFS) and concrete database implementations.
//!
//! ## Architecture
//!
//! ```text
//! AgentFS → AgentDB → (AgentSQL | AgentKV | AgentGraph) → Concrete Backends
//! ```
//!
//! ## Core Traits
//!
//! - [`AgentDB`]: Main trait for database operations (CRUD, transactions, queries)
//! - [`Capabilities`]: Describes backend capabilities (transactions, indexes, etc.)
//! - [`Transaction`]: Transaction management interface
//!
//! ## Example
//!
//! ```rust,ignore
//! use agentdb::{AgentDB, Capabilities};
//!
//! async fn example(db: impl AgentDB) -> Result<(), Box<dyn std::error::Error>> {
//!     // Check capabilities
//!     if db.capabilities().supports_transactions() {
//!         let tx = db.begin().await?;
//!         db.put("key", b"value").await?;
//!         tx.commit().await?;
//!     }
//!     Ok(())
//! }
//! ```

use std::fmt;

pub mod error;
pub mod traits;

pub use error::{AgentDbError, Result};
pub use traits::{AgentDB, Capabilities, DefaultCapabilities, Row, Transaction, QueryResult, ScanResult};

/// Backend family identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendFamily {
    /// SQL-based backends (SQLite, PostgreSQL, MySQL)
    Sql,
    /// Key-value backends (Redis, KeyDB, FoundationDB)
    Kv,
    /// Graph backends (Neo4j, Dgraph, TigerGraph)
    Graph,
}

impl fmt::Display for BackendFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendFamily::Sql => write!(f, "SQL"),
            BackendFamily::Kv => write!(f, "KeyValue"),
            BackendFamily::Graph => write!(f, "Graph"),
        }
    }
}

/// Value type for database operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(Vec<u8>);

impl Value {
    /// Create a new value from bytes
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Create a value from a byte slice
    pub fn from_slice(data: &[u8]) -> Self {
        Self(data.to_vec())
    }

    /// Get the value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Convert value into bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Value {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<&[u8]> for Value {
    fn from(data: &[u8]) -> Self {
        Self(data.to_vec())
    }
}

impl AsRef<[u8]> for Value {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
