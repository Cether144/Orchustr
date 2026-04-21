# or-tools-loaders API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-loaders/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `DocumentLoader`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Async contract implemented by each loader backend.

**Signature**
```rust
pub trait DocumentLoader: Send + Sync + 'static
```

### `DocumentKind`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Document kind taxonomy used for routing and output metadata.

**Signature**
```rust
pub enum DocumentKind {
    Text,
    Markdown,
    Html,
    Json,
    Csv,
    Pdf,
    Docx,
    Unknown,
}
```

### `LoaderSource`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Input source as a path or raw content payload.

**Signature**
```rust
pub enum LoaderSource {
    Path { path: String },
    Raw { content: String },
}
```

### `LoaderRequest`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Loader request envelope.

**Signature**
```rust
pub struct LoaderRequest {
    pub source: LoaderSource,
    pub kind_hint: Option<DocumentKind>,
    pub chunk_size: usize,
    pub metadata: Value,
}
```

### `Document`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Normalized document chunk.

**Signature**
```rust
pub struct Document {
    pub content: String,
    pub kind: DocumentKind,
    pub chunk_index: usize,
    pub metadata: Value,
}
```

### `LoaderError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Loader-specific error model.

**Signature**
```rust
pub enum LoaderError {
    UnsupportedFormat(String),
    Io { path: String, reason: String },
    Parse(String),
    InvalidSource(String),
}
```

### `LoaderOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-loaders/src/lib.rs` |

**Description**: Router that picks a loader by explicit kind hint or path extension.

**Signature**
```rust
pub struct LoaderOrchestrator { ... }
```

### `LoaderTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-loaders/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_loaders::application::orchestrators` |

**Description**: Generic `Tool` adapter for `LoaderOrchestrator`.

**Signature**
```rust
pub struct LoaderTool { ... }
```

## Feature-gated Loader Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `TextLoader` | `crates/tools/or-tools-loaders/src/infra/text.rs` | `text` | `DocumentLoader` |
| `MarkdownLoader` | `crates/tools/or-tools-loaders/src/infra/markdown.rs` | `markdown` | `DocumentLoader` |
| `JsonLoader` | `crates/tools/or-tools-loaders/src/infra/json.rs` | `json` | `DocumentLoader` |
| `CsvLoader` | `crates/tools/or-tools-loaders/src/infra/csv_loader.rs` | `csv` | `DocumentLoader` |
| `HtmlLoader` | `crates/tools/or-tools-loaders/src/infra/html.rs` | `html` | `DocumentLoader` |
| `PdfLoader` | `crates/tools/or-tools-loaders/src/infra/pdf.rs` | `pdf` | `DocumentLoader` |

## Known Gaps & Limitations

- `DocumentKind::Docx` exists in the public API, but the current source tree does not wire a `docx` loader implementation.
- Binary raw input is only handled explicitly in the PDF path; other raw sources are treated as strings.
