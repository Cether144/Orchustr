# or-tools-productivity API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-productivity/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `EmailClient`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Async contract for listing and sending email.

**Signature**
```rust
pub trait EmailClient: Send + Sync + 'static
```

### `CalendarClient`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Async contract for listing and creating calendar events.

**Signature**
```rust
pub trait CalendarClient: Send + Sync + 'static
```

### `ProjectTracker`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Async contract for listing and creating issues/tasks in external trackers.

**Signature**
```rust
pub trait ProjectTracker: Send + Sync + 'static
```

### `KnowledgeBase`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Async contract for searching and creating pages/documents.

**Signature**
```rust
pub trait KnowledgeBase: Send + Sync + 'static
```

### `TeamMessenger`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Async contract for team messaging and message search.

**Signature**
```rust
pub trait TeamMessenger: Send + Sync + 'static
```

### `Email`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Normalized email record.

**Signature**
```rust
pub struct Email {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub timestamp: Option<String>,
}
```

### `CalendarEvent`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Normalized calendar event record.

**Signature**
```rust
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start: String,
    pub end: String,
    pub attendees: Vec<String>,
}
```

### `Issue`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Normalized issue or task record.

**Signature**
```rust
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assignee: Option<String>,
    pub labels: Vec<String>,
}
```

### `Page`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Normalized page or knowledge entry.

**Signature**
```rust
pub struct Page {
    pub id: String,
    pub title: String,
    pub content: String,
    pub url: Option<String>,
}
```

### `ProductivityTask`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Normalized task entity kept alongside the issue/page/email models.

**Signature**
```rust
pub struct ProductivityTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_date: Option<String>,
    pub assignee: Option<String>,
}
```

### `ProductivityError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Error model covering missing credentials, transport/upstream failures, invalid input, and not-found cases.

**Signature**
```rust
pub enum ProductivityError {
    MissingCredential(String),
    Transport(String),
    Upstream { provider: String, status: u16, body: String },
    InvalidInput(String),
    NotFound(String),
}
```

### `ProductivityOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Optional dependency holder for email, calendar, project tracker, knowledge base, and team messenger clients.

**Signature**
```rust
pub struct ProductivityOrchestrator { ... }
```

### `ProductivityTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-productivity/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-productivity/src/lib.rs` |

**Description**: Generic `Tool` adapter exposing a selected subset of orchestrator operations.

**Signature**
```rust
pub struct ProductivityTool { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `GmailClient` | `crates/tools/or-tools-productivity/src/infra/gmail.rs` | `gmail` | `EmailClient` |
| `GoogleCalendarClient` | `crates/tools/or-tools-productivity/src/infra/gcalendar.rs` | `gcalendar` | `CalendarClient` |
| `SlackMessenger` | `crates/tools/or-tools-productivity/src/infra/slack.rs` | `slack` | `TeamMessenger` |
| `JiraTracker` | `crates/tools/or-tools-productivity/src/infra/jira.rs` | `jira` | `ProjectTracker` |
| `GitHubTracker` | `crates/tools/or-tools-productivity/src/infra/github.rs` | `github` | `ProjectTracker` |
| `TrelloTracker` | `crates/tools/or-tools-productivity/src/infra/trello.rs` | `trello` | `ProjectTracker` |
| `NotionBase` | `crates/tools/or-tools-productivity/src/infra/notion.rs` | `notion` | `KnowledgeBase` |
| `ClickUpTracker` | `crates/tools/or-tools-productivity/src/infra/clickup.rs` | `clickup` | `ProjectTracker` |
| `OutlookEmailClient` | `crates/tools/or-tools-productivity/src/infra/office365.rs` | `office365` | `EmailClient` |
| `OutlookCalendarClient` | `crates/tools/or-tools-productivity/src/infra/office365.rs` | `office365` | `CalendarClient` |

## Known Gaps & Limitations

- Backend constructors are module-scoped and are not re-exported from `src/lib.rs`.
- The generic `Tool` surface exposes only a subset of the underlying trait methods.
