# or-tools-vector API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-vector/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `VectorStoreClient`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Async contract implemented by each vector backend.

**Signature**
```rust
pub trait VectorStoreClient: Send + Sync + 'static
```

### `Distance`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Distance metric selector.

**Signature**
```rust
pub enum Distance {
    Cosine,
    Euclidean,
    DotProduct,
}
```

### `CollectionConfig`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Collection creation settings.

**Signature**
```rust
pub struct CollectionConfig {
    pub name: String,
    pub dimension: u32,
    pub distance: Distance,
}
```

### `UpsertItem`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: One vector row to insert or update.

**Signature**
```rust
pub struct UpsertItem {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: Value,
}
```

### `UpsertBatch`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Batch upsert payload scoped to a collection.

**Signature**
```rust
pub struct UpsertBatch {
    pub collection: String,
    pub items: Vec<UpsertItem>,
}
```

### `DeleteRequest`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Deletion request by collection and IDs.

**Signature**
```rust
pub struct DeleteRequest {
    pub collection: String,
    pub ids: Vec<String>,
}
```

### `QueryFilter`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Vector query envelope.

**Signature**
```rust
pub struct QueryFilter {
    pub collection: String,
    pub vector: Vec<f32>,
    pub top_k: u32,
    pub filter: Option<Value>,
}
```

### `VectorMatch`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Normalized vector query hit.

**Signature**
```rust
pub struct VectorMatch {
    pub id: String,
    pub score: f32,
    pub metadata: Value,
}
```

### `VectorError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Error model covering credential, validation, collection, transport, upstream, and serialization failures.

**Signature**
```rust
pub enum VectorError {
    MissingCredential(String),
    InvalidInput(String),
    DimensionMismatch { expected: u32, actual: u32 },
    CollectionNotFound(String),
    Upstream { provider: String, status: u16, body: String },
    Transport { provider: String, reason: String },
    Serialization { provider: String, reason: String },
}
```

### `RagOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-vector/src/lib.rs` |

**Description**: Thin tracing wrapper around a single `VectorStoreClient`.

**Signature**
```rust
pub struct RagOrchestrator<C: VectorStoreClient> { ... }
```

### `VectorStoreTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-vector/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_vector::application::orchestrators` |

**Description**: Generic `Tool` adapter exposing vector operations through JSON payloads with `op` and `data`.

**Signature**
```rust
pub struct VectorStoreTool<C: VectorStoreClient> { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `PineconeClient` | `crates/tools/or-tools-vector/src/infra/pinecone.rs` | `pinecone` | `VectorStoreClient` |
| `WeaviateClient` | `crates/tools/or-tools-vector/src/infra/weaviate.rs` | `weaviate` | `VectorStoreClient` |
| `QdrantClient` | `crates/tools/or-tools-vector/src/infra/qdrant.rs` | `qdrant` | `VectorStoreClient` |
| `ChromaClient` | `crates/tools/or-tools-vector/src/infra/chroma.rs` | `chroma` | `VectorStoreClient` |
| `MilvusClient` | `crates/tools/or-tools-vector/src/infra/milvus.rs` | `milvus` | `VectorStoreClient` |
| `PgVectorClient` | `crates/tools/or-tools-vector/src/infra/pgvector.rs` | `pgvector` | `VectorStoreClient` |

## Known Gaps & Limitations

- Backend constructors are feature-gated and available through module paths rather than re-exported from `src/lib.rs`.
- The normalized API surface does not expose backend-specific administration features beyond collection ensure, upsert, delete, and query.
