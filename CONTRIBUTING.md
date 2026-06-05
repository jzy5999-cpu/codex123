# Contributing to codex123

Thank you for your interest in codex123.

codex123 is an independent, unofficial, MIT-licensed project derived from
CodexPlusPlus. It is primarily built for personal Codex development workflows
and is open-sourced so people with similar needs can study, reference, and use
it at their own risk.

## Project Scope

- Keep the project focused on improving local Codex development experience.
- Preserve the official ChatGPT login state whenever remote-control-compatible
  relay mode is involved.
- Do not modify the original Codex App installation files or `app.asar`.
- Keep enhancement behavior in the external launcher, manager, and Chromium
  DevTools Protocol injection path.
- macOS Apple Silicon is the primary supported platform. Do not add Windows or
  Linux release work unless the maintainer explicitly requests it.

## What Not To Submit

- Do not submit API keys, tokens, account emails, private relay URLs, local
  absolute paths with personal data, or logs containing secrets.
- Do not submit changes intended for attacks, abuse, account-permission bypass,
  or evading platform safety mechanisms.
- Do not remove upstream attribution to CodexPlusPlus or other referenced
  open-source projects.
- Do not vendor large generated assets, local caches, build outputs, or video
  project working directories unless they are explicitly requested.

## Development Setup

```bash
git clone https://github.com/jzy5999-cpu/codex123.git
cd codex123
cd apps/codex-plus-manager
npm ci
cd ../..
cargo build --release
```

## Common Checks

Run the checks that match your change scope:

```bash
cd apps/codex-plus-manager
npm run check
npm run vite:build
cd ../..
cargo fmt --check
cargo test
git diff --check
```

Before publishing or opening a PR, also scan for secrets:

```bash
rg -n "sk-|OPENAI_API_KEY|access_token|refresh_token|Bearer"
```

## Change Guidelines

- Prefer small, reviewable changes.
- Follow the existing Rust/Tauri/React structure.
- Add or update tests when behavior changes.
- Keep documentation in `README.md`, `README_EN.md`, `NOTICE`, and
  `UPSTREAM.md` consistent when the project position, attribution, or release
  behavior changes.
- Treat upstream CodexPlusPlus changes as selective imports. Review the
  behavior and branding impact before copying code.

## Pull Requests

Use a clear title and include:

- What changed.
- Why it is needed.
- What was tested.
- Whether the change affects macOS packaging, relay configuration, CDP
  injection, Petdex, or official ChatGPT login state.

## License

By contributing, you agree that your contribution is licensed under the MIT
License used by this project.
