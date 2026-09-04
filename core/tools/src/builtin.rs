use crate::registry::ToolRegistry;
use crate::tool::{Tool, ToolInput, ToolOutput, ToolResult};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file"
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Path to file"}
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput> {
        let path = input
            .args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::tool::ToolError::InvalidArguments("Missing path".to_string()))?;

        Ok(ToolOutput::success(format!("File contents of {}", path)))
    }
}

struct FileWriteTool;

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Write contents to a file"
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "contents": {"type": "string"}
            },
            "required": ["path", "contents"]
        })
    }

    async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput> {
        let path = input
            .args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::tool::ToolError::InvalidArguments("Missing path".to_string()))?;

        Ok(ToolOutput::success(format!("File written to {}", path)))
    }
}

struct FileListTool;

#[async_trait]
impl Tool for FileListTool {
    fn name(&self) -> &str {
        "file_list"
    }

    fn description(&self) -> &str {
        "List directory contents"
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"}
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput> {
        let path = input
            .args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::tool::ToolError::InvalidArguments("Missing path".to_string()))?;

        Ok(ToolOutput::success(format!("Directory listing for {}", path)))
    }
}

struct ShellExecTool;

#[async_trait]
impl Tool for ShellExecTool {
    fn name(&self) -> &str {
        "shell_exec"
    }

    fn description(&self) -> &str {
        "Execute a shell command"
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string"}
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput> {
        let command = input
            .args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::tool::ToolError::InvalidArguments("Missing command".to_string()))?;

        Ok(ToolOutput::success(format!("Executed: {}", command)))
    }
}

pub fn register_builtin_tools(registry: &mut ToolRegistry) {
    registry.register(Arc::new(FileReadTool));
    registry.register(Arc::new(FileWriteTool));
    registry.register(Arc::new(FileListTool));
    registry.register(Arc::new(ShellExecTool));
}
