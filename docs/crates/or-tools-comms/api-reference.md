# or-tools-comms API Reference

This page documents the public surface re-exported by `crates/tools/or-tools-comms/src/lib.rs` and the key entry points behind those re-exports.

If you are new to this crate, start with the README first. This page is the precise reference for integrators and maintainers who need exact public type names, file locations, and entry points.

### `MessageSender`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Async contract for outbound channel senders.

**Signature**
```rust
pub trait MessageSender: Send + Sync + 'static
```

### `SocialReader`

| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/contracts.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Async contract for reading social platform data.

**Signature**
```rust
pub trait SocialReader: Send + Sync + 'static
```

### `Channel`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Normalized outbound channel selector.

**Signature**
```rust
pub enum Channel {
    Sms,
    Telegram,
    Discord,
    WhatsApp,
    Facebook,
    Messenger,
}
```

### `Message`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Outbound message envelope.

**Signature**
```rust
pub struct Message {
    pub channel: Channel,
    pub to: String,
    pub body: String,
    pub from: Option<String>,
}
```

### `SendResult`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Normalized send result returned by senders.

**Signature**
```rust
pub struct SendResult {
    pub message_id: String,
    pub channel: Channel,
    pub status: String,
}
```

### `SocialPost`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/entities.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Normalized social feed item.

**Signature**
```rust
pub struct SocialPost {
    pub id: String,
    pub author: String,
    pub body: String,
    pub timestamp: Option<String>,
    pub platform: String,
}
```

### `CommsError`

| Property | Value |
|---|---|
| **Kind** | enum |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/domain/errors.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Error model covering missing credentials, transport/upstream failures, invalid input, and unsupported channels.

**Signature**
```rust
pub enum CommsError {
    MissingCredential(String),
    Transport(String),
    Upstream { provider: String, status: u16, body: String },
    InvalidInput(String),
    UnsupportedChannel(String),
}
```

### `CommsOrchestrator`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: Routes a `Message` to the first registered sender whose `channel()` matches the normalized channel string.

**Signature**
```rust
pub struct CommsOrchestrator { ... }
```

### `CommsTool`

| Property | Value |
|---|---|
| **Kind** | struct |
| **Visibility** | pub |
| **File** | `crates/tools/or-tools-comms/src/application/orchestrators.rs` |
| **Exported from** | `crates/tools/or-tools-comms/src/lib.rs` |

**Description**: `Tool` adapter that deserializes a `Message`, sends it through `CommsOrchestrator`, and returns a serialized `SendResult`.

**Signature**
```rust
pub struct CommsTool { ... }
```

## Feature-gated Backend Types

| Type | File | Feature | Implements |
|---|---|---|---|
| `TwilioSender` | `crates/tools/or-tools-comms/src/infra/twilio.rs` | `twilio` | `MessageSender` |
| `TelegramSender` | `crates/tools/or-tools-comms/src/infra/telegram.rs` | `telegram` | `MessageSender` |
| `DiscordSender` | `crates/tools/or-tools-comms/src/infra/discord.rs` | `discord` | `MessageSender` |
| `WhatsAppSender` | `crates/tools/or-tools-comms/src/infra/whatsapp.rs` | `whatsapp` | `MessageSender` |
| `FacebookSender` | `crates/tools/or-tools-comms/src/infra/facebook.rs` | `facebook` | `MessageSender` |
| `MessengerSender` | `crates/tools/or-tools-comms/src/infra/messenger.rs` | `messenger` | `MessageSender` |

## Known Gaps & Limitations

- No concrete `SocialReader` type is exposed from the current infra layer.
- Provider constructors are available through module paths and are not re-exported from `src/lib.rs`.
