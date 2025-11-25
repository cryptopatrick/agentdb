//! Core traits for AgentDB

use crate::{BackendFamily, Result, Value};
use async_trait::async_trait;
use std::collections::HashMap;

/// Main trait for database operations
///
/// Provides a unified interface for CRUD operations, transactions, queries, and scans
/// across different backend families (SQL, KV, Graph).
#[async_trait]
pub trait AgentDB: Send + Sync {
    /// Get backend family type
    fn family(&self) -> BackendFamily;

    /// Get backend capabilities
    fn capabilities(&self) -> &dyn Capabilities;

    /// Store a key-value pair
    async fn put(&self, key: &str, value: Value) -> Result<()>;

    /// Retrieve a value by key
    async fn get(&self, key: &str) -> Result<Option<Value>>;

    /// Delete a key
    async fn delete(&self, key: &str) -> Result<()>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Execute a query (backend-specific)
    async fn query(&self, query: &str, params: Vec<Value>) -> Result<QueryResult>;

    /// Scan keys with a prefix
    async fn scan(&self, prefix: &str) -> Result<ScanResult>;

    /// Begin a transaction
    async fn begin(&self) -> Result<Box<dyn Transaction>>;

    /// Close the database connection
    async fn close(&self) -> Result<()>;
}

/// Backend capability descriptor
pub trait Capabilities: Send + Sync {
    /// Does this backend support ACID transactions?
    fn supports_transactions(&self) -> bool;

    /// Does this backend support directory-like hierarchies?
    fn supports_directories(&self) -> bool;

    /// Does this backend support graph traversals?
    fn supports_graph_queries(&self) -> bool;

    /// Does this backend support structured SQL queries?
    fn supports_sql_queries(&self) -> bool;

    /// Does this backend support secondary indexes?
    fn supports_indexes(&self) -> bool;

    /// Does this backend support TTL (time-to-live) for keys?
    fn supports_ttl(&self) -> bool;

    /// Maximum key size in bytes (None = unlimited)
    fn max_key_size(&self) -> Option<usize>;

    /// Maximum value size in bytes (None = unlimited)
    fn max_value_size(&self) -> Option<usize>;
}

/// Transaction interface
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Commit the transaction
    async fn commit(self: Box<Self>) -> Result<()>;

    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> Result<()>;
}

/// Result of a query operation
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// Rows returned by the query
    pub rows: Vec<Row>,

    /// Number of rows affected (for INSERT/UPDATE/DELETE)
    pub rows_affected: usize,
}

impl QueryResult {
    /// Create a new query result
    pub fn new(rows: Vec<Row>, rows_affected: usize) -> Self {
        Self {
            rows,
            rows_affected,
        }
    }

    /// Create an empty result
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            rows_affected: 0,
        }
    }
}

/// A single row from a query result
#[derive(Debug, Clone)]
pub struct Row {
    /// Column name to value mapping
    pub columns: HashMap<String, Value>,
}

impl Row {
    /// Create a new row
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    /// Add a column to the row
    pub fn with_column(mut self, name: impl Into<String>, value: Value) -> Self {
        self.columns.insert(name.into(), value);
        self
    }

    /// Get a column value
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.columns.get(name)
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a scan operation
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Keys matching the prefix
    pub keys: Vec<String>,
}

impl ScanResult {
    /// Create a new scan result
    pub fn new(keys: Vec<String>) -> Self {
        Self { keys }
    }

    /// Create an empty scan result
    pub fn empty() -> Self {
        Self { keys: Vec::new() }
    }
}

/// Default capabilities implementation
#[derive(Debug, Clone)]
pub struct DefaultCapabilities {
    pub transactions: bool,
    pub directories: bool,
    pub graph_queries: bool,
    pub sql_queries: bool,
    pub indexes: bool,
    pub ttl: bool,
    pub max_key_size: Option<usize>,
    pub max_value_size: Option<usize>,
}

impl Capabilities for DefaultCapabilities {
    fn supports_transactions(&self) -> bool {
        self.transactions
    }

    fn supports_directories(&self) -> bool {
        self.directories
    }

    fn supports_graph_queries(&self) -> bool {
        self.graph_queries
    }

    fn supports_sql_queries(&self) -> bool {
        self.sql_queries
    }

    fn supports_indexes(&self) -> bool {
        self.indexes
    }

    fn supports_ttl(&self) -> bool {
        self.ttl
    }

    fn max_key_size(&self) -> Option<usize> {
        self.max_key_size
    }

    fn max_value_size(&self) -> Option<usize> {
        self.max_value_size
    }
}

impl Default for DefaultCapabilities {
    fn default() -> Self {
        Self {
            transactions: false,
            directories: false,
            graph_queries: false,
            sql_queries: false,
            indexes: false,
            ttl: false,
            max_key_size: None,
            max_value_size: None,
        }
    }
}
