use super::shared::{build_url, decode_response, transport};
use crate::domain::contracts::SearchProvider;
use crate::domain::entities::{SearchQuery, SearchResponse, SearchResult};
use crate::domain::errors::SearchError;
use async_trait::async_trait;
use serde::Deserialize;

const PROVIDER: &str = "searxng";

/// Self-hosted SearxNG meta-search backend. No API key required — user
/// supplies the endpoint (e.g. `https://searx.example.com`).
#[derive(Clone)]
pub struct SearxngSearch {
    client: reqwest::Client,
    endpoint: String,
}

impl SearxngSearch {
    pub fn from_env() -> Result<Self, SearchError> {
        let endpoint = std::env::var("SEARXNG_ENDPOINT")
            .map_err(|_| SearchError::MissingApiKey("SEARXNG_ENDPOINT".into()))?;
        Ok(Self {
            client: reqwest::Client::new(),
            endpoint,
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

#[derive(Debug, Deserialize)]
struct SearxngResponse {
    #[serde(default)]
    results: Vec<SearxngResult>,
}

#[derive(Debug, Deserialize)]
struct SearxngResult {
    title: String,
    url: String,
    #[serde(default)]
    content: String,
    #[serde(default, rename = "publishedDate")]
    published_date: Option<String>,
}

#[async_trait]
impl SearchProvider for SearxngSearch {
    fn name(&self) -> &'static str {
        PROVIDER
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        let base = format!("{}/search", self.endpoint.trim_end_matches('/'));
        let url = build_url(
            PROVIDER,
            &base,
            &[("q", query.query.as_str()), ("format", "json")],
        )?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| transport(PROVIDER, e))?;
        let parsed: SearxngResponse = decode_response(PROVIDER, response).await?;
        let results = parsed
            .results
            .into_iter()
            .take(query.max_results as usize)
            .map(|r| SearchResult {
                title: r.title,
                url: r.url,
                snippet: r.content,
                score: None,
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
