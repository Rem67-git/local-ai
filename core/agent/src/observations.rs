use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationResult {
    pub tool: String,
    pub args: serde_json::Map<String, serde_json::Value>,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Duration,
}

impl ObservationResult {
    pub fn success(
        tool: String,
        args: serde_json::Map<String, serde_json::Value>,
        output: String,
        execution_time: Duration,
    ) -> Self {
        Self {
            tool,
            args,
            success: true,
            output,
            error: None,
            execution_time,
        }
    }

    pub fn error(
        tool: String,
        args: serde_json::Map<String, serde_json::Value>,
        error: String,
        execution_time: Duration,
    ) -> Self {
        Self {
            tool,
            args,
            success: false,
            output: String::new(),
            error: Some(error),
            execution_time,
        }
    }

    pub fn summary(&self) -> String {
        if self.success {
            format!(
                "✅ {} succeeded in {:?}: {}",
                self.tool, self.execution_time, self.output
            )
        } else {
            format!(
                "❌ {} failed in {:?}: {}",
                self.tool,
                self.execution_time,
                self.error.as_ref().unwrap_or(&"unknown error".to_string())
            )
        }
    }
}
