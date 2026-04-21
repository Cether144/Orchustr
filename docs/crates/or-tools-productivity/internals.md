# or-tools-productivity Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: capability-family traits, normalized productivity entities, and crate-local errors.
- `application/`: builder-style orchestrator and generic `Tool` adapter.
- `infra/`: feature-gated HTTP clients for email, calendar, tracker, knowledge, and messaging providers.
- `tests/`: stub-backed tool coverage for each supported operation and error path.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `gmail` | `infra/gmail.rs` | Gmail email client. |
| `gcalendar` | `infra/gcalendar.rs` | Google Calendar client. |
| `slack` | `infra/slack.rs` | Slack messenger client. |
| `jira` | `infra/jira.rs` | Jira project tracker. |
| `github` | `infra/github.rs` | GitHub issues tracker. |
| `trello` | `infra/trello.rs` | Trello tracker. |
| `notion` | `infra/notion.rs` | Notion knowledge base client. |
| `clickup` | `infra/clickup.rs` | ClickUp tracker. |
| `office365` | `infra/office365.rs` | Outlook email and calendar clients. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-productivity/Cargo.toml` | toml | 37 | Package manifest, feature flags, and dependencies. |
| `crates/tools/or-tools-productivity/src/lib.rs` | rs | 8 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-productivity/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-productivity/src/application/orchestrators.rs` | rs | 80 | Implements `ProductivityOrchestrator` and `ProductivityTool`. |
| `crates/tools/or-tools-productivity/src/domain/contracts.rs` | rs | 39 | Defines client-family traits. |
| `crates/tools/or-tools-productivity/src/domain/entities.rs` | rs | 49 | Defines normalized email, event, issue, page, and task entities. |
| `crates/tools/or-tools-productivity/src/domain/errors.rs` | rs | 15 | Defines `ProductivityError`. |
| `crates/tools/or-tools-productivity/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-productivity/src/infra/clickup.rs` | rs | 101 | Feature-gated ClickUp tracker. |
| `crates/tools/or-tools-productivity/src/infra/gcalendar.rs` | rs | 88 | Feature-gated Google Calendar client. |
| `crates/tools/or-tools-productivity/src/infra/github.rs` | rs | 109 | Feature-gated GitHub tracker. |
| `crates/tools/or-tools-productivity/src/infra/gmail.rs` | rs | 100 | Feature-gated Gmail client. |
| `crates/tools/or-tools-productivity/src/infra/jira.rs` | rs | 96 | Feature-gated Jira tracker. |
| `crates/tools/or-tools-productivity/src/infra/mod.rs` | rs | 22 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-productivity/src/infra/notion.rs` | rs | 113 | Feature-gated Notion knowledge base client. |
| `crates/tools/or-tools-productivity/src/infra/office365.rs` | rs | 170 | Feature-gated Outlook email/calendar clients. |
| `crates/tools/or-tools-productivity/src/infra/shared.rs` | rs | 18 | Shared URL, credential, and transport helpers. |
| `crates/tools/or-tools-productivity/src/infra/slack.rs` | rs | 81 | Feature-gated Slack messenger client. |
| `crates/tools/or-tools-productivity/src/infra/trello.rs` | rs | 86 | Feature-gated Trello tracker. |
| `crates/tools/or-tools-productivity/tests/unit_suite.rs` | rs | 116 | Covers supported tool ops plus invalid-op and missing-client paths. |

## Test Shape

- `tests/unit_suite.rs` uses stub implementations for each trait family instead of hitting real provider APIs.
- The suite validates the currently supported generic tool operations: email listing, event listing, issue listing, knowledge search, and message posting.
- Error coverage includes unknown operations and missing configured clients.

## Known Gaps & Limitations

- `ProductivityOrchestrator` can hold richer clients than `ProductivityTool` currently exposes.
- `office365.rs` only covers email and calendar, even though the crate description mentions broader productivity integrations.
