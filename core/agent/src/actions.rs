use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid action type: {0}")]
    InvalidActionType(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),
}

pub type ActionResult<T> = Result<T, ActionError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredAction {
    pub action_type: String,
    pub tool: String,
    pub args: serde_json::Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

impl StructuredAction {
    pub fn from_json(json_str: &str) -> ActionResult<Self> {
        let value: Value = serde_json::from_str(json_str)?;
        Self::from_value(&value)
    }

    pub fn from_value(value: &Value) -> ActionResult<Self> {
        let obj = value
            .as_object()
            .ok_or_else(|| ActionError::ParseError("Expected object".to_string()))?;

        let action_type = obj
            .get("action_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ActionError::MissingField("action_type".to_string()))?
            .to_string();

        let tool = obj
            .get("tool")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ActionError::MissingField("tool".to_string()))?
            .to_string();

        let args = obj
            .get("args")
            .and_then(|v| v.as_object())
            .ok_or_else(|| ActionError::MissingField("args".to_string()))?
            .clone();

        let reasoning = obj.get("reasoning").and_then(|v| v.as_str()).map(String::from);

        Ok(StructuredAction {
            action_type,
            tool,
            args,
            reasoning,
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_action_from_json() {
        let json = r#"{
            "action_type": "tool_call",
            "tool": "file_read",
            "args": {"path": "/tmp/test.txt"},
            "reasoning": "Reading test file"
        }"#;

        let action = StructuredAction::from_json(json).unwrap();
        assert_eq!(action.action_type, "tool_call");
        assert_eq!(action.tool, "file_read");
        assert_eq!(action.reasoning, Some("Reading test file".to_string()));
    }

    #[test]
    fn test_structured_action_missing_field() {
        let json = r#"{"action_type": "tool_call"}"#;
        let result = StructuredAction::from_json(json);
        assert!(result.is_err());
    }
}
