# zalo-client (Rust)

Rust port of the [zalo-client](../) TypeScript library — a reverse-engineered client for the Zalo web API protocol.

## Features

- **Crypto**: MD5 signing, AES-CBC (zero IV, PKCS7) parameter encryption/decryption, `ParamsEncryptor` for login flows
- **HTTP**: Cookie-aware `reqwest` client with retry (exponential backoff + jitter) and timeout
- **Login**: `getLoginInfo` / `getServerInfo` with encrypted params
- **API**: `get_user_info`, `keep_alive`, `send_message` (text, user/group)
- **WebSocket**: Binary protocol listener for real-time events (messages, reactions, control, seen/delivered)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
zalo-client = { path = "../rust-client" }
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use zalo_client::{Credentials, ZaloClient, ZaloCookie};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials {
        imei: "your-imei".into(),
        cookie: vec![/* ZaloCookie objects */],
        user_agent: "Mozilla/5.0 ...".into(),
        language: Some("vi".into()),
    };

    let client = ZaloClient::new(Default::default());
    let api = client.login(credentials).await?;

    println!("Logged in as {}", api.uid());

    let info = api.get_user_info(&[api.uid()]).await?;
    println!("{:?}", info);

    // WebSocket listener
    let mut listener = api.listener();
    let mut rx = listener.start()?;
    while let Some(event) = rx.recv().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
```

## Protocol notes

- Base API: `https://wpa.chat.zalo.me/api/login/...`
- API version/type query params: `zpw_ver=685`, `zpw_type=30`
- Sign key: `MD5("zsecure" + type + sorted param values)`
- AES: CBC mode, 16-byte zero IV, PKCS7 padding
- Response `data` is AES-encrypted with the session `secretKey` (`zpw_enk`)
- WebSocket frames: 4-byte header `[version, cmd(LE u16), subCmd]` + JSON payload

## Status

Core protocol (crypto, login, basic API, WS framing) is implemented. Many endpoints
from the TypeScript client are not yet ported — contributions welcome.

## License

MIT
