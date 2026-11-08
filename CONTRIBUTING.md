# Contributing to zalo-rs

`zalo-rs` is an unofficial Rust client for the reverse-engineered Zalo web protocol. Contributions are welcome, but changes should be careful: protocol behavior can change without notice and misuse can affect user accounts.

## Before you start

- Read [SECURITY.md](./SECURITY.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
- Search existing issues before opening a duplicate.
- For large protocol or API changes, open an issue first to discuss scope and risk.

## Development setup

Requirements:

- Rust stable toolchain (edition 2021)
- Cargo

Common commands:

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
cargo build
```

Run the example only with your own test credentials:

```bash
export ZALO_IMEI="..."
export ZALO_USER_AGENT="..."
export ZALO_COOKIE_JSON='[...]'
cargo run --example login
```

## Project layout

```text
src/
├── api/        # Endpoint modules and ZaloApi methods
├── core/       # Shared internals
├── models/     # Request/response/event models and enums
├── ws/         # WebSocket listener and framing
├── client.rs   # ZaloClient login entry point
├── context.rs  # Credentials/session context
├── crypto.rs   # Crypto/signing/encryption helpers
├── http.rs     # HTTP transport
├── errors.rs
├── factory.rs
├── lib.rs
└── logger.rs
tests/          # Integration tests
examples/       # Runnable examples
```

## Adding or changing an endpoint

The old TypeScript code generation flow does not apply.

Typical steps:

1. Add or update a module under `src/api/` (for example `src/api/some_endpoint.rs`).
2. Export/wire the module in `src/api/mod.rs`.
3. Add any request/response types needed by the endpoint. Keep types close to the protocol shape and avoid inventing undocumented fields.
4. Expose a `ZaloApi` method in `src/api/mod.rs` if the endpoint is part of the high-level API.
5. Add or update tests when the behavior can be tested without live credentials.
6. Run formatting and tests:

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## Testing guidelines

- Prefer deterministic unit/integration tests.
- Do not add tests that require real Zalo accounts, cookies, IMEI values, or private tokens.
- When touching crypto, framing, or URL/signing behavior, add focused regression tests.
- If a change depends on observed protocol behavior, describe the evidence and risk in the PR.

## Code style

- Follow standard Rust formatting (`cargo fmt`).
- Keep public API changes explicit.
- Prefer small, reviewable diffs.
- Avoid `unsafe` unless there is a clear reason and discussion.
- Do not add comments that only restate obvious code.

## Pull requests

- Keep PRs focused on one change.
- Include a clear description, motivation, and testing notes.
- Call out protocol assumptions, breaking API changes, and account/security risks.
- Never commit credentials, cookies, session data, or personal identifiers.

## Security issues

Do not open public issues for vulnerabilities or credential exposure. Follow [SECURITY.md](./SECURITY.md).

## License

By contributing, you agree that your contributions are licensed under the repository's MIT license.
