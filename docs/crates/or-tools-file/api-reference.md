# or-tools-file API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-file/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `FileStore`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Async contract for file-oriented storage backends.

**Signature**
```rust
pub trait FileStore: Send + Sync + 'static
```

### `DataSource`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Async contract for JSON-returning external data sources.

**Signature**
```rust
pub trait DataSource: Send + Sync + 'static
```

### `FileEntry`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Directory listing entry with normalized metadata.

**Signature**
```rust
pub struct FileEntry {
    pub path: String,
    pub size_bytes: u64,
    pub is_dir: bool,
    pub modified_at: Option<String>,
}
```

### `FileContent`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Normalized file read result.

**Signature**
```rust
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub size_bytes: u64,
}
```

### `JsonQuery`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: JSON query envelope using jq-style path segments.

**Signature**
```rust
pub struct JsonQuery {
    pub data: Value,
    pub path: Vec<String>,
}
```

### `ResearchPaper`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Normalized research paper record.

**Signature**
```rust
pub struct ResearchPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub summary: String,
    pub pdf_url: String,
    pub published: Option<String>,
    pub categories: Vec<String>,
}
```

### `FinancialRecord`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Normalized market snapshot record.

**Signature**
```rust
pub struct FinancialRecord {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change_pct: f64,
    pub volume: u64,
    pub market_cap: Option<f64>,
    pub currency: String,
}
```

### `FileError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Error model covering filesystem failures, JSON decoding issues, missing credentials, and upstream HTTP problems.

**Signature**
```rust
pub enum FileError {
    NotFound(String),
    PermissionDenied(String),
    Io { path: String, reason: String },
    Json(String),
    MissingCredential(String),
    Upstream { provider: String, status: u16, body: String },
    Transport(String),
}
```

### `FileOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-file/src/lib.rs` |

**Description**: Thin wrapper around a `FileStore` that currently exposes traced reads.

**Signature**
```rust
pub struct FileOrchestrator { ... }
```

### `FileStoreTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_file::application::orchestrators` |

**Description**: Generic `Tool` adapter exposing `read`, `write`, `list`, and `delete`.

**Signature**
```rust
pub struct FileStoreTool { ... }
```

### `DataSourceTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-file/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_file::application::orchestrators` |

**Description**: Generic `Tool` adapter for any `DataSource`.

**Signature**
```rust
pub struct DataSourceTool { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `LocalFileSystem` | `crates/tools/or-tools-file/src/infra/local_fs.rs` | `local` | `FileStore` |
| `JsonToolkit` | `crates/tools/or-tools-file/src/infra/json_toolkit.rs` | `json-toolkit` | `DataSource` |
| `GoogleDriveStore` | `crates/tools/or-tools-file/src/infra/gdrive.rs` | `gdrive` | `FileStore` |
| `ArxivSource` | `crates/tools/or-tools-file/src/infra/arxiv.rs` | `arxiv` | `DataSource` |
| `FinancialDatasetsSource` | `crates/tools/or-tools-file/src/infra/financial.rs` | `financial` | `DataSource` |

## Known Gaps & Limitations

- Backend constructors are module-scoped types and are not re-exported from `src/lib.rs`.
- The public API exposes normalized records only; provider-specific metadata from Drive, ArXiv, and Financial Datasets is not preserved here.
