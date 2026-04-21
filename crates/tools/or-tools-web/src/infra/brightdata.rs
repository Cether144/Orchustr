use super::shared::{load_credential, response_to_fetch, transport};
use crate::domain::contracts::WebBrowser;
use crate::domain::entities::{FetchRequest, FetchResponse};
use crate::domain::errors::WebError;
use async_trait::async_trait;
use serde_json::json;

const PROVIDER: &str = "brightdata";
const TOKEN_ENV: &str = "BRIGHTDATA_API_TOKEN";
const DEFAULT_URL: &str = "https://api.brightdata.com/request";

/// BrightData's Web Unlocker / Proxy API. Pass a URL, receive rendered HTML
/// through residential proxies.
#[derive(Clone)]
pub struct BrightDataScraper {
    client: reqwest::Client,
    endpoint: String,
    token: String,
    zone: String,
}

impl BrightDataScraper {
    pub fn from_env() -> Result<Self, WebError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            token: load_credential(TOKEN_ENV)?,
            zone: std::env::var("BRIGHTDATA_ZONE").unwrap_or_else(|_| "web_unlocker".into()),
        })
    }

    #[must_use]
    pub fn with_endpoint(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        token: impl Into<String>,
        zone: impl Into<String>,
    ) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
            token: token.into(),
            zone: zone.into(),
        }
    }
}

#[async_trait]
impl WebBrowser for BrightDataScraper {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn fetch(&self, req: FetchRequest) -> Result<FetchResponse, WebError> {
        let body = json!({
            "zone": self.zone,
            "url": req.url,
            "format": "raw",
            "method": req.method.as_str(),
        });
        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        response_to_fetch(PROVIDER, response).await
    }
}
