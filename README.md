# zalo-client

Rust client for the reverse-engineered Zalo web protocol.

> **Unofficial project.** This library is not affiliated with, endorsed by, or supported by Zalo. It may violate Zalo's terms of service, stop working without notice, and can put accounts or data at risk. Use it only if you understand those risks.

## Status

`zalo-client` is an early Rust rewrite (v0.1.0) of the previous TypeScript/Node codebase. The crate lives at the repository root and is built with Cargo.

Current implementation includes:

- Login flow with browser-exported credentials (`ZaloClient::login` / `login_from_browser`)
- Cookie-aware HTTP client built on `reqwest` and `cookie_store`
- Request signing and AES-CBC parameter encryption/decryption helpers
- Encrypted response handling for API calls
- Broad API surface exposed through `ZaloApi` (158 endpoint modules under `src/api/`, excluding `client.rs`, `mod.rs`, and `url.rs`)
- Multipart upload support for `upload_attachment` and `upload_product_photo`
- WebSocket listener for real-time events (`api.listener()` / `api.create_listener()`), with binary framing `[version, cmd (LE u16), subCmd] + JSON payload`, event channel delivery, reconnect behavior, and cipher-key handshake handling
- Low-level QR login helpers in `src/api/login_qr.rs` (caller-driven flow; not a complete interactive UI)

This project does not claim full protocol parity, production readiness, or complete event decryption.

## Requirements

- Rust stable (edition 2021)
- Tokio runtime for async APIs

## Installation

This crate is not published to crates.io from this repository. Add it as a Git or path dependency:

```toml
[dependencies]
zalo-client = { git = "https://github.com/DEVTRAN07/Zalo-Client" }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

For local development:

```toml
[dependencies]
zalo-client = { path = "../zalo-client" }
```

## Quick start

```rust
use zalo_client::{Credentials, ZaloClient, ZaloCookie};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookies: Vec<ZaloCookie> = serde_json::from_str(&std::env::var("ZALO_COOKIE_JSON")?)?;

    let credentials = Credentials {
        imei: std::env::var("ZALO_IMEI")?,
        cookie: cookies,
        user_agent: std::env::var("ZALO_USER_AGENT")?,
        language: Some("vi".into()),
    };

    let client = ZaloClient::new(Default::default());
    let api = client.login(credentials).await?;

    let uid = api.uid().to_string();
    let info = api.get_user_info(&[uid.as_str()]).await?;
    println!("{}", serde_json::to_string_pretty(&info)?);

    Ok(())
}
```

### Environment variables for the example

```bash
export ZALO_IMEI="your-imei"
export ZALO_USER_AGENT="Mozilla/5.0 ..."
export ZALO_COOKIE_JSON='[{"name":"...","value":"...","domain":"chat.zalo.me"}]'
cargo run --example login
```

## API usage notes

- Most endpoint methods are async and return `Result<...>`.
- `send_message` currently takes `(message, thread_id, thread_type: u8)`. The `ThreadType` enum is available under `zalo_client::models::r#enum::ThreadType` if you prefer typed values.
- `ZaloApi` also exposes helpers such as `uid()`, `context()`, `api_client()`, `get_own_id()`, `get_cookie()`, and `get_context()`.
- Endpoint payload/response types are re-exported from `zalo_client::api` where applicable.

## WebSocket listener

```rust
use zalo_client::{Credentials, ZaloClient};

# async fn demo(api: zalo_client::ZaloApi) -> Result<(), Box<dyn std::error::Error>> {
let mut listener = api.listener(); // or: api.create_listener()
let mut rx = listener.start()?;

while let Some(event) = rx.recv().await {
    println!("event: {:?}", event);
}
# Ok(())
# }
```

WebSocket behavior is based on the observed Zalo web protocol and may change. Treat event payloads as protocol data, not stable API contracts.

## QR login

`src/api/login_qr.rs` contains low-level helpers for the QR login flow (login page/version discovery, QR generation, scan/confirm polling, session/user info checks). It is not a packaged interactive login UI; callers are responsible for presenting the QR and driving the flow.

## Project structure

```text
.
├── Cargo.toml
├── examples/
│   └── login.rs
├── src/
│   ├── api/          # Endpoint modules and ZaloApi surface
│   ├── core/
│   ├── models/
│   ├── ws/           # WebSocket listener and framing
│   ├── client.rs     # ZaloClient login entry point
│   ├── context.rs
│   ├── crypto.rs
│   ├── errors.rs
│   ├── factory.rs
│   ├── http.rs
│   ├── lib.rs
│   └── logger.rs
└── tests/
    ├── encryption_test.rs
    ├── login_flow_test.rs
    ├── url_resolution_test.rs
    └── ws_framing_test.rs
```

## Development

```bash
cargo test
cargo build
cargo fmt
cargo clippy --all-targets --all-features
```

Current tests include integration tests and a doctest (`cargo test` reports 18 passing tests in the current workspace state).

## Security and responsible use

Read [SECURITY.md](./SECURITY.md) before using or reporting issues. Do not commit credentials, cookies, IMEI values, or private tokens. Report security concerns privately as described in the security policy.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Contributions should be narrowly scoped, tested when practical, and careful about protocol/account risk.

## License

MIT. See [LICENSE](./LICENSE).
