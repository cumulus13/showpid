//! Error types for showpid

use std::fmt;

/// Custom error type for showpid operations
#[derive(Debug)]
pub enum ShowpidError {
    /// No window found for the given PID
    NoWindowFound { pid: u32, attempts: u32, message: String },

    /// Platform-specific error
    PlatformError { platform: String, message: String },

    /// Invalid configuration
    InvalidConfig { message: String },

    /// I/O error
    IoError(std::io::Error),

    /// Generic error
    Other(String),
}

/// Convenience Result type
pub type Result<T> = std::result::Result<T, ShowpidError>;

impl fmt::Display for ShowpidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowpidError::NoWindowFound { pid, attempts, message } => {
                write!(f, "No window found for PID {} after {} attempts: {}", pid, attempts, message)
            }
            ShowpidError::PlatformError { platform, message } => {
                write!(f, "Platform error on {}: {}", platform, message)
            }
            ShowpidError::InvalidConfig { message } => {
                write!(f, "Invalid configuration: {}", message)
            }
            ShowpidError::IoError(err) => {
                write!(f, "I/O error: {}", err)
            }
            ShowpidError::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for ShowpidError {}

impl From<std::io::Error> for ShowpidError {
    fn from(err: std::io::Error) -> Self {
        ShowpidError::IoError(err)
    }
}

impl From<String> for ShowpidError {
    fn from(msg: String) -> Self {
        ShowpidError::Other(msg)
    }
}

impl From<&str> for ShowpidError {
    fn from(msg: &str) -> Self {
        ShowpidError::Other(msg.to_string())
    }
}
