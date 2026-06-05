# Security Policy

codex123 is an independent, unofficial local enhancement tool for Codex. It is
intended for personal development workflows and open-source study. It is not
intended for attacks, abuse, account-permission bypass, or evading platform
safety mechanisms.

## Supported Versions

The project currently focuses on the latest macOS Apple Silicon release.
Windows builds, when present, are development artifacts and are not considered
fully verified.

## Reporting Security Issues

Please report security-sensitive issues privately when possible. If GitHub
private vulnerability reporting is available for this repository, use it.
Otherwise, open a minimal public issue without secrets and state that you need
to share security details privately.

Do not include:

- API keys or relay keys.
- `auth.json` tokens.
- ChatGPT account emails.
- Private Base URLs.
- Full local paths containing personal information.
- Unredacted logs.

## Project Boundaries

codex123 does not modify the original Codex App installation files and does not
patch `app.asar`. Enhancements are applied through an external launcher,
manager configuration, and Chromium DevTools Protocol injection.

Remote-control-compatible relay mode is designed to avoid breaking the official
ChatGPT login state by keeping relay credentials out of `auth.json`
`OPENAI_API_KEY`. It cannot guarantee mobile ChatGPT remote-control access,
because account eligibility and remote session availability are controlled by
OpenAI.

## Sensitive Configuration

Before sharing diagnostics or opening an issue, redact:

```text
sk-...
OPENAI_API_KEY
access_token
refresh_token
id_token
Bearer ...
experimental_bearer_token
```

The manager diagnostics are designed to redact secrets, but manual review is
still required before posting logs publicly.
