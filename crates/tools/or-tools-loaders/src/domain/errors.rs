use or_tools_core::ToolError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum LoaderError {
    #[error("unsupported format `{0}`")]
    UnsupportedFormat(String),

    #[error("io error for `{path}`: {reason}")]
    Io { path: String, reason: String },

    #[error("parse error: {0}")]
    Parse(String),

    #[error("invalid source: {0}")]
    InvalidSource(String),
}

impl From<LoaderError> for ToolError {
    fn from(err: LoaderError) -> Self {
        match err {
            LoaderError::UnsupportedFormat(f) => {
                ToolError::invalid_input("loader", format!("unsupported: {f}"))
            }
            LoaderError::Io { path, reason } => ToolError::Transport {
                tool: "loader".into(),
                reason: format!("{path}: {reason}"),
            },
            LoaderError::Parse(r) => ToolError::Serialization {
                tool: "loader".into(),
                reason: r,
            },
            LoaderError::InvalidSource(r) => ToolError::invalid_input("loader", r),
        }
    }
}
