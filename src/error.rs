//! Error types for AgentDB operations

use std::fmt;

/// Result type for AgentDB operations
pub type Result<T> = std::result::Result<T, AgentDbError>;

/// Error types for AgentDB operations
#[derive(Debug)]
pub enum AgentDbError {
    /// Key not found
    NotFound(String),

    /// Operation not supported by this backend
    Unsupported(String),

    /// Transaction error
    Transaction(String),

    /// Connection error
    Connection(String),

    /// Serialization/deserialization error
    Serialization(String),

    /// Invalid operation or parameters
    InvalidOperation(String),

    /// Backend-specific error
    Backend(String),

    /// IO error
    Io(std::io::Error),

    /// Other error
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for AgentDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentDbError::NotFound(key) => write!(f, "Key not found: {}", key),
            AgentDbError::Unsupported(op) => write!(f, "Operation not supported: {}", op),
            AgentDbError::Transaction(msg) => write!(f, "Transaction error: {}", msg),
            AgentDbError::Connection(msg) => write!(f, "Connection error: {}", msg),
            AgentDbError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            AgentDbError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            AgentDbError::Backend(msg) => write!(f, "Backend error: {}", msg),
            AgentDbError::Io(err) => write!(f, "IO error: {}", err),
            AgentDbError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for AgentDbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AgentDbError::Io(err) => Some(err),
            AgentDbError::Other(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AgentDbError {
    fn from(err: std::io::Error) -> Self {
        AgentDbError::Io(err)
    }
}
