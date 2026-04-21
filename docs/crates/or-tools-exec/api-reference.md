# or-tools-exec API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-exec/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `CodeExecutor`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Async execution contract implemented by each runtime backend.

**Signature**
```rust
pub trait CodeExecutor: Send + Sync + 'static
```

### `Language`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Normalized language selector.

**Signature**
```rust
pub enum Language {
    Python,
    Shell,
    JavaScript,
    TypeScript,
    Ruby,
}
```

### `ExecRequest`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Execution request envelope.

**Signature**
```rust
pub struct ExecRequest {
    pub code: String,
    pub language: Language,
    pub timeout_ms: u64,
    pub env: HashMap<String, String>,
}
```

### `ExecResult`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Normalized execution result.

**Signature**
```rust
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
}
```

### `ExecError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Execution error model covering unsupported languages, missing credentials, timeouts, process spawn/IO failures, and upstream transport failures.

**Signature**
```rust
pub enum ExecError {
    UnsupportedLanguage(String),
    ExecutorNotFound { executor: String, reason: String },
    MissingCredential(String),
    Timeout(u64),
    Spawn(String),
    Upstream { provider: String, status: u16, body: String },
    Transport(String),
    Io(String),
}
```

### `ExecOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-exec/src/lib.rs` |

**Description**: Chooses the first registered executor that supports the requested language and runs it with tracing.

**Signature**
```rust
pub struct ExecOrchestrator { ... }
```

### `ExecTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-exec/src/application/orchestrators.rs` |
| **Exported from** | `or_tools_exec::application::orchestrators` |

**Description**: Generic `Tool` adapter for `ExecOrchestrator`.

**Signature**
```rust
pub struct ExecTool { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `PythonExecutor` | `crates/tools/or-tools-exec/src/infra/python.rs` | `python` | `CodeExecutor` |
| `ShellExecutor` | `crates/tools/or-tools-exec/src/infra/shell.rs` | `shell` | `CodeExecutor` |
| `E2BExecutor` | `crates/tools/or-tools-exec/src/infra/e2b.rs` | `e2b` | `CodeExecutor` |
| `BearlyExecutor` | `crates/tools/or-tools-exec/src/infra/bearly.rs` | `bearly` | `CodeExecutor` |
| `DaytonaExecutor` | `crates/tools/or-tools-exec/src/infra/daytona.rs` | `daytona` | `CodeExecutor` |

## Known Gaps & Limitations

- Executor constructors are exposed through their module paths rather than re-exported from `src/lib.rs`.
- The public API normalizes execution results, so provider-specific sandbox metadata is not surfaced here.
