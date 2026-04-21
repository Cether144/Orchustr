use super::shared::{build_url, decode_response, load_api_key, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;

const PROVIDER: &str = "brave";
const DEFAULT_URL: &str = "https://api.search.brave.com/res/v1/web/search";
const API_KEY_ENV: &str = "BRAVE_SEARCH_API_KEY";

#[derive(Clone)]
pub struct BraveSearch {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
}

impl BraveSearch {
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
struct BraveResponse {
    #[serde(default)]
    web: Option<BraveWeb>,
}

#[derive(Debug, Deserialize)]
struct BraveWeb {
    #[serde(default)]
    results: Vec<BraveResult>,
}

#[derive(Debug, Deserialize)]
struct BraveResult {
    title: String,
    url: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    age: Option<String>,
}

#[async_trait]
impl SearchProvider for BraveSearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let key = self
            .api_key
            .clone()
            .ok_or_else(|| SearchError::MissingApiKey(API_KEY_ENV.into()))?;
        let count = query.max_results.to_string();
        let url = build_url(
            PROVIDER,
            &self.endpoint,
            &[("q", query.query.as_str()), ("count", count.as_str())],
        )?;
        let response = self
            .client
            .get(url)
            .header("X-Subscription-Token", key)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: BraveResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .web
            .map(|w| w.results)
            .unwrap_or_default()
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
                snippet: r.description,
                score: None,
                published_at: r.age,
            })
            .collect();
        Ok(SearchResponse {
            provider: PROVIDER.into(),
            query: query.query,
            results,
        })
    }
}
