use super::shared::{build_url, decode_response, load_api_key, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;

const PROVIDER: &str = "youcom";
const DEFAULT_URL: &str = "https://api.ydc-index.io/search";
const API_KEY_ENV: &str = "YOUCOM_API_KEY";

#[derive(Clone)]
pub struct YouComSearch {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
}

impl YouComSearch {
    pub fn from_env() -> Result<Self, SearchError> {
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint: DEFAULT_URL.to_string(),
            api_key: Some(load_api_key(API_KEY_ENV)?),
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
            api_key: Some(api_key.into()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct YouComResponse {
    #[serde(default)]
    hits: Vec<YouComHit>,
}

#[derive(Debug, Deserialize)]
struct YouComHit {
    title: String,
    url: String,
    #[serde(default)]
    description: String,
    #[serde(default, rename = "publishedDate")]
    published_date: Option<String>,
}

#[async_trait]
impl SearchProvider for YouComSearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let key = self
            .api_key
            .clone()
            .ok_or_else(|| SearchError::MissingApiKey(API_KEY_ENV.into()))?;
        let limit = query.max_results.to_string();
        let url = build_url(
            PROVIDER,
            &self.endpoint,
            &[
                ("query", query.query.as_str()),
                ("num_web_results", limit.as_str()),
            ],
        )?;
        let response = self
            .client
            .get(url)
            .header("X-API-Key", key)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: YouComResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .hits
            .into_iter()
            .map(|h| SearchResult {
                title: h.title,
                url: h.url,
                snippet: h.description,
                score: None,
                published_at: h.published_date,
            })
            .collect();
        Ok(SearchResponse {
            provider: PROVIDER.into(),
            query: query.query,
            results,
        })
    }
}
