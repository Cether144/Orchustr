use super::shared::{load_credential, response_to_fetch, transport};
use crate::domain::contracts::WebBrowser;
use crate::domain::entities::{FetchRequest, FetchResponse};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use serde_json::json;

const PROVIDER: &str = "hyperbrowser";
const API_KEY_ENV: &str = "HYPERBROWSER_API_KEY";
const DEFAULT_URL: &str = "https://api.hyperbrowser.ai/v1/scrape";

/// Hyperbrowser managed headless-browser API. Accepts a URL, returns rendered
/// HTML without the user having to provision Playwright infrastructure.
#[derive(Clone)]
pub struct HyperbrowserClient {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
}

impl HyperbrowserClient {
    pub fn from_env() -> Result<Self, WebError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            api_key: load_credential(API_KEY_ENV)?,
        })
    }

    #[must_use]
    pub fn with_endpoint(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            api_key: api_key.into(),
        }
    }
}

#[async_trait]
impl WebBrowser for HyperbrowserClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        let body = json!({
            "url": req.url,
            "renderJs": req.render_js,
            "timeoutMs": req.timeout_ms,
        });
        let response = self
            .client
            .post(&self.endpoint)
            .header("x-api-key", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        response_to_fetch(PROVIDER, response).await
    }
}
