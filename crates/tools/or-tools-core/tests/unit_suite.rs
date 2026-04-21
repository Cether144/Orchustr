use async_trait::async_trait;
use or_tools_core::{
    InMemoryToolRegistry, Tool, ToolCapability, ToolDispatcher, ToolError, ToolInput, ToolMeta,
    ToolOutput, ToolRegistry,
};
use serde_json::json;
use std::sync::Arc;

struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("echo", "echoes payload").with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        Ok(ToolOutput::new(input.tool, input.payload))
    }
}

struct FailingTool;

#[async_trait]
impl Tool for FailingTool {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new("fail", "always fails")
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        Err(ToolError::invalid_input(input.tool, "always fails"))
    }
}

#[tokio::test]
async fn registry_registers_and_retrieves_tool() {
    let registry = InMemoryToolRegistry::new();
    registry.register(Arc::new(EchoTool)).await.unwrap();
    let tool = registry.get("echo").await.unwrap();
    assert_eq!(tool.meta().name, "echo");
}

#[tokio::test]
async fn registry_rejects_duplicate_registration() {
    let registry = InMemoryToolRegistry::new();
    registry.register(Arc::new(EchoTool)).await.unwrap();
    let result = registry.register(Arc::new(EchoTool)).await;
    assert!(matches!(result, Err(ToolError::AlreadyRegistered { .. })));
}

#[tokio::test]
async fn registry_returns_not_found_for_unknown_tool() {
    let registry = InMemoryToolRegistry::new();
    let result = registry.get("missing").await;
    assert!(matches!(result, Err(ToolError::NotFound(_))));
}

#[tokio::test]
async fn dispatcher_invokes_registered_tool() {
    let registry = Arc::new(InMemoryToolRegistry::new());
    registry.register(Arc::new(EchoTool)).await.unwrap();
    let dispatcher = ToolDispatcher::new(registry);
    let output = dispatcher
        .dispatch(ToolInput::new("echo", json!({"hello": "world"})))
        .await
        .unwrap();
    assert_eq!(output.payload, json!({"hello": "world"}));
}

#[tokio::test]
async fn dispatcher_propagates_tool_failure() {
    let registry = Arc::new(InMemoryToolRegistry::new());
    registry.register(Arc::new(FailingTool)).await.unwrap();
    let dispatcher = ToolDispatcher::new(registry);
    let err = dispatcher
        .dispatch(ToolInput::new("fail", json!({})))
        .await
        .unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput { .. }));
}

#[tokio::test]
async fn registry_lists_tool_meta() {
    let registry = InMemoryToolRegistry::new();
    registry.register(Arc::new(EchoTool)).await.unwrap();
    let meta = registry.list().await;
    assert_eq!(meta.len(), 1);
    assert_eq!(meta[0].name, "echo");
}
