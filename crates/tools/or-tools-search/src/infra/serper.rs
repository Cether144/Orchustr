use super::shared::{decode_response, load_api_key, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

const PROVIDER: &str = "serper";
const DEFAULT_URL: &str = "https://google.serper.dev/search";
const API_KEY_ENV: &str = "SERPER_API_KEY";

#[derive(Clone)]
pub struct SerperSearch {
    client: reqwest::Client,
    endpoint: String,
    api_key: Option<String>,
}

impl SerperSearch {
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
struct SerperResponse {
    #[serde(default)]
    organic: Vec<SerperOrganic>,
}

#[derive(Debug, Deserialize)]
struct SerperOrganic {
    title: String,
    link: String,
    #[serde(default)]
    snippet: String,
    #[serde(default)]
    date: Option<String>,
}

#[async_trait]
impl SearchProvider for SerperSearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let key = self
            .api_key
            .clone()
            .ok_or_else(|| SearchError::MissingApiKey(API_KEY_ENV.into()))?;
        let body = json!({
            "q": query.query,
            "num": query.max_results,
        });
        let response = self
            .client
            .post(&self.endpoint)
            .header("X-API-KEY", key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: SerperResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .organic
            .into_iter()
            .map(|r| SearchResult {
                title: r.title,
                url: r.link,
                snippet: r.snippet,
                score: None,
                published_at: r.date,
            })
            .collect();
        Ok(SearchResponse {
            provider: PROVIDER.into(),
            query: query.query,
            results,
        })
    }
}
