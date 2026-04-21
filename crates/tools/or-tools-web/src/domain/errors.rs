use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum WebError {
    #[error("missing credential `{0}`")]
    MissingCredential(String),

    #[error("invalid URL `{0}`")]
    InvalidUrl(String),

    #[error("unsafe URL scheme `{0}` (only http/https allowed)")]
    UnsafeScheme(String),

    #[error("upstream `{provider}` returned {status}: {body}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },

    #[error("transport error for `{provider}`: {reason}")]
    Transport { provider: String, reason: String },

    #[error("failed to parse HTML: {0}")]
    HtmlParse(String),

    #[error("request timed out after {0}ms")]
    Timeout(u64),

    #[error("method `{0}` not supported by this provider")]
    MethodUnsupported(String),
}

impl From<WebError> for ToolError {
    fn from(err: WebError) -> Self {
        match err {
            WebError::MissingCredential(env_var) => ToolError::MissingCredential {
                tool: "web".into(),
                env_var,
            },
            WebError::InvalidUrl(u) => ToolError::invalid_input("web", format!("invalid url: {u}")),
            WebError::UnsafeScheme(s) => {
                ToolError::invalid_input("web", format!("unsafe scheme: {s}"))
            }
            WebError::Upstream {
                provider,
                status,
                body,
            } => ToolError::Upstream {
                tool: provider,
                status,
                body,
            },
            WebError::Transport { provider, reason } => ToolError::Transport {
                tool: provider,
                reason,
            },
            WebError::HtmlParse(reason) => ToolError::Serialization {
                tool: "web".into(),
                reason,
            },
            WebError::Timeout(ms) => ToolError::Timeout {
                tool: "web".into(),
                timeout_ms: ms,
            },
            WebError::MethodUnsupported(m) => ToolError::invalid_input("web", m),
        }
    }
}
