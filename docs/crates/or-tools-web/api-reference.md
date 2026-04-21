# or-tools-web API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-web/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `WebBrowser`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Async contract for fetch-style web access backends.

**Signature**
```rust
pub trait WebBrowser: Send + Sync + 'static
```

### `Scraper`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Async contract for structured scrape/extraction backends.

**Signature**
```rust
pub trait Scraper: Send + Sync + 'static
```

### `HttpMethod`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Normalized HTTP verbs used by `FetchRequest`.

**Signature**
```rust
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
```

### `FetchRequest`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Normalized browser request envelope.

**Signature**
```rust
pub struct FetchRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub render_js: bool,
    pub timeout_ms: Option<u64>,
}
```

### `FetchResponse`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Normalized fetch result.

**Signature**
```rust
pub struct FetchResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub final_url: Option<String>,
}
```

### `ScrapedPage`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Normalized scrape result.

**Signature**
```rust
pub struct ScrapedPage {
    pub url: String,
    pub title: Option<String>,
    pub text: String,
    pub links: Vec<String>,
}
```

### `WebError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: Error model covering invalid URLs, unsafe schemes, transport/upstream failures, parsing failures, unsupported methods, and missing credentials.

**Signature**
```rust
pub enum WebError { ... }
```

### `WebOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-web/src/lib.rs` |

**Description**: URL-validating fetch entry point that delegates to a configured `WebBrowser`.

**Signature**
```rust
pub struct WebOrchestrator { ... }
```

### `BrowserTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_web::application::orchestrators` |

**Description**: Generic `Tool` adapter for `WebBrowser` implementations.

**Signature**
```rust
pub struct BrowserTool<B: WebBrowser> { ... }
```

### `ScraperTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-web/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_web::application::orchestrators` |

**Description**: Generic `Tool` adapter for `Scraper` implementations.

**Signature**
```rust
pub struct ScraperTool<S: Scraper> { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `RequestsClient` | `crates/tools/or-tools-web/src/infra/http_client.rs` | `requests` | `WebBrowser` |
| `PlaywrightBrowser` | `crates/tools/or-tools-web/src/infra/playwright.rs` | `playwright` | `WebBrowser` |
| `BrightDataScraper` | `crates/tools/or-tools-web/src/infra/brightdata.rs` | `brightdata` | `WebBrowser` |
| `HyperbrowserClient` | `crates/tools/or-tools-web/src/infra/hyperbrowser.rs` | `hyperbrowser` | `WebBrowser` |
| `AgentQlScraper` | `crates/tools/or-tools-web/src/infra/agentql.rs` | `agentql` | `Scraper` |
| `OxylabsScraper` | `crates/tools/or-tools-web/src/infra/oxylabs.rs` | `oxylabs` | `WebBrowser` |

## Known Gaps & Limitations

- The public API normalizes fetch and scrape results but does not preserve provider-specific raw response payloads.
- Feature-gated backend constructors are publicly available through module paths rather than re-exported from `src/lib.rs`.
