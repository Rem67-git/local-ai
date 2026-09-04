use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Tool not found: {0}")]
    NotFound(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Timeout")]
    Timeout,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub tool_name: String,
    pub args: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub success: bool,
    pub output: String,
    pub metadata: Option<serde_json::Map<String, Value>>,
}

impl ToolOutput {
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            metadata: None,
        }
    }

    pub fn error(output: String) -> Self {
        Self {
            success: false,
            output,
            metadata: None,
        }
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

impl ToolMetadata {
    pub fn new(name: &str, description: &str, schema: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: schema,
        }
    }
}
