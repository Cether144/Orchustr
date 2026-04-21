use super::shared::{response_to_fetch, transport};
use crate::domain::contracts::WebBrowser;
use crate::domain::entities::{FetchRequest, FetchResponse};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use serde_json::json;

const PROVIDER: &str = "oxylabs";
const DEFAULT_URL: &str = "https://realtime.oxylabs.io/v1/queries";

/// Oxylabs Realtime scraper — enterprise-grade residential proxy + render.
#[derive(Clone)]
pub struct OxylabsScraper {
    client: reqwest::Client,
    endpoint: String,
    username: String,
    password: String,
}

impl OxylabsScraper {
    pub fn from_env() -> Result<Self, WebError> {
        let username = std::env::var("OXYLABS_USERNAME")
            .map_err(|_| WebError::MissingCredential("OXYLABS_USERNAME".into()))?;
        let password = std::env::var("OXYLABS_PASSWORD")
            .map_err(|_| WebError::MissingCredential("OXYLABS_PASSWORD".into()))?;
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            username,
            password,
        })
    }

    #[must_use]
    pub fn with_credentials(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}

#[async_trait]
impl WebBrowser for OxylabsScraper {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        let body = json!({
            "source": "universal",
            "url": req.url,
            "render": if req.render_js { "html" } else { "" },
        });
        let response = self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.username, Some(&self.password))
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        response_to_fetch(PROVIDER, response).await
    }
}
