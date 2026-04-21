use async_trait::async_trait;
use or_tools_core::{Tool, ToolError, ToolInput};
use or_tools_search::{
    SearchError, SearchOrchestrator, SearchProvider, SearchProviderTool, SearchQuery,
    SearchResponse, SearchResult,
};
use serde_json::json;
use std::sync::Arc;

struct StubProvider {
    name: &'static str,
    results: Vec<SearchResult>,
}

#[async_trait]
impl SearchProvider for StubProvider {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        Ok(SearchResponse {
            provider: self.name.into(),
            query: query.query,
            results: self.results.clone(),
        })
    }
}

struct ErrorProvider {
    name: &'static str,
}

#[async_trait]
impl SearchProvider for ErrorProvider {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn search(&self, _q: SearchQuery) -> Result<SearchResponse, SearchError> {
        Err(SearchError::Upstream {
            provider: self.name.into(),
            status: 500,
            body: "boom".into(),
        })
    }
}

fn sample_result() -> SearchResult {
    SearchResult {
        title: "t".into(),
        url: "https://example.com".into(),
        snippet: "s".into(),
        score: Some(0.9),
        published_at: None,
    }
}

#[tokio::test]
async fn orchestrator_rejects_empty_query() {
    let orch = SearchOrchestrator::new(vec![Arc::new(StubProvider {
        name: "stub",
        results: vec![sample_result()],
    })]);
    let res = orch.search(SearchQuery::new("   ")).await;
    assert!(matches!(res, Err(SearchError::EmptyQuery)));
}

#[tokio::test]
async fn orchestrator_rejects_no_providers() {
    let orch = SearchOrchestrator::new(vec![]);
    let res = orch.search(SearchQuery::new("hi")).await;
    assert!(matches!(res, Err(SearchError::NoProviders)));
}

#[tokio::test]
async fn orchestrator_returns_first_non_empty_response() {
    let orch = SearchOrchestrator::new(vec![Arc::new(StubProvider {
        name: "stub",
        results: vec![sample_result()],
    })]);
    let response = orch.search(SearchQuery::new("rust")).await.unwrap();
    assert_eq!(response.provider, "stub");
    assert_eq!(response.results.len(), 1);
}

#[tokio::test]
async fn orchestrator_falls_back_on_upstream_error() {
    let orch = SearchOrchestrator::new(vec![
        Arc::new(ErrorProvider { name: "bad" }),
        Arc::new(StubProvider {
            name: "good",
            results: vec![sample_result()],
        }),
    ]);
    let response = orch.search(SearchQuery::new("rust")).await.unwrap();
    assert_eq!(response.provider, "good");
}

#[tokio::test]
async fn provider_tool_wraps_search_provider() {
    let tool = SearchProviderTool::new(StubProvider {
        name: "stub",
        results: vec![sample_result()],
    });
    assert_eq!(tool.meta().name, "search.stub");
    let out = tool
        .invoke(ToolInput::new(
            "search.stub",
            json!({ "query": "rust", "max_results": 5 }),
        ))
        .await
        .unwrap();
    assert_eq!(out.payload["results"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn provider_tool_rejects_invalid_payload() {
    let tool = SearchProviderTool::new(StubProvider {
        name: "stub",
        results: vec![],
    });
    let result = tool
        .invoke(ToolInput::new("search.stub", json!({ "bad": 1 })))
        .await;
    assert!(matches!(result, Err(ToolError::InvalidInput { .. })));
}
