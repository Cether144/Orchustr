# or-tools-web Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: browser and scraper contracts, fetch/scrape entities, and crate-local errors.
- `application/`: URL validation plus `Tool` adapters for browser and scraper backends.
- `infra/`: feature-gated browser and scraper implementations plus shared HTTP helpers.
- `tests/`: unit coverage for URL validation and tool-adapter behavior.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `requests` | `infra/http_client.rs` | Baseline raw HTTP fetch client. |
| `playwright` | `infra/playwright.rs` | HTTP client for a remote Playwright-compatible service. |
| `brightdata` | `infra/brightdata.rs` | BrightData Web Unlocker browser-style fetch backend. |
| `hyperbrowser` | `infra/hyperbrowser.rs` | Managed browser fetch backend. |
| `agentql` | `infra/agentql.rs` | Natural-language scraper backend. |
| `oxylabs` | `infra/oxylabs.rs` | Oxylabs realtime scraper backend. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-web/Cargo.toml` | toml | 33 | Package manifest, default features, and dependencies. |
| `crates/tools/or-tools-web/src/lib.rs` | rs | 10 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-web/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-web/src/application/orchestrators.rs` | rs | 110 | Implements `WebOrchestrator`, `BrowserTool`, and `ScraperTool`. |
| `crates/tools/or-tools-web/src/domain/contracts.rs` | rs | 17 | Defines `WebBrowser` and `Scraper`. |
| `crates/tools/or-tools-web/src/domain/entities.rs` | rs | 77 | Defines HTTP method, fetch, and scrape entities. |
| `crates/tools/or-tools-web/src/domain/errors.rs` | rs | 71 | Defines `WebError` and conversion to `ToolError`. |
| `crates/tools/or-tools-web/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-web/src/infra/agentql.rs` | rs | 83 | Feature-gated AgentQL scraper backend. |
| `crates/tools/or-tools-web/src/infra/brightdata.rs` | rs | 71 | Feature-gated BrightData browser backend. |
| `crates/tools/or-tools-web/src/infra/http_client.rs` | rs | 65 | Default `requests` browser backend. |
| `crates/tools/or-tools-web/src/infra/hyperbrowser.rs` | rs | 66 | Feature-gated Hyperbrowser backend. |
| `crates/tools/or-tools-web/src/infra/mod.rs` | rs | 14 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-web/src/infra/oxylabs.rs` | rs | 72 | Feature-gated Oxylabs backend. |
| `crates/tools/or-tools-web/src/infra/playwright.rs` | rs | 62 | Feature-gated Playwright backend. |
| `crates/tools/or-tools-web/src/infra/shared.rs` | rs | 60 | Shared method conversion, response mapping, and credential helpers. |
| `crates/tools/or-tools-web/tests/unit_suite.rs` | rs | 116 | Covers URL validation, fetch, scrape, and tool wrapping. |

## Test Shape

- `tests/unit_suite.rs` uses stub `WebBrowser` and `Scraper` implementations.
- The suite checks both direct orchestrator behavior and the generic `Tool` adapters.

## Known Gaps & Limitations

- Unsafe URL schemes are rejected centrally in `application/orchestrators.rs`, so backends never receive them.
- Only the `requests` backend is enabled by default; all managed-browser backends are opt-in features.
