# Security Policy

## Supported Versions

`zalo-client` is currently an early Rust crate (`0.1.x`) under active development in this repository.

| Version | Supported |
| --- | --- |
| 0.1.x (current development line) | Best effort |
| < 0.1.0 | No |

Only the latest code on the default branch should be considered for security fixes unless maintainers state otherwise.

## Reporting a Vulnerability

Do **not** open a public GitHub issue for security vulnerabilities, credential leaks, or exploit details.

Please report privately with:

- a clear description of the issue
- affected files, versions, or commits
- reproduction steps or proof of concept (if available)
- impact assessment
- any suggested mitigation

If the repository has GitHub private vulnerability reporting enabled, use that channel first. Otherwise, contact the maintainers through GitHub privately and include `[SECURITY] zalo-client` in the subject or message.

### Response expectations

- Initial response: best effort within 48 hours
- Triage/update: best effort within 7 days
- Fix timeline: depends on severity, reproducibility, and maintainer availability

## Security Considerations

This is an unofficial, reverse-engineered client for Zalo's web protocol.

- **Account risk:** Using this library may violate Zalo terms and can result in rate limiting, session invalidation, suspension, or account bans.
- **Credential sensitivity:** Cookies, IMEI values, user-agent strings, tokens, and session keys are sensitive. Treat them as secrets.
- **Protocol volatility:** Zalo endpoints, encryption details, and behavior may change without notice, which can create correctness or security issues.
- **No official support:** There is no guarantee of safety, availability, or compatibility.

## Best Practices for Users

- Store credentials in environment variables or a secret manager, never in source control.
- Use dedicated test accounts when experimenting.
- Keep dependencies updated (`cargo update` / review `Cargo.lock` changes).
- Validate and log carefully; avoid logging raw cookies, tokens, or encrypted payload keys.
- Review protocol-facing changes before deploying automation.

## Best Practices for Contributors

- Never commit real credentials, cookies, QR/session artifacts, or personal data.
- Add regression tests for crypto, framing, signing, and parsing changes when feasible.
- Avoid widening request/response types with guessed fields.
- Call out account risk and behavior changes in pull requests.
- Do not add telemetry or hidden network behavior.

## Security Updates

Security fixes, when available, will be documented in [CHANGELOG.md](./CHANGELOG.md) and released through the repository's normal Git/Cargo workflow.

## Acknowledgments

We appreciate responsible disclosure. Reporters may be credited in advisories or release notes unless they request anonymity.
