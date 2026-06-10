# orchustr CLI (Go)

The official command-line interface for [Orchustr](https://github.com/Regent33/Orchustr).
A single static binary that scaffolds, validates, and runs Orchustr agent
projects in Rust, Python, TypeScript, or Dart.

## Install

```sh
go install github.com/Regent33/Orchustr/cli@latest
# or from a checkout:
cd cli && go build -o orchustr .
```

## Commands

| Command | What it does |
|---|---|
| `orchustr init <name>` | Create a new agent project (`--lang`, `--topology`, `--provider`) |
| `orchustr new node <name>` | Add a node handler stub in the project's language |
| `orchustr new topology <name>` | Add a custom Rust `LoopTopology` stub |
| `orchustr lint [path]` | Validate graphs offline — a project, a file, or a directory |
| `orchustr run [dir]` | Run the project with its language toolchain |
| `orchustr trace [dir]` | Open the local or-lens dashboard (delegates to the Rust binary) |
| `orchustr doctor` | Check your toolchains and project config |

Every path argument defaults to the current directory.

```sh
orchustr init hermes-agent --lang rust
cd hermes-agent
orchustr lint .
orchustr run .
```

## Architecture

Feature-based clean architecture: each command is a feature under
`features/<name>/` with `domain/` (pure rules), `data/` (filesystem, process,
embedded templates), and `presentation/` (cobra command). Shared primitives
live in `shared/kernel/`; all wiring happens in `app/di/`.

`orchustr trace` is the one command that needs the Rust toolchain: the
dashboard is hosted in-process by `or-lens`, so the Go CLI delegates to the
Rust `orchustr` binary (`cargo install --path crates/or-cli`), or to the
binary named by `ORCHUSTR_RUST_BIN`.

## Development

```sh
go test ./...
go vet ./...
```
