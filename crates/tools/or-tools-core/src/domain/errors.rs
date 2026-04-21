use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Canonical error type returned by every tool implementation.
#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum ToolError {
    #[error("tool `{0}` not found in registry")]
    NotFound(String),

    #[error("tool `{name}` already registered")]
    AlreadyRegistered { name: String },

    #[error("invalid input for tool `{tool}`: {reason}")]
    InvalidInput { tool: String, reason: String },

    #[error("transport error contacting `{tool}`: {reason}")]
    Transport { tool: String, reason: String },

    #[error("upstream returned {status} for `{tool}`: {body}")]
    Upstream {
        tool: String,
        status: u16,
        body: String,
    },

    #[error("missing credential `{env_var}` for tool `{tool}`")]
    MissingCredential { tool: String, env_var: String },

    #[error("tool `{tool}` timed out after {timeout_ms}ms")]
    Timeout { tool: String, timeout_ms: u64 },

    #[error("tool `{tool}` unavailable: {reason}")]
    Unavailable { tool: String, reason: String },

    #[error("serialization error in tool `{tool}`: {reason}")]
    Serialization { tool: String, reason: String },
}

impl ToolError {
    #[must_use]
    pub fn invalid_input(tool: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidInput {
            tool: tool.into(),
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn transport(tool: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Transport {
            tool: tool.into(),
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn missing_credential(tool: impl Into<String>, env_var: impl Into<String>) -> Self {
        Self::MissingCredential {
            tool: tool.into(),
            env_var: env_var.into(),
        }
    }
}
