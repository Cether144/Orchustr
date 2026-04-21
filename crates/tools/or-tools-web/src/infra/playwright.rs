use super::shared::{load_credential, response_to_fetch, transport};
use crate::domain::contracts::WebBrowser;
use crate::domain::entities::{FetchRequest, FetchResponse};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use serde_json::json;

const PROVIDER: &str = "playwright";
const ENDPOINT_ENV: &str = "PLAYWRIGHT_ENDPOINT";

/// Talks to a Playwright browser service over HTTP (e.g. Browserless, a
/// locally-hosted playwright-server, or a custom microservice). The endpoint
/// is expected to accept a POST with `{url, wait_until, timeout_ms}` and
/// return `{status, body, final_url, headers}`.
#[derive(Clone)]
pub struct PlaywrightBrowser {
    client: reqwest::Client,
    endpoint: String,
}

impl PlaywrightBrowser {
    pub fn from_env() -> Result<Self, WebError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: load_credential(ENDPOINT_ENV)?,
        })
    }

    #[must_use]
    pub fn with_endpoint(client: reqwest::Client, endpoint: impl Into<String>) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
        }
    }
}

#[async_trait]
impl WebBrowser for PlaywrightBrowser {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        let payload = json!({
            "url": req.url,
            "method": req.method.as_str(),
            "headers": req.headers,
            "body": req.body,
            "timeout_ms": req.timeout_ms,
            "render_js": req.render_js,
        });
        let response = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        response_to_fetch(PROVIDER, response).await
    }
}
