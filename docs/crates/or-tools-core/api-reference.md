# or-tools-core API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-core/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `Tool`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Core async contract implemented by every Orchustr tool.

**Signature**
```rust
pub trait Tool: Send + Sync + 'static
```

### `ToolRegistry`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Async registry abstraction that stores and retrieves tools by name.

**Signature**
```rust
pub trait ToolRegistry: Send + Sync + 'static
```

### `ToolCapability`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Declarative capability tags used for discovery and policy decisions.

**Signature**
```rust
pub enum ToolCapability {
    Network,
    Filesystem,
    Subprocess,
    Database,
    Vector,
    Auth,
    Streaming,
}
```

### `ToolMeta`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Declarative metadata describing a registered tool.

**Signature**
```rust
pub struct ToolMeta {
    pub name: String,
    pub description: String,
    pub capabilities: Vec<ToolCapability>,
    pub input_schema: Option<Value>,
    pub output_schema: Option<Value>,
}
```

### `ToolInput`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Tool invocation envelope.

**Signature**
```rust
pub struct ToolInput {
    pub tool: String,
    pub payload: Value,
}
```

### `ToolOutput`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Tool invocation result envelope.

**Signature**
```rust
pub struct ToolOutput {
    pub tool: String,
    pub payload: Value,
    pub duration_ms: u64,
}
```

### `ToolError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Canonical error type returned by every tool implementation.

**Signature**
```rust
pub enum ToolError {
    NotFound(String),
    AlreadyRegistered { name: String },
    InvalidInput { tool: String, reason: String },
    Transport { tool: String, reason: String },
    Upstream { tool: String, status: u16, body: String },
    MissingCredential { tool: String, env_var: String },
    Timeout { tool: String, timeout_ms: u64 },
    Unavailable { tool: String, reason: String },
    Serialization { tool: String, reason: String },
}
```

### `ToolDispatcher`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: Resolves a tool from a `ToolRegistry`, invokes it, and records elapsed milliseconds in the returned `ToolOutput`.

**Signature**
```rust
pub struct ToolDispatcher { ... }
```

### `InMemoryToolRegistry`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-core/src/infra/registry.rs` |
| **Exported from** | `crates/tools/or-tools-core/src/lib.rs` |

**Description**: HashMap-backed `ToolRegistry` implementation used by tests and lightweight in-process tool setups.

**Signature**
```rust
pub struct InMemoryToolRegistry { ... }
```

## Known Gaps & Limitations

- `ToolDispatcher` records duration on success only because the `ToolOutput` envelope is only available on successful invocations.
- The API surface in this crate is intentionally generic; concrete tool payload schemas are defined in downstream crates.
