# Glossary

- **Anchor**: Retrieval-oriented system for indexing and chunking embeddings to supply RAG (Retrieval-Augmented Generation) memory.
- **Beacon**: Prompt templating subsystem used to validate and render `{{variable}}` placeholders.
- **Bridge**: Narrow FFI layer for Python and Node entry points backed by Rust code.
- **Checkpoint Gate**: Named pause point that serializes state and can later restore it from persistence.
- **Colony**: Multi-agent coordination layer built around member roles and shared transcripts.
- **Compass**: Predicate router that selects a named route from a state object.
- **Conduit**: Orchustr term for an LLM provider client such as OpenAI or Anthropic.
- **DynState**: `HashMap<String, serde_json::Value>` used for dynamic state at graph, binding, and agent boundaries.
- **Forge Tool**: Schema-described async callable registered in `or-forge`. Not to be confused with the `or-tools-*` crates: `or-forge` is the registry agents invoke tools *through*, while the `or-tools-*` family provides concrete tool *implementations* (search, web, files, exec, ...) built on `or-tools-core`.
- **Lens**: Optional local execution dashboard and in-process trace collector (`or-lens`). Prism *installs* tracing; Lens *displays* it.
- **Loom**: Directed state-graph runtime with explicit entry, exit, branching, and pause behavior.
- **Nexus**: MCP client/server layer implemented by `or-mcp`. The crate also exports crate-name-aligned aliases `McpClient` and `McpServer` for `NexusClient`/`NexusServer`.
- **OrchState**: Core Rust trait that all strongly typed state objects implement; it defines clone, serde, and merge behavior.
- **Pipeline**: Ordered sequence of async nodes that each return a state patch merged into running state.
- **Prism**: Observability bootstrap crate for tracing subscriber and OTLP export installation. Pairs with **Lens**: Prism wires the pipes, Lens is the local dashboard you look at.
- **Recall**: Long-term conversational memory store backed by a persistence layer (e.g., PostgreSQL/SQLite) to retain continuous dialogue.
- **Relay**: Concurrent branch executor that merges multiple state patches deterministically.
- **Schema**: Serializable graph descriptors (`GraphSpec`, `NodeSpec`, `EdgeSpec`) in `or-schema`, loadable from YAML or JSON and shared by the runtimes, the CLI linter, and the bindings.
- **Sentinel**: Agent runtime that composes conduit, forge, and loom into think-act loops.
- **Sieve**: Data parsing constraint system designed to validate LLM outputs against strict JSON schemas before returning.

## Known Gaps & Limitations

- The glossary only covers terms defined directly in the codebase or used consistently across the repository.
- No separate terminology registry was found outside the source tree.
