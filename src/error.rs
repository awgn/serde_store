use serde::ser;
use thiserror::Error; // Import ser module from serde for Error trait

/// Represents errors that can occur during serialization or deserialization
/// using the Store format.
#[derive(Debug, Error)]
pub enum StoreError {
    /// An underlying I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A general error occurred during serialization or deserialization
    /// reported by Serde or the serializer/deserializer logic.
    #[error("Serde error: {0}")]
    Serde(String),
}

// Implement the Serde error traits for StoreError
impl ser::Error for StoreError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        StoreError::Serde(msg.to_string())
    }
}
