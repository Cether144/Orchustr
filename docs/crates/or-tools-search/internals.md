# or-tools-search Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: search contracts, normalized request/response types, and crate-local errors.
- `application/`: orchestration and `Tool` adapter logic.
- `infra/`: feature-gated provider implementations plus shared HTTP helpers.
- `tests/`: stub-driven unit coverage for orchestration and tool wrapping.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `tavily` | `infra/tavily.rs` | Tavily HTTP provider implementation. |
| `exa` | `infra/exa.rs` | Exa HTTP provider implementation. |
| `brave` | `infra/brave.rs` | Brave Search API implementation. |
| `serper` | `infra/serper.rs` | Google Serper API implementation. |
| `searxng` | `infra/searxng.rs` | SearxNG endpoint client. |
| `youcom` | `infra/youcom.rs` | You.com API implementation. |
| `bing` | `infra/bing.rs` | Bing Search API implementation. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-search/Cargo.toml` | toml | 34 | Package manifest, feature flags, and dependencies. |
| `crates/tools/or-tools-search/src/lib.rs` | rs | 15 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-search/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-search/src/application/orchestrators.rs` | rs | 89 | Implements `SearchOrchestrator` and `SearchProviderTool`. |
| `crates/tools/or-tools-search/src/domain/contracts.rs` | rs | 13 | Defines the `SearchProvider` trait. |
| `crates/tools/or-tools-search/src/domain/entities.rs` | rs | 55 | Defines normalized query, hit, and response types. |
| `crates/tools/or-tools-search/src/domain/errors.rs` | rs | 61 | Defines `SearchError` and conversion to `ToolError`. |
| `crates/tools/or-tools-search/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-search/src/infra/bing.rs` | rs | 108 | Feature-gated Bing provider backend. |
| `crates/tools/or-tools-search/src/infra/brave.rs` | rs | 109 | Feature-gated Brave provider backend. |
| `crates/tools/or-tools-search/src/infra/exa.rs` | rs | 103 | Feature-gated Exa provider backend. |
| `crates/tools/or-tools-search/src/infra/mod.rs` | rs | 16 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-search/src/infra/searxng.rs` | rs | 91 | Feature-gated SearxNG provider backend. |
| `crates/tools/or-tools-search/src/infra/serper.rs` | rs | 101 | Feature-gated Serper provider backend. |
| `crates/tools/or-tools-search/src/infra/shared.rs` | rs | 67 | Shared URL-building, response decoding, and credential helpers. |
| `crates/tools/or-tools-search/src/infra/tavily.rs` | rs | 103 | Feature-gated Tavily provider backend. |
| `crates/tools/or-tools-search/src/infra/youcom.rs` | rs | 103 | Feature-gated You.com provider backend. |
| `crates/tools/or-tools-search/tests/unit_suite.rs` | rs | 127 | Covers validation, fallback behavior, and tool wrapping. |

## Test Shape

- `tests/unit_suite.rs` uses stub providers to exercise validation and fallback behavior without live network calls.
- The unit suite verifies both the orchestrator path and the `Tool` adapter path.

## Known Gaps & Limitations

- The crate inventories feature flags for all providers, but a default build compiles none of them.
- Provider behavior is normalized into the crate response model, so provider-specific payload details are intentionally not preserved here.
