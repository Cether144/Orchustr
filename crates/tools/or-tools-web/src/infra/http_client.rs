use super::shared::{response_to_fetch, to_reqwest_method, transport};
use crate::domain::contracts::WebBrowser;
use crate::domain::entities::{FetchRequest, FetchResponse};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use std::time::Duration;

const PROVIDER: &str = "requests";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// Baseline HTTP client for API calls and simple page fetches. No JS support.
#[derive(Clone)]
pub struct RequestsClient {
    client: reqwest::Client,
}

impl Default for RequestsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestsClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
                .build()
                .unwrap_or_default(),
        }
    }

    #[must_use]
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl WebBrowser for RequestsClient {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        if req.render_js {
            return Err(WebError::MethodUnsupported(
                "requests client cannot render JS".into(),
            ));
        }
        let mut builder = self.client.request(to_reqwest_method(req.method), &req.url);
        for (k, v) in &req.headers {
            builder = builder.header(k, v);
        }
        if let Some(body) = req.body {
            builder = builder.body(body);
        }
        if let Some(ms) = req.timeout_ms {
            builder = builder.timeout(Duration::from_millis(ms));
        }
        let response = builder.send().await.map_err(|e| transport(PROVIDER, e))?;
        response_to_fetch(PROVIDER, response).await
    }
}
