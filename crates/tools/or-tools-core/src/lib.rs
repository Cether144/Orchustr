//! Shared tool abstractions for the Orchustr tool ecosystem.
//!
//! All `or-tools-*` crates depend on this crate for the [`Tool`] trait,
//! registry, dispatcher, and error types.

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::ToolDispatcher;
pub use domain::contracts::{Tool, ToolRegistry};
pub use domain::entities::{ToolCapability, ToolInput, ToolMeta, ToolOutput};
pub use domain::errors::ToolError;
pub use infra::registry::InMemoryToolRegistry;
