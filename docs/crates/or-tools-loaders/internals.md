# or-tools-loaders Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: loader contract, request/source entities, and crate-local errors.
- `application/`: loader registration, routing, and `Tool` adapter behavior.
- `infra/`: concrete format loaders plus shared source/chunk helpers.
- `tests/`: unit coverage for routing and format-specific behavior.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `text` | `infra/text.rs` | Plain-text loader. |
| `markdown` | `infra/markdown.rs` | Markdown loader with front-matter stripping. |
| `json` | `infra/json.rs` | JSON validation and pretty-print loader. |
| `csv` | `infra/csv_loader.rs` | CSV row-to-document loader. |
| `html` | `infra/html.rs` | HTML-to-text loader. |
| `pdf` | `infra/pdf.rs` | PDF text extraction loader. |
| `docx` | `(declared only)` | Feature flag present in `Cargo.toml`, but not wired in `src/infra/mod.rs`. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-loaders/Cargo.toml` | toml | 35 | Package manifest, feature flags, and optional parser dependencies. |
| `crates/tools/or-tools-loaders/src/lib.rs` | rs | 8 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-loaders/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-loaders/src/application/orchestrators.rs` | rs | 96 | Implements loader registration, routing, and `LoaderTool`. |
| `crates/tools/or-tools-loaders/src/domain/contracts.rs` | rs | 10 | Defines the `DocumentLoader` trait. |
| `crates/tools/or-tools-loaders/src/domain/entities.rs` | rs | 73 | Defines loader request, source, document, and kind types. |
| `crates/tools/or-tools-loaders/src/domain/errors.rs` | rs | 37 | Defines `LoaderError` and conversion to `ToolError`. |
| `crates/tools/or-tools-loaders/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-loaders/src/infra/csv_loader.rs` | rs | 62 | Feature-gated CSV loader backend. |
| `crates/tools/or-tools-loaders/src/infra/html.rs` | rs | 80 | Feature-gated HTML loader backend. |
| `crates/tools/or-tools-loaders/src/infra/json.rs` | rs | 24 | Feature-gated JSON loader backend. |
| `crates/tools/or-tools-loaders/src/infra/markdown.rs` | rs | 47 | Feature-gated Markdown loader backend. |
| `crates/tools/or-tools-loaders/src/infra/mod.rs` | rs | 14 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-loaders/src/infra/pdf.rs` | rs | 49 | Feature-gated PDF loader backend. |
| `crates/tools/or-tools-loaders/src/infra/shared.rs` | rs | 44 | Shared source reading and chunking helpers. |
| `crates/tools/or-tools-loaders/src/infra/text.rs` | rs | 17 | Feature-gated text loader backend. |
| `crates/tools/or-tools-loaders/tests/unit_suite.rs` | rs | 100 | Covers routing, chunking, validation, and tool dispatch. |

## Test Shape

- `tests/unit_suite.rs` exercises the orchestrator and `LoaderTool`.
- Inline tests also exist in `infra/markdown.rs` and `infra/html.rs` for helper behavior.

## Known Gaps & Limitations

- The public API advertises `Docx`, but the current infra module wiring stops at text, markdown, HTML, JSON, CSV, and PDF.
- Chunking is character-count based in `infra/shared.rs`; it does not attempt sentence-aware or token-aware boundaries.
