# or-tools-file Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: file/data contracts, normalized entities, and crate-local errors.
- `application/`: store wrapper plus generic `Tool` adapters for file stores and data sources.
- `infra/`: local filesystem backend, JSON path toolkit, and feature-gated HTTP integrations.
- `tests/`: in-memory store coverage and JSON toolkit behavior.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `local` | `infra/local_fs.rs` | Host filesystem backend implementing `FileStore`. |
| `json-toolkit` | `infra/json_toolkit.rs` | Traverses a JSON payload using an array of path segments. |
| `gdrive` | `infra/gdrive.rs` | Google Drive-backed `FileStore`. |
| `arxiv` | `infra/arxiv.rs` | ArXiv query-backed `DataSource`. |
| `financial` | `infra/financial.rs` | Financial Datasets snapshot-backed `DataSource`. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-file/Cargo.toml` | toml | 32 | Package manifest, default features, and dependencies. |
| `crates/tools/or-tools-file/src/lib.rs` | rs | 8 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-file/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-file/src/application/orchestrators.rs` | rs | 98 | Implements `FileOrchestrator`, `FileStoreTool`, and `DataSourceTool`. |
| `crates/tools/or-tools-file/src/domain/contracts.rs` | rs | 20 | Defines `FileStore` and `DataSource`. |
| `crates/tools/or-tools-file/src/domain/entities.rs` | rs | 49 | Defines normalized file and data-source entities. |
| `crates/tools/or-tools-file/src/domain/errors.rs` | rs | 35 | Defines `FileError` and conversion to `ToolError`. |
| `crates/tools/or-tools-file/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-file/src/infra/arxiv.rs` | rs | 84 | Feature-gated ArXiv data source. |
| `crates/tools/or-tools-file/src/infra/financial.rs` | rs | 86 | Feature-gated Financial Datasets source. |
| `crates/tools/or-tools-file/src/infra/gdrive.rs` | rs | 112 | Feature-gated Google Drive store. |
| `crates/tools/or-tools-file/src/infra/json_toolkit.rs` | rs | 49 | Default JSON path toolkit. |
| `crates/tools/or-tools-file/src/infra/local_fs.rs` | rs | 42 | Default local filesystem backend. |
| `crates/tools/or-tools-file/src/infra/mod.rs` | rs | 7 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-file/src/infra/shared.rs` | rs | 7 | Shared transport and credential helpers. |
| `crates/tools/or-tools-file/tests/unit_suite.rs` | rs | 84 | Covers store behavior, tool dispatch, and JSON toolkit resolution. |

## Test Shape

- `tests/unit_suite.rs` uses an in-memory `MemStore` to validate `read`, `write`, `list`, and `delete` semantics without touching the host filesystem.
- The suite exercises both direct store usage and `FileStoreTool` dispatch.
- JSON toolkit coverage validates both nested resolution and missing-path behavior under the default `json-toolkit` feature.

## Known Gaps & Limitations

- `FileOrchestrator` is intentionally thin and does not yet wrap the full `FileStore` surface.
- `GoogleDriveStore` mixes two path conventions: file IDs for read/delete and file names for create/write.
- `LocalFileSystem::delete()` is file-only and does not remove directories recursively.
