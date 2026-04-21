use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum SearchError {
    #[error("missing API key: set `{0}` environment variable")]
    MissingApiKey(String),

    #[error("empty query")]
    EmptyQuery,

    #[error("upstream `{provider}` returned {status}: {body}")]
    Upstream {
        provider: String,
        status: u16,
        body: String,
    },

    #[error("transport error for `{provider}`: {reason}")]
    Transport { provider: String, reason: String },

    #[error("serialization error for `{provider}`: {reason}")]
    Serialization { provider: String, reason: String },

    #[error("no providers registered")]
    NoProviders,
}

impl From<SearchError> for ToolError {
    fn from(err: SearchError) -> Self {
        match err {
            SearchError::MissingApiKey(env_var) => ToolError::MissingCredential {
                tool: "search".into(),
                env_var,
            },
            SearchError::EmptyQuery => ToolError::invalid_input("search", "empty query"),
            SearchError::Upstream {
                provider,
                status,
                body,
            } => ToolError::Upstream {
                tool: provider,
                status,
                body,
            },
            SearchError::Transport { provider, reason } => ToolError::Transport {
                tool: provider,
                reason,
            },
            SearchError::Serialization { provider, reason } => ToolError::Serialization {
                tool: provider,
                reason,
            },
            SearchError::NoProviders => ToolError::Unavailable {
                tool: "search".into(),
                reason: "no providers registered".into(),
            },
        }
    }
}
