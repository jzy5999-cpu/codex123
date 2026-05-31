# codex123

<p align="center">
  <a href="README.md">中文</a> | English
</p>

<p align="center">
  <img alt="Release" src="https://img.shields.io/github/v/release/jzy5999-cpu/codex123">
  <img alt="Stars" src="https://img.shields.io/github/stars/jzy5999-cpu/codex123">
  <img alt="License" src="https://img.shields.io/github/license/jzy5999-cpu/codex123">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Tauri" src="https://img.shields.io/badge/tauri-2.x-24C8DB">
</p>

codex123 is an independent, unofficial, development-experience-first external launcher and manager for the Codex App. Its goal is to make Codex easier to use: preserve the official ChatGPT login state, reduce the risk of relay configuration breaking remote-control prerequisites, and keep the external launcher plus Chromium DevTools Protocol injection path without modifying the original Codex App installation files.

This project is a tribute to and derivative of [CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus). CodexPlusPlus provided the external launcher, manager, CDP injection approach, installer structure, and much of the foundation that made this project possible. codex123 adds a remote-control-compatible relay mode and focuses the first deliverable on macOS Apple Silicon. It also learns from other open-source tooling practices and prioritizes personal development workflows.

codex123 is not an official OpenAI project and is not affiliated with, sponsored by, or endorsed by OpenAI. Codex, ChatGPT, and related names belong to their respective owners.

## Current Scope

- Only macOS Apple Silicon is supported for now.
- The first version uses local builds and ad-hoc signing, without Apple Developer ID notarization.
- The ChatGPT mobile remote-control entry point, account eligibility, and remote session availability are controlled by OpenAI; codex123 cannot guarantee 100% availability.
- This project only aims to keep local configuration from breaking the official ChatGPT login state while keeping relay requests and CDP injection enhancements usable together.

## Quick Start

The first codex123 build targets Apple Silicon MacBook only:

- macOS Apple Silicon: `codex123-*-macos-arm64.dmg`

After installation, two entry points are available:

- `codex123`: a silent launcher. It does not show the manager UI and only starts Codex with codex123 injection.
- `codex123 Manager`: a Tauri control panel for launch, diagnostics, repair, updates, relay injection, enhancements, and user scripts.

The macOS DMG installs `/Applications/codex123.app` and `/Applications/codex123 管理工具.app`.

Build a local macOS installer from source:

```bash
cd /Users/jiangzengyan/Downloads/codex/codex123
cd apps/codex-plus-manager
npm ci
npm run vite:build
cd ../..
cargo build --release
BINARY_DIR="$PWD/target/release" bash scripts/installer/macos/package-dmg.sh 0.1.3 arm64
```

The generated file is `dist/macos/codex123-0.1.3-macos-arm64.dmg`. This first version uses ad-hoc signing and is not notarized with an Apple Developer ID.

## Open Source and Thanks

codex123 is released under the MIT License. See [UPSTREAM.md](UPSTREAM.md) for upstream source and local change notes, and [NOTICE](NOTICE) for attribution and disclaimers.

Special thanks to [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus). Without CodexPlusPlus exploring the external launcher, manager, CDP injection, and installer experience, codex123 would not have taken shape this quickly.

## Highlights

- Rust backend and silent launcher with no extra runtime requirement.
- Tauri + React manager with dark/light theme support.
- External CDP injection. No `app.asar` patching and no DLL writes into the Codex installation.
- Relay injection mode with multiple relay profiles, `codex123` provider configuration, and a one-click switch back to official ChatGPT login mode.
- Traditional enhancement mode with plugin entry unlock, forced plugin install, session delete, Markdown export, project move, Timeline, and more.
- Independent user script management with startup injection.
- Provider Sync to keep historical sessions visible after switching providers.
- Zed open entry detects remote SSH context and opens the matching remote file in Zed Remote Development from Codex.
- Upstream worktree creation: create new worktrees from `upstream/<base-branch>` after fetching the remote branch, reducing conflicts caused by stale local HEAD state.
- GitHub Release updates. Both the manager and silent launcher can detect available updates.
- Windows single instance, no console window, administrator manifest, and system Desktop path detection.
- Separate macOS x64 and arm64 DMGs. The silent launcher hides its Dock icon.

## Relay Injection

Relay injection is for users who are already logged in with an official ChatGPT account in Codex/ChatGPT and want model requests to go through a custom compatible API.

In the manager's Relay Injection page:

1. Make sure ChatGPT login status is detected.
2. Add one or more relay profiles with Base URL and Key.
3. Select the active profile and apply relay injection.
4. Launch `codex123`.

codex123 writes configuration similar to this into `~/.codex/config.toml`:

```toml
model_provider = "codex123"

[model_providers.codex123]
name = "codex123"
wire_api = "responses"
requires_openai_auth = true
base_url = "https://example.com/v1"
experimental_bearer_token = "sk-..."
```

To return to the official login mode, use the clear API mode button in the Relay Injection page. This removes `OPENAI_API_KEY` related configuration and switches Codex back to official ChatGPT authentication.

## Enhancements

Enhancements are controlled in the manager. Enhancement injection is enabled by default. When disabled, codex123 will not inject its menu or scripts.

When relay injection mode is active, plugin entry unlock and forced plugin install are unnecessary, and the UI will say so. Other enhancements, including session delete, export, move, Timeline, and user scripts, can still be used.

## Updates and Packages

codex123 publishes installers through GitHub Releases. Windows builds an NSIS installer, while macOS builds separate Intel x64 and Apple Silicon arm64 DMGs.

The manager's About page can check and start updates. When the silent launcher finds a new version, it opens the manager directly on the update prompt.

## Data Locations

- Codex config: `~/.codex/config.toml`
- Codex auth state: `~/.codex/auth.json`
- Codex local database: `~/.codex/state_5.sqlite`
- codex123 state and logs: `~/.codex123/`
- Legacy compatibility: if `~/.codex123/settings.json` does not exist, the manager reads `~/.codex-session-delete/settings.json`; saving writes to the new path.
- Provider Sync backups: `~/.codex/backups_state/provider-sync`

## FAQ

### The codex123 menu does not appear

Make sure Codex was launched from the `codex123` entry instead of the original Codex entry. You can also inspect the Diagnostics and Logs pages in the manager.

### The plugin says the backend is disconnected

First test the helper endpoint:

```powershell
Invoke-RestMethod -Method Post -Uri http://127.0.0.1:57321/backend/status -Body "{}" -ContentType "application/json"
```

If the endpoint works but the plugin still times out, it is usually a Codex page CDP bridge or script cache issue. Restart codex123, or check manager logs for `renderer.script_loaded`, `bridge.request`, and `bridge.response`.

### How is Upstream worktree different from Codex native creation?

codex123 updates the remote branch first, then creates the worktree as if you ran:

```bash
git worktree add -b <new-branch> <worktree-path> upstream/<base-branch>
```

The new worktree starts from the fresh remote tracking branch instead of the local HEAD used by the current session. If codex123 cannot safely recognize the current Codex version's native worktree form, use the codex123 menu entry and enter the repository path, branch name, worktree path, remote, and base branch manually.

### macOS says the app cannot be opened or is damaged

Unsigned and unnotarized builds may be blocked by Gatekeeper. Allow the app in System Settings -> Privacy & Security. For formal distribution, configure Apple Developer ID signing and notarization.

### Does it support Intel Macs?

Yes. Releases provide both `macos-x64.dmg` and `macos-arm64.dmg`. Intel Macs should use the x64 package, while Apple Silicon Macs should use the arm64 package.

## Development

```bash
# Frontend checks
cd apps/codex-plus-manager
npm install
npm run check
npm run vite:build

# Rust checks
cd ../..
cargo fmt --check
cargo test
cargo build --release
```

Project structure:

```text
apps/
  codex-plus-launcher/          Silent launcher
  codex-plus-manager/           Tauri manager
assets/inject/
  renderer-inject.js            Enhancement script injected into Codex
crates/
  codex-plus-core/              Launch, injection, config, update, install, bridge
  codex-plus-data/              Session data, export, Provider Sync
scripts/installer/
  windows/codex123.nsi     Windows NSIS installer
  macos/package-dmg.sh          macOS DMG packager
```

## Friendly Links

- [LINUX DO](https://linux.do)

## Notes

codex123 is an external enhancement tool and does not modify original Codex App files. If a future Codex App update changes page structure, the injection script may need updates.
