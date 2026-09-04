use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodingError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid code: {0}")]
    InvalidCode(String),

    #[error("Modification failed: {0}")]
    ModificationFailed(String),

    #[error("Syntax error at line {line}: {message}")]
    SyntaxError { line: usize, message: String },

    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

pub type CodingResult<T> = Result<T, CodingError>;
