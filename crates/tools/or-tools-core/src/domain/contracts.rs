use crate::domain::entities::{ToolInput, ToolMeta, ToolOutput};
use crate::domain::errors::ToolError;
use async_trait::async_trait;
use std::sync::Arc;

/// Contract implemented by every concrete tool (search, exec, vector, etc.).
///
/// Implementors are expected to be stateless or internally-synchronised so
/// they can be shared across async tasks.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Tool: Send + Sync + 'static {
    /// Declarative metadata used for discovery & schema publication.
    fn meta(&self) -> ToolMeta;

    /// Execute the tool with the supplied input envelope.
    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError>;
}

/// Registry abstraction that stores and retrieves tools by name.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ToolRegistry: Send + Sync + 'static {
    async fn register(&self, tool: Arc<dyn Tool>) -> Result<(), ToolError>;
    async fn get(&self, name: &str) -> Result<Arc<dyn Tool>, ToolError>;
    async fn list(&self) -> Vec<ToolMeta>;
}
