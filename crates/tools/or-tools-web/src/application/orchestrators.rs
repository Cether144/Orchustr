use crate::domain::contracts::{Scraper, WebBrowser};
use crate::domain::entities::{FetchRequest, FetchResponse, ScrapedPage};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use or_tools_core::{Tool, ToolCapability, ToolError, ToolInput, ToolMeta, ToolOutput};
use std::sync::Arc;

/// Validates URL safety & dispatches to the chosen browser / scraper.
#[derive(Clone)]
pub struct WebOrchestrator {
    browser: Arc<dyn WebBrowser>,
}

impl WebOrchestrator {
    #[must_use]
    pub fn new(browser: Arc<dyn WebBrowser>) -> Self {
        Self { browser }
    }

    pub async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        let span = tracing::info_span!(
            "tools.web.fetch",
            otel.name = "tools.web.fetch",
            provider = self.browser.name(),
            url = %req.url,
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        validate_url(&req.url)?;
        let result = self.browser.fetch(req).await;
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}

pub(crate) fn validate_url(raw: &str) -> Result<url::Url, WebError> {
    let parsed = url::Url::parse(raw).map_err(|_| WebError::InvalidUrl(raw.into()))?;
    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        other => Err(WebError::UnsafeScheme(other.into())),
    }
}

pub struct BrowserTool<B: WebBrowser> {
    browser: B,
}

impl<B: WebBrowser> BrowserTool<B> {
    #[must_use]
    pub fn new(browser: B) -> Self {
        Self { browser }
    }
}

#[async_trait]
impl<B: WebBrowser> Tool for BrowserTool<B> {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(format!("web.{}", self.browser.name()), "web browser tool")
            .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let req: FetchRequest = serde_json::from_value(input.payload)
            .map_err(|e| ToolError::invalid_input(&input.tool, e.to_string()))?;
        validate_url(&req.url)?;
        let resp = self.browser.fetch(req).await?;
        let payload = serde_json::to_value(&resp).map_err(|e| ToolError::Serialization {
            tool: input.tool.clone(),
            reason: e.to_string(),
        })?;
        Ok(ToolOutput::new(input.tool, payload))
    }
}

pub struct ScraperTool<S: Scraper> {
    scraper: S,
}

impl<S: Scraper> ScraperTool<S> {
    #[must_use]
    pub fn new(scraper: S) -> Self {
        Self { scraper }
    }
}

#[async_trait]
impl<S: Scraper> Tool for ScraperTool<S> {
    fn meta(&self) -> ToolMeta {
        ToolMeta::new(
            format!("web.scrape.{}", self.scraper.name()),
            "web scraper tool",
        )
        .with_capability(ToolCapability::Network)
    }

    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let url = input
            .payload
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::invalid_input(&input.tool, "missing `url`"))?;
        validate_url(url)?;
        let page: ScrapedPage = self.scraper.scrape(url).await?;
        let payload = serde_json::to_value(&page).map_err(|e| ToolError::Serialization {
            tool: input.tool.clone(),
            reason: e.to_string(),
        })?;
        Ok(ToolOutput::new(input.tool, payload))
    }
}
