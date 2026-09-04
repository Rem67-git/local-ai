use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Memory not found: {0}")]
    NotFound(String),

    #[error("Invalid memory entry: {0}")]
    InvalidEntry(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Recall error: {0}")]
    RecallError(String),
}

pub type MemoryResult<T> = Result<T, MemoryError>;
