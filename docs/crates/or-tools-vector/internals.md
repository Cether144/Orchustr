# or-tools-vector Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: vector-store contract, collection/query/upsert types, and crate-local errors.
- `application/`: tracing-oriented orchestrator and generic `Tool` adapter.
- `infra/`: feature-gated backend clients plus shared HTTP helper functions.
- `tests/`: stub-driven vector client coverage.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `pinecone` | `infra/pinecone.rs` | Pinecone HTTP client. |
| `weaviate` | `infra/weaviate.rs` | Weaviate REST client. |
| `qdrant` | `infra/qdrant.rs` | Qdrant REST client. |
| `chroma` | `infra/chroma.rs` | ChromaDB HTTP client. |
| `milvus` | `infra/milvus.rs` | Milvus REST v2 client. |
| `pgvector` | `infra/pgvector.rs` | Postgres / PGVector backend using `sqlx`. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-vector/Cargo.toml` | toml | 35 | Package manifest, feature flags, and optional `sqlx` dependency. |
| `crates/tools/or-tools-vector/src/lib.rs` | rs | 14 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-vector/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-vector/src/application/orchestrators.rs` | rs | 125 | Implements `RagOrchestrator` and `VectorStoreTool`. |
| `crates/tools/or-tools-vector/src/domain/contracts.rs` | rs | 19 | Defines the `VectorStoreClient` trait. |
| `crates/tools/or-tools-vector/src/domain/entities.rs` | rs | 75 | Defines collection, upsert, delete, query, and result entities. |
| `crates/tools/or-tools-vector/src/domain/errors.rs` | rs | 68 | Defines `VectorError` and conversion to `ToolError`. |
| `crates/tools/or-tools-vector/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-vector/src/infra/chroma.rs` | rs | 129 | Feature-gated Chroma backend. |
| `crates/tools/or-tools-vector/src/infra/milvus.rs` | rs | 108 | Feature-gated Milvus backend. |
| `crates/tools/or-tools-vector/src/infra/mod.rs` | rs | 14 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-vector/src/infra/pgvector.rs` | rs | 109 | Feature-gated PGVector backend. |
| `crates/tools/or-tools-vector/src/infra/pinecone.rs` | rs | 106 | Feature-gated Pinecone backend. |
| `crates/tools/or-tools-vector/src/infra/qdrant.rs` | rs | 114 | Feature-gated Qdrant backend. |
| `crates/tools/or-tools-vector/src/infra/shared.rs` | rs | 63 | Shared response decoding, status checking, transport, and credential helpers. |
| `crates/tools/or-tools-vector/src/infra/weaviate.rs` | rs | 127 | Feature-gated Weaviate backend. |
| `crates/tools/or-tools-vector/tests/unit_suite.rs` | rs | 108 | Covers stub-client upsert, query, tool dispatch, and enum serialization. |

## Test Shape

- `tests/unit_suite.rs` drives the crate through a stub `VectorStoreClient`.
- The test suite validates both direct orchestrator usage and the generic `Tool` payload path.

## Known Gaps & Limitations

- A default build wires no backend modules because every real backend is feature-gated.
- `pgvector` is the only backend with an extra dependency (`sqlx`) and an external database requirement.
