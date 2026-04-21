use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use std::sync::Arc;

/// Aggregates multiple [`SearchProvider`]s and runs fall-back search across
/// them until one returns a non-empty result set.
#[derive(Clone)]
pub struct SearchOrchestrator {
    providers: Vec<Arc<dyn SearchProvider>>,
}

impl SearchOrchestrator {
    #[must_use]
    pub fn new(providers: Vec<Arc<dyn SearchProvider>>) -> Self {
        Self { providers }
    }

    pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        if self.providers.is_empty() {
            return Err(SearchError::NoProviders);
        }
        if query.query.trim().is_empty() {
            return Err(SearchError::EmptyQuery);
        }
        let span = tracing::info_span!(
            "tools.search.orchestrate",
            otel.name = "tools.search.orchestrate",
            provider_count = self.providers.len(),
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let mut last_err: Option<SearchError> = None;
        for provider in &self.providers {
            match provider.search(query.clone()).await {
                Ok(response) if !response.results.is_empty() => {
                    span.record("status", "success");
                    return Ok(response);
                }
                Ok(response) => {
                    last_err = None;
                    if !self.providers.is_empty() {
                        return Ok(response);
                    }
                }
                Err(err) => last_err = Some(err),
            }
        }
        span.record("status", "failure");
        Err(last_err.unwrap_or(SearchError::NoProviders))
    }
}

/// Adapter turning a [`SearchProvider`] into an [`or_tools_core::Tool`] so it
/// can be registered alongside other tools.
pub struct SearchProviderTool<P: SearchProvider> {
    provider: P,
}

impl<P: SearchProvider> SearchProviderTool<P> {
    #[must_use]
    pub fn new(provider: P) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl<P: SearchProvider> Tool for SearchProviderTool<P> {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            format!("search.{}", self.provider.name()),
            format!("{} search provider", self.provider.name()),
        )
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let query: SearchQuery = serde_json::from_value(input.payload)
            .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
        let response = self.provider.search(query).await?;
        let payload = serde_json::to_value(&response).map_err(|e| ToolError::Serialization {
            tool: input.tool.clone(),
            reason: e.to_string(),
        })?;
        Ok(ToolOutput::new(input.tool, payload))
    }
}
