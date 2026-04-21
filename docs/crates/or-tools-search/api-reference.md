# or-tools-search API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-search/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `SearchProvider`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Async contract implemented by each feature-gated search backend.

**Signature**
```rust
pub trait SearchProvider: Send + Sync + 'static
```

### `SearchQuery`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Normalized search request model.

**Signature**
```rust
pub struct SearchQuery {
    pub query: String,
    pub max_results: u32,
    pub language: Option<String>,
    pub region: Option<String>,
    pub safe_search: bool,
}
```

### `SearchResult`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: One normalized search hit.

**Signature**
```rust
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub score: Option<f32>,
    pub published_at: Option<String>,
}
```

### `SearchResponse`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Normalized provider response.

**Signature**
```rust
pub struct SearchResponse {
    pub provider: String,
    pub query: String,
    pub results: Vec<SearchResult>,
}
```

### `SearchError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Search error model covering missing credentials, validation, upstream failures, transport failures, serialization failures, and the no-provider case.

**Signature**
```rust
pub enum SearchError {
    MissingApiKey(String),
    EmptyQuery,
    Upstream { provider: String, status: u16, body: String },
    Transport { provider: String, reason: String },
    Serialization { provider: String, reason: String },
    NoProviders,
}
```

### `SearchOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Fallback-oriented runtime that validates input and queries providers in order until one returns a non-empty result set.

**Signature**
```rust
pub struct SearchOrchestrator { ... }
```

### `SearchProviderTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-search/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-search/src/lib.rs` |

**Description**: Generic `Tool` adapter that wraps a single `SearchProvider`.

**Signature**
```rust
pub struct SearchProviderTool<P: SearchProvider> { ... }
```

## Feature-gated Provider Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `TavilySearch` | `crates/tools/or-tools-search/src/infra/tavily.rs` | `tavily` | `SearchProvider` |
| `ExaSearch` | `crates/tools/or-tools-search/src/infra/exa.rs` | `exa` | `SearchProvider` |
| `BraveSearch` | `crates/tools/or-tools-search/src/infra/brave.rs` | `brave` | `SearchProvider` |
| `SerperSearch` | `crates/tools/or-tools-search/src/infra/serper.rs` | `serper` | `SearchProvider` |
| `SearxngSearch` | `crates/tools/or-tools-search/src/infra/searxng.rs` | `searxng` | `SearchProvider` |
| `YouComSearch` | `crates/tools/or-tools-search/src/infra/youcom.rs` | `youcom` | `SearchProvider` |
| `BingSearch` | `crates/tools/or-tools-search/src/infra/bing.rs` | `bing` | `SearchProvider` |

## Known Gaps & Limitations

- Provider constructors and env-var loading are feature-gated and live under `infra/`; they are not re-exported from `src/lib.rs`.
- The public API normalizes search results but does not preserve provider-specific raw payloads.
