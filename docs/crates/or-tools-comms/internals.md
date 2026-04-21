# or-tools-comms Internals

If you are changing this crate rather than just using it, this page is your maintenance map. It shows where responsibilities live so you know which layer to inspect first.

## Layering

- `domain/`: outbound/send contracts, social-read contract, normalized message entities, and crate-local errors.
- `application/`: sender routing and `Tool` adapter behavior.
- `infra/`: feature-gated HTTP senders plus shared credential/transport helpers.
- `tests/`: stub sender coverage for routing and payload validation.

## Feature-Gated Infra

| Feature | Module | Role |
|---|---|---|
| `twilio` | `infra/twilio.rs` | Twilio SMS sender. |
| `telegram` | `infra/telegram.rs` | Telegram bot sender. |
| `discord` | `infra/discord.rs` | Discord channel sender. |
| `whatsapp` | `infra/whatsapp.rs` | WhatsApp Cloud API sender. |
| `facebook` | `infra/facebook.rs` | Facebook page messaging sender. |
| `messenger` | `infra/messenger.rs` | Meta Messenger sender. |

## File Registry

| Path | Type | LOC | Role |
|---|---|---:|---|
| `crates/tools/or-tools-comms/Cargo.toml` | toml | 34 | Package manifest, feature flags, and dependencies. |
| `crates/tools/or-tools-comms/src/lib.rs` | rs | 8 | Public module wiring and crate re-exports. |
| `crates/tools/or-tools-comms/src/application/mod.rs` | rs | 1 | Wires the application module. |
| `crates/tools/or-tools-comms/src/application/orchestrators.rs` | rs | 52 | Implements `CommsOrchestrator` and `CommsTool`. |
| `crates/tools/or-tools-comms/src/domain/contracts.rs` | rs | 16 | Defines `MessageSender` and `SocialReader`. |
| `crates/tools/or-tools-comms/src/domain/entities.rs` | rs | 35 | Defines channels, messages, send results, and social posts. |
| `crates/tools/or-tools-comms/src/domain/errors.rs` | rs | 15 | Defines `CommsError`. |
| `crates/tools/or-tools-comms/src/domain/mod.rs` | rs | 3 | Wires the domain module. |
| `crates/tools/or-tools-comms/src/infra/discord.rs` | rs | 54 | Feature-gated Discord sender. |
| `crates/tools/or-tools-comms/src/infra/facebook.rs` | rs | 58 | Feature-gated Facebook sender. |
| `crates/tools/or-tools-comms/src/infra/messenger.rs` | rs | 54 | Feature-gated Messenger sender. |
| `crates/tools/or-tools-comms/src/infra/mod.rs` | rs | 16 | Feature-gated infra module wiring. |
| `crates/tools/or-tools-comms/src/infra/shared.rs` | rs | 9 | Shared credential and transport helpers. |
| `crates/tools/or-tools-comms/src/infra/telegram.rs` | rs | 57 | Feature-gated Telegram sender. |
| `crates/tools/or-tools-comms/src/infra/twilio.rs` | rs | 70 | Feature-gated Twilio sender. |
| `crates/tools/or-tools-comms/src/infra/whatsapp.rs` | rs | 70 | Feature-gated WhatsApp sender. |
| `crates/tools/or-tools-comms/tests/unit_suite.rs` | rs | 67 | Covers routing, unsupported channels, tool invocation, and invalid payloads. |

## Test Shape

- `tests/unit_suite.rs` uses stub `MessageSender` implementations instead of real network backends.
- The suite verifies successful routing, skipped non-matching senders, unsupported channels, and `CommsTool` payload decoding.
- No test currently exercises a real feature-gated provider module or the `SocialReader` trait.

## Known Gaps & Limitations

- The current infra layer only covers outbound sending; social read support remains a domain-level extension point.
- Channel routing relies on string equality between sender `channel()` values and lowercased enum debug output.
