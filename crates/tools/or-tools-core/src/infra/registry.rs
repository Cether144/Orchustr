use crate::domain::contracts::{Tool, ToolRegistry};
use crate::domain::entities::ToolMeta;
use crate::domain::errors::ToolError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Thread-safe in-memory [`ToolRegistry`]. Suitable for tests & single-process
/// deployments. For distributed setups, implement a remote registry backed by
/// a shared store (Redis, etcd, etc.).
#[derive(Default, Clone)]
pub struct InMemoryToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}

impl InMemoryToolRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ToolRegistry for InMemoryToolRegistry {
    async fn register(&self, tool: Arc<dyn Tool>) -> Result<(), ToolError> {
        let name = tool.meta().name;
        let mut guard = self.tools.write().await;
        if guard.contains_key(&name) {
            return Err(ToolError::AlreadyRegistered { name });
        }
        guard.insert(name, tool);
        Ok(())
    }

    async fn get(&self, name: &str) -> Result<Arc<dyn Tool>, ToolError> {
        self.tools
            .read()
            .await
            .get(name)
            .cloned()
            .ok_or_else(|| ToolError::NotFound(name.to_owned()))
    }

    async fn list(&self) -> Vec<ToolMeta> {
        self.tools
            .read()
            .await
            .values()
            .map(|tool| tool.meta())
            .collect()
    }
}
