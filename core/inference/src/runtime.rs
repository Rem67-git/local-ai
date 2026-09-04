use thiserror::Error;

#[derive(Debug, Clone)]
pub struct InferenceOptions {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.95,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HealthStatus {
    Ok,
    Warning(String),
    Error(String),
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Initialization error: {0}")]
    InitError(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub trait LLMRuntime: Send + Sync {
    fn initialize(&mut self) -> RuntimeResult<()>;
    fn infer(&self, prompt: &str, opts: InferenceOptions) -> RuntimeResult<String>;
    fn health_check(&self) -> HealthStatus;
    fn name(&self) -> &str;
}
