use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchQuery {
    pub query: String,
    #[serde(default = "default_limit")]
    pub max_results: u32,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub safe_search: bool,
}

fn default_limit() -> u32 {
    10
}

impl SearchQuery {
    #[must_use]
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            max_results: default_limit(),
            language: None,
            region: None,
            safe_search: false,
        }
    }

    #[must_use]
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.max_results = limit;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    #[serde(default)]
    pub score: Option<f32>,
    #[serde(default)]
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    pub provider: String,
    pub query: String,
    pub results: Vec<SearchResult>,
}
