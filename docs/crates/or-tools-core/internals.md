# or-tools-core Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: shared tool contracts, serializable envelopes, and the canonical error type.
- `application/`: dispatch orchestration and tracing around tool invocation.
- `infra/`: concrete registry implementation.
- `tests/`: crate-level unit coverage for registration and dispatch behavior.

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-core/Cargo.toml` | toml | 20 | Package manifest and shared dependency declarations. |
| `crates/tools/or-tools-core/src/lib.rs` | rs | 14 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-core/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-core/src/application/orchestrators.rs` | rs | 46 | Implements `ToolDispatcher`. |
| `crates/tools/or-tools-core/src/domain/contracts.rs` | rs | 27 | Defines `Tool` and `ToolRegistry`. |
| `crates/tools/or-tools-core/src/domain/entities.rs` | rs | 84 | Defines metadata, input, output, and capability types. |
| `crates/tools/or-tools-core/src/domain/errors.rs` | rs | 63 | Defines `ToolError` and helper constructors. |
| `crates/tools/or-tools-core/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-core/src/infra/mod.rs` | rs | 1 | Wires the infra module. |
| `crates/tools/or-tools-core/src/infra/registry.rs` | rs | 53 | Implements `InMemoryToolRegistry`. |
| `crates/tools/or-tools-core/tests/unit_suite.rs` | rs | 89 | Covers registration, listing, dispatch, and failure propagation. |

## Test Shape

- `tests/unit_suite.rs` exercises the registry and dispatcher through stub tool implementations.
- The crate has no feature-gated infra modules, so the default test surface is the full crate surface.

## Known Gaps & Limitations

- The internals here only cover the shared tool runtime; individual tool families add their own orchestrators and payload semantics in downstream crates.
