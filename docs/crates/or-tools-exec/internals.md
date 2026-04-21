# or-tools-exec Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: executor contract, request/result entities, and crate-local errors.
- `application/`: executor routing and `Tool` adapter behavior.
- `infra/`: local process runners, remote sandbox clients, and shared HTTP helpers.
- `tests/`: stub executor coverage plus a real shell smoke test.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `python` | `infra/python.rs` | Local `python3 -c` executor. |
| `shell` | `infra/shell.rs` | Local shell / command executor. |
| `e2b` | `infra/e2b.rs` | E2B remote sandbox client. |
| `bearly` | `infra/bearly.rs` | Bearly remote execution client. |
| `daytona` | `infra/daytona.rs` | Daytona remote workspace execution client. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-exec/Cargo.toml` | toml | 31 | Package manifest, default features, and dependencies. |
| `crates/tools/or-tools-exec/src/lib.rs` | rs | 8 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-exec/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-exec/src/application/orchestrators.rs` | rs | 63 | Implements `ExecOrchestrator` and `ExecTool`. |
| `crates/tools/or-tools-exec/src/domain/contracts.rs` | rs | 11 | Defines the `CodeExecutor` trait. |
| `crates/tools/or-tools-exec/src/domain/entities.rs` | rs | 67 | Defines language, request, and result entities. |
| `crates/tools/or-tools-exec/src/domain/errors.rs` | rs | 47 | Defines `ExecError` and conversion to `ToolError`. |
| `crates/tools/or-tools-exec/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-exec/src/infra/bearly.rs` | rs | 66 | Feature-gated Bearly backend. |
| `crates/tools/or-tools-exec/src/infra/daytona.rs` | rs | 62 | Feature-gated Daytona backend. |
| `crates/tools/or-tools-exec/src/infra/e2b.rs` | rs | 68 | Feature-gated E2B backend. |
| `crates/tools/or-tools-exec/src/infra/mod.rs` | rs | 7 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-exec/src/infra/python.rs` | rs | 41 | Default local Python executor. |
| `crates/tools/or-tools-exec/src/infra/shared.rs` | rs | 22 | Shared response decoding, transport, and credential helpers. |
| `crates/tools/or-tools-exec/src/infra/shell.rs` | rs | 43 | Default local shell executor. |
| `crates/tools/or-tools-exec/tests/unit_suite.rs` | rs | 87 | Covers routing, invalid payloads, result helpers, and shell execution. |

## Test Shape

- `tests/unit_suite.rs` uses stub executors for orchestrator behavior and also runs a real `ShellExecutor` echo case.
- The unit suite validates both the orchestrator path and the `Tool` adapter path.

## Known Gaps & Limitations

- Local executors depend on host binaries and process spawning rather than a bundled runtime.
- Remote backends live behind feature flags and rely on credentials loaded from the environment.
