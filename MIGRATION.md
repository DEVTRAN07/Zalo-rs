# Migration Guide

This repository is now a pure Rust crate (`zalo-client`, v0.1.0) at the repository root. The previous TypeScript/Node.js implementation and npm packaging are no longer part of this tree.

## Who this guide is for

- Users of the old TypeScript/Node `zalo-client` / `zca-js` codebase who want to move to the Rust crate.
- Contributors updating old docs, scripts, or examples that still assume `npm`, `bun`, `tsc`, `src/*.ts`, or a `rust-client/` subdirectory.

## Key changes

| Area | Previous state | Current state |
| --- | --- | --- |
| Language/runtime | TypeScript / Node.js | Rust / Tokio |
| Package manager | npm / bun | Cargo |
| Crate/package location | TS project; earlier experiments used `rust-client/` | `Cargo.toml` at repository root |
| Distribution | npm-oriented workflow in legacy project | Git/path dependency from this repository; no crates.io publication claimed here |
| API style | JS/TS objects and promises | Rust structs, enums, `async` methods, `Result<T, _>` |

## Porting notes

The Rust API is not a drop-in replacement. Method names, payload types, and signatures differ. Use the Rust source and examples as the source of truth.

### Login

```rust
use zalo_client::{Credentials, ZaloClient, ZaloCookie};

let client = ZaloClient::new(Default::default());
let api = client.login(Credentials {
    imei,
    cookie,      // Vec<ZaloCookie>
    user_agent,
    language: Some("vi".into()),
}).await?;
```

### Sending a message

`ZaloApi::send_message` currently accepts `(message, thread_id, thread_type: u8)`. A `ThreadType` enum exists under `zalo_client::models::r#enum::ThreadType` if you want typed conversions.

```rust
let result = api.send_message("hello", thread_id, 0).await?;
```

Check the current signature in `src/api/mod.rs` and `src/api/send_message.rs` before porting production code.

### Real-time events

The Rust crate exposes a WebSocket listener:

```rust
let mut listener = api.listener();
let mut rx = listener.start()?;
while let Some(event) = rx.recv().await {
    // handle event
}
```

Event payloads follow the reverse-engineered Zalo web protocol and may not match legacy TypeScript event shapes exactly.

### QR login

QR login is exposed as low-level, caller-driven helpers in `src/api/login_qr.rs`. It should not be treated as a complete interactive login replacement.

## Migration checklist

1. Replace Node.js dependencies/scripts with a Cargo-based Rust project.
2. Add `tokio` and `serde_json` (or equivalent) to your Rust application.
3. Recreate credential loading. Do not hardcode cookies, IMEI values, or tokens.
4. Port each endpoint call by checking its Rust method signature and response type.
5. Move event handling to the async listener/channel model.
6. Run `cargo test` and `cargo build` in this repository when developing changes.

## Limitations

- No npm package is provided by this repository.
- Protocol behavior is reverse-engineered and can change without notice.
- The Rust port has broad endpoint coverage, but exact behavioral parity with the legacy TypeScript client is not guaranteed.
- WebSocket event decoding may be incomplete for some payloads.
