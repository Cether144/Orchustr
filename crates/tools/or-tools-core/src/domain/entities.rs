use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Capabilities a tool may declare for dispatch & policy decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolCapability {
    Network,
    Filesystem,
    Subprocess,
    Database,
    Vector,
    Auth,
    Streaming,
}

/// Declarative metadata describing a registered [`Tool`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolMeta {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub capabilities: Vec<ToolCapability>,
    #[serde(default)]
    pub input_schema: Option<Value>,
    #[serde(default)]
    pub output_schema: Option<Value>,
}

impl ToolMeta {
    #[must_use]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            capabilities: Vec::new(),
            input_schema: None,
            output_schema: None,
        }
    }

    #[must_use]
    pub fn with_capability(mut self, cap: ToolCapability) -> Self {
        self.capabilities.push(cap);
        self
    }
}

/// Envelope for tool invocation inputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolInput {
    pub tool: String,
    pub payload: Value,
}

impl ToolInput {
    #[must_use]
    pub fn new(tool: impl Into<String>, payload: Value) -> Self {
        Self {
            tool: tool.into(),
            payload,
        }
    }
}

/// Envelope for tool invocation results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolOutput {
    pub tool: String,
    pub payload: Value,
    #[serde(default)]
    pub duration_ms: u64,
}

impl ToolOutput {
    #[must_use]
    pub fn new(tool: impl Into<String>, payload: Value) -> Self {
        Self {
            tool: tool.into(),
            payload,
            duration_ms: 0,
        }
    }
}
