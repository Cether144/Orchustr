use super::shared::{decode_response, load_api_key, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "exa";
const DEFAULT_URL: &str = "https://api.exa.ai/search";
const API_KEY_ENV: &str = "EXA_API_KEY";

#[derive(Clone)]
pub struct ExaSearch {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
}

impl ExaSearch {
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
struct ExaResponse {
    #[serde(default)]
    results: Vec<ExaResult>,
}

#[derive(Debug, Deserialize)]
struct ExaResult {
    title: Option<String>,
    url: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    score: Option<f32>,
    #[serde(default)]
    published_date: Option<String>,
}

#[async_trait]
impl SearchProvider for ExaSearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let key = self
            .api_key
            .clone()
            .ok_or_else(|| SearchError::MissingApiKey(API_KEY_ENV.into()))?;
        let body = json!({
            "query": query.query,
            "num_results": query.max_results,
            "type": "neural",
        });
        let response = self
            .client
            .post(&self.endpoint)
            .header("x-api-key", key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: ExaResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .results
            .into_iter()
            .map(|r| SearchResult {
                title: r.title.unwrap_or_default(),
                url: r.url,
                snippet: r.text.unwrap_or_default(),
                score: r.score,
                published_at: r.published_date,
            })
            .collect();
        Ok(SearchResponse {
            provider: PROVIDER.into(),
            query: query.query,
            results,
        })
    }
}
