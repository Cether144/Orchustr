# API Reference Matrix

This document provides a highly detailed yet intuitive breakdown of all the **primary functions, methods, and classes** across the Orchustr framework. It maps the native Rust concepts (`crates/`) to their respective availability in the **Python**, **TypeScript**, and **Dart** bindings.

> [!NOTE]
> 🟢 **Available natively** | 🟡 **Re-implemented purely in binding language** | 🔴 **Not yet exposed**

---

## 1. Core State & Utilities (`or-core` / `or-checkpoint`)
Manages the shared memory maps, retry policies, token limits, and DB checkpoints.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`DynState`** | `merge(patch)`, `get(key)` | The universal dictionary used to pass memory between nodes. It safely deep-merges nested dictionaries when an agent finishes a step. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`TokenBudget`** | `new(ctx, comp)` | Defines strict token limits to prevent LLMs from infinitely generating or blowing past billing caps before HTTP calls are even made. `or-core`. | 🟢 | 🔴 | 🔴 | 🔴 |
| **`RetryPolicy`** | `default_llm()`, `delay()` | Defines the exponential backoff strategies when encountering `HTTP 429` Rate Limits or `503` server errors. `or-core`. | 🟢 | 🔴 | 🔴 | 🔴 |
| **`CheckpointGate`** | `save()`, `restore()` | Pauses a graph's execution, serializes the `DynState` to a database via a PersistenceBackend, and waits for Human-in-the-loop approval. `or-checkpoint`. | 🟢 | 🔴 | 🔴 | 🔴 |

---

## 2. LLM Providers (`or-conduit`)
The abstraction layer meant to route Prompts and chat completion instructions to 22+ AI providers.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`OpenAiCompatConduit`** | `complete_text(prompt)` | Sends a single raw string to OpenAI-compatible endpoints (OpenRouter, Groq, Together, Mistral, Ollama) and returns the text output. | 🟢 | 🟡 | 🟡 | 🟡 |
| *(Any Conduit)* | `complete_messages(list)` | The lower-level API that accepts specific role arrays (System, User, Assistant) with Vision/Image contexts. Used internally by Agents. | 🟢 | 🟡 | 🟡 | 🟡 |
| *(Any Conduit)* | `stream_text()` | Streams the response chunk-by-chunk for Real-time UX interactions or streaming web-apps. | 🟢 | 🔴 | 🟡 | 🔴 |
| **`AnthropicConduit`** | `new(key, model)` | Bypasses the OpenAI standard mapping to handle Claude's unique requirement of placing System messages at the top-level rather than in the array. | 🟢 | 🟡 | 🟡 | 🟡 |

---

## 3. Tool Calling & MCP (`or-forge`, `or-mcp`, `or-sieve`)
Gives your agents "hands" to manipulate files, run shell commands, fetch web pages, or bridge into local MCP servers.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`ForgeRegistry`** | `register_tool(name, fn)` | Registers local native language codes (like a Python script or JS function) so the LLM can trigger it. `or-forge`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`ForgeRegistry`** | `invoke(name, args)` | Programmatically executes a registered tool via its JSON schema. `or-forge`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`ForgeRegistry`** | `import_from_mcp(client)` | Imports remote tools from a local MCP server (like the Firebase or Desktop MCP servers) directly into the agent. `or-mcp`. | 🟢 | 🔴 | 🟡 | 🟡 |
| **`NexusClient`** | `connect_http(url)` | Creates a bidirectional WebSocket/HTTP connection to a Model Context Protocol Server to discover remote tools. `or-mcp`. | 🟢 | 🔴 | 🟡 | 🟡 |
| **`Sieve`** | `validate_json(schema)` | Validates that an LLM's returned JSON arguments exactly match the strict JSON Schema defined by your tool before executing it. `or-sieve`. | 🟢 | 🔴 | 🔴 | 🔴 |

---

## 4. Graphs, Pipelines & Routing (`or-loom`, `or-pipeline`, `or-relay`)
The heart of Orchustr's ReAct agent memory looping and flow control.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`GraphBuilder`** | `add_node()`, `add_edge()` | Creates cyclic workflows where an LLM can think, execute, and loop back indefinitely until finished. `or-loom`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`GraphBuilder`** | `set_entry()`, `set_exit()`| Dictates exactly where the memory loop begins and at what node the entire process shuts down. `or-loom`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`GraphExecutor`** | `execute(initial_state)` | Compiles the graph, checks for deadlocks, and begins the async traversal until the exit node is hit. `or-loom`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`PipelineBuilder`** | `add_node(name, fn)` | Creates a strict **sequential** pipeline of tasks. No complex cycles or branching, just Step A -> Step B -> Step C. `or-pipeline`. | 🟢 | 🟡 | 🟡 | 🟡 |
| **`RelayRouter`** | `fan_out()`, `gather()` | Executes multiple tasks concurrently (e.g. searching 3 web pages at the same time) and gathers the result. `or-relay`. | 🟢 | 🔴 | 🔴 | 🔴 |

---

## 5. Prompts & Memory (`or-beacon`, `or-anchor`, `or-recall`)
Ensures templates are safely passed downstream, and handles RAG (Retrieval-Augmented Generation) memory.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`PromptBuilder`** | `template()`, `render()` | Defines a Handlebars/Mustache string like `"Hello {{name}}!"` and safely substitutes parameters to prevent injection attacks. `or-beacon`. | 🟢 | 🟢 | 🟢 | 🟢 |
| **`AnchorPipeline`** | `chunk()`, `retrieve()` | Takes massive text documents, splits them into small embeddings, and retrieves the most relevant chunks via Vector cosine-similarity. `or-anchor`. | 🟢 | 🔴 | 🔴 | 🔴 |
| **`RecallStore`** | `store_interaction()` | Automatically stores User and Assistant dialogue into a long-term database (like SQLite/PostgreSQL) so the agent remembers past conversations across sessions. `or-recall`.| 🟢 | 🔴 | 🔴 | 🔴 |

---

## 6. Pre-Packaged Agents (`or-sentinel`)
If you don't want to build a graph from scratch, these pre-built templates do the work for you.

| Struct / Class | Key Functions / Methods | Purpose & Usecase | Rust | Python | TS | Dart |
|:---|:---|:---|:---:|:---:|:---:|:---:|
| **`ReActAgent`** | `new(conduit, tools)` | A pre-compiled graph that automatically gives an LLM a Prompt, lets it call tools from a Registry, and loops back to itself until it solves the user's objective. | 🟢 | 🔴 | 🔴 | 🔴 |
| **`RouterAgent`** | `route(intent)` | A pre-compiled pipeline that uses a fast LLM to look at a user's prompt and "routes" it to a specific sub-agent (e.g. routing math questions to the Calculator Agent). | 🟢 | 🔴 | 🔴 | 🔴 |

---

> [!TIP]
> **Why are some marked as 🟡 Re-implemented?** 
> To maximize parallel performance in JavaScript, Python, and Dart, network features like `fetch` or `aiohttp` are implemented entirely natively inside those bindings rather than funneling C-string pointers across an FFI bridge. Operations that require zero-latency mathematical bounding or secure Handlebars processing (like `PromptBuilder`) rely directly on the Native Rust Bridge (`or-bridge`).
