use crate::actions::StructuredAction;
use crate::observations::ObservationResult;
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;

pub struct Executor {
    available_tools: Vec<String>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            available_tools: vec![
                "file_read".to_string(),
                "file_write".to_string(),
                "file_list".to_string(),
                "shell_exec".to_string(),
                "code_parse".to_string(),
            ],
        }
    }

    pub fn available_tools(&self) -> &[String] {
        &self.available_tools
    }

    pub async fn execute(&self, action: &StructuredAction) -> ExecutorResult<ObservationResult> {
        let start = Instant::now();

        if !self.available_tools.contains(&action.tool) {
            let elapsed = start.elapsed();
            return Ok(ObservationResult::error(
                action.tool.clone(),
                action.args.clone(),
                format!("Tool not found: {}", action.tool),
                elapsed,
            ));
        }

        let result = self.execute_tool(&action.tool, &action.args).await;
        let elapsed = start.elapsed();

        match result {
            Ok(output) => Ok(ObservationResult::success(
                action.tool.clone(),
                action.args.clone(),
                output,
                elapsed,
            )),
            Err(e) => Ok(ObservationResult::error(
                action.tool.clone(),
                action.args.clone(),
                e.to_string(),
                elapsed,
            )),
        }
    }

    async fn execute_tool(
        &self,
        tool: &str,
        _args: &serde_json::Map<String, serde_json::Value>,
    ) -> ExecutorResult<String> {
        match tool {
            "file_read" => Ok("File contents (mock)".to_string()),
            "file_write" => Ok("File written (mock)".to_string()),
            "file_list" => Ok("Directory listing (mock)".to_string()),
            "shell_exec" => Ok("Command output (mock)".to_string()),
            "code_parse" => Ok("Parse result (mock)".to_string()),
            _ => Err(ExecutorError::ToolNotFound(tool.to_string())),
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_available_tools() {
        let executor = Executor::new();
        assert_eq!(executor.available_tools().len(), 5);
        assert!(executor.available_tools().contains(&"file_read".to_string()));
    }

    #[tokio::test]
    async fn test_executor_tool_not_found() {
        let executor = Executor::new();
        let action = StructuredAction {
            action_type: "tool_call".to_string(),
            tool: "nonexistent_tool".to_string(),
            args: serde_json::Map::new(),
            reasoning: None,
        };

        let result = executor.execute(&action).await;
        match result {
            Ok(obs) => assert!(!obs.success),
            Err(_) => panic!("Should return ObservationResult with error"),
        }
    }
}
