use super::shared::{decode_response, load_api_key, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "tavily";
const DEFAULT_URL: &str = "https://api.tavily.com/search";
const API_KEY_ENV: &str = "TAVILY_API_KEY";

#[derive(Clone)]
pub struct TavilySearch {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
}

impl TavilySearch {
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
struct TavilyResponse {
    #[serde(default)]
    results: Vec<TavilyResult>,
}

#[derive(Debug, Deserialize)]
struct TavilyResult {
    title: String,
    url: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    score: Option<f32>,
    #[serde(default)]
    published_date: Option<String>,
}

#[async_trait]
impl SearchProvider for TavilySearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let key = self
            .api_key
            .clone()
            .ok_or_else(|| SearchError::MissingApiKey(API_KEY_ENV.into()))?;
        let body = json!({
            "api_key": key,
            "query": query.query,
            "max_results": query.max_results,
            "search_depth": "basic",
        });
        let response = self
            .client
            .post(&self.endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: TavilyResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .results
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
                snippet: r.content,
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
