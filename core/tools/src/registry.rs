use crate::tool::{Tool, ToolInput, ToolMetadata, ToolError, ToolResult, ToolOutput};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
    metadata: HashMap<String, ToolMetadata>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.name().to_string();
        let metadata = ToolMetadata::new(
            tool.name(),
            tool.description(),
            tool.input_schema(),
        );
        self.tools.insert(name.clone(), tool);
        self.metadata.insert(name, metadata);
    }

    pub fn list_tools(&self) -> Vec<ToolMetadata> {
        self.metadata.values().cloned().collect()
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub async fn execute(&self, input: &ToolInput) -> ToolResult<ToolOutput> {
        let tool = self
            .tools
            .get(&input.tool_name)
            .ok_or_else(|| ToolError::NotFound(input.tool_name.clone()))?;

        tool.execute(input).await
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockTool;

    #[async_trait::async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            "mock_tool"
        }

        fn description(&self) -> &str {
            "A mock tool for testing"
        }

        fn input_schema(&self) -> serde_json::Value {
            json!({
                "type": "object",
                "properties": {
                    "arg1": {"type": "string"}
                }
            })
        }

        async fn execute(&self, _input: &ToolInput) -> ToolResult<ToolOutput> {
            Ok(ToolOutput::success("Mock execution".to_string()))
        }
    }

    #[test]
    fn test_registry_register() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool));

        assert_eq!(registry.list_tools().len(), 1);
        assert!(registry.get_tool("mock_tool").is_some());
    }

    #[test]
    fn test_registry_list_tools() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool));

        let tools = registry.list_tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "mock_tool");
    }

    #[tokio::test]
    async fn test_registry_execute() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool));

        let input = ToolInput {
            tool_name: "mock_tool".to_string(),
            args: serde_json::Map::new(),
        };

        let output = registry.execute(&input).await.unwrap();
        assert!(output.success);
    }

    #[tokio::test]
    async fn test_registry_execute_not_found() {
        let registry = ToolRegistry::new();

        let input = ToolInput {
            tool_name: "nonexistent".to_string(),
            args: serde_json::Map::new(),
        };

        let result = registry.execute(&input).await;
        assert!(result.is_err());
    }
}
