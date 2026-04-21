//! Search-provider tool implementations.
//!
//! Each provider is behind a feature flag — `tavily`, `exa`, `brave`,
//! `serper`, `searxng`, `youcom`, `bing` — so users only compile what they
//! need. All providers implement [`SearchProvider`] and the generic
//! [`or_tools_core::Tool`] trait via [`SearchProviderTool`].

pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::{SearchOrchestrator, SearchProviderTool};
pub use domain::contracts::SearchProvider;
pub use domain::entities::{SearchQuery, SearchResponse, SearchResult};
pub use domain::errors::SearchError;
