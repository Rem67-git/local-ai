use thiserror::Error;

#[derive(Debug, Error)]
pub enum RAGError {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    #[error("Indexing error: {0}")]
    IndexingError(String),

    #[error("Retrieval error: {0}")]
    RetrievalError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}

pub type RAGResult<T> = Result<T, RAGError>;
