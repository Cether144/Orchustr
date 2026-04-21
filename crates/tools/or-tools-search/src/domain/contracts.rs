use crate::domain::entities::{SearchQuery, SearchResponse};
use crate::domain::errors::SearchError;
use async_trait::async_trait;

/// Contract implemented by every search backend (Tavily, Exa, Brave, ...).
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait SearchProvider: Send + Sync + 'static {
    /// Canonical provider identifier (e.g. `"tavily"`).
    fn name(&self) -> &'static str;

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError>;
}
