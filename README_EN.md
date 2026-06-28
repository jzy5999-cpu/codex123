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

> In one sentence: if you use Codex on macOS, want model requests to go through a relay API, and still want to preserve the prerequisites for ChatGPT mobile remote control, codex123 is built for that workflow.

## Download

The primary release target is macOS Apple Silicon:

[Download the latest macOS DMG](https://github.com/jzy5999-cpu/codex123/releases/latest)

After installation, you get two entry points:

- `codex123`: silent launcher for starting Codex and injecting enhancements through CDP.
- `codex123 管理工具`: manager UI for providers, relay setup, enhancements, diagnostics, and updates.

## Who It Is For

- You use Codex on a Mac and want model requests to go through a relay API or third-party compatible API.
- You want to preserve the official ChatGPT login state instead of turning Codex into a pure API-key mode.
- You need DeepSeek or Chat Completions upstream compatibility.
- You want external launcher + CDP injection enhancements without modifying the original Codex App files or `app.asar`.

## Highlights

- **Remote-control-compatible relay mode**: stores the relay key in provider config, not in `auth.json` as `OPENAI_API_KEY`.
- **Remote-control prerequisite diagnostics**: checks ChatGPT auth state, `auth_mode`, provider config, `base_url`, `wire_api`, `requires_openai_auth`, and bearer token.
- **DeepSeek / Chat Completions compatibility**: local proxy converts Codex Responses requests to Chat Completions and converts upstream responses back.
- **Optional paste fix**: pasting rich text from Word and similar sources into the Codex composer can be forced to plain text to reduce accidental image/file attachment detection.
- **macOS Computer Use cleanup**: periodically removes orphaned `SkyComputerUseClient` subprocesses to reduce memory pressure during long Codex sessions.
- **codex123 pet source**: ship 10 Codex-compatible pet packages generated from open-source Noto Emoji image resources and install them into `~/.codex/pets`.
- **External enhancement injection**: launcher + Chromium DevTools Protocol, without patching the Codex App installation.
- **Installable macOS app**: Apple Silicon DMG with a silent launcher and a manager entry point.

## Docs

- [FAQ](docs/FAQ.md): common questions about remote-control-compatible relay mode, DeepSeek, macOS, and Windows scope.
- [Promotion copy](docs/PROMOTION.md): ready-to-post Chinese copy for V2EX, Jike, X, Zhihu, and other communities.
- [Outreach checklist](docs/OUTREACH.md): GitHub topics, community channels, and messaging priorities.

## Current Limits

- macOS Apple Silicon is the only actively maintained release target.
- Windows is kept only as a development build and CI artifact, not verified on a real Windows machine.
- Whether ChatGPT mobile can see and remotely control Codex is ultimately controlled by OpenAI account eligibility and feature availability.
- Relay providers must support `/v1/responses`, or be compatible through the local Chat Completions proxy.

This project is a tribute to and derivative of [CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus). CodexPlusPlus provided the external launcher, manager, CDP injection approach, installer structure, and much of the foundation that made this project possible. codex123 adds a remote-control-compatible relay mode and focuses the first deliverable on macOS Apple Silicon. It also learns from other open-source tooling practices, especially [ccswitch](https://github.com/farion1231/cc-switch)'s local routing and protocol conversion approach for connecting Codex to DeepSeek / Chat Completions upstreams, and prioritizes personal development workflows.

codex123 is primarily built for the author's personal use and is open-sourced so people with similar needs can study, reference, and use it at their own risk. It is not intended for attacks, abuse, account-permission bypass, or evading platform safety mechanisms. It only enhances local configuration, launcher behavior, and CDP injection externally, with the goal of improving development experience while preserving the official ChatGPT login state where possible.

codex123 is not an official OpenAI project and is not affiliated with, sponsored by, or endorsed by OpenAI. Codex, ChatGPT, and related names belong to their respective owners.

## Current Scope

- macOS Apple Silicon is the primary supported platform. Windows x64 is provided only as a development build and CI artifact for now.
- The developer uses a Mac, so the Windows build has not been verified on a real Windows machine and is not guaranteed to work.
- macOS uses local builds and ad-hoc signing without Apple Developer ID notarization. Windows uses an NSIS installer and is not code-signed yet.
- The ChatGPT mobile remote-control entry point, account eligibility, and remote session availability are controlled by OpenAI; codex123 cannot guarantee 100% availability.
- This project only aims to keep local configuration from breaking the official ChatGPT login state while keeping relay requests and CDP injection enhancements usable together.

## Quick Start

Release assets:

- macOS Apple Silicon: `codex123-*-macos-arm64.dmg`
- Windows x64: `codex123-*-windows-x64-setup.exe` (development build, not real-machine verified)

After installation, two entry points are available:

- `codex123`: a silent launcher. It does not show the manager UI and only starts Codex with codex123 injection.
- `codex123 Manager`: a Tauri control panel for launch, diagnostics, repair, updates, relay injection, enhancements, and user scripts.

The macOS DMG installs `/Applications/codex123.app` and `/Applications/codex123 管理工具.app`. The Windows setup installs `codex123.exe` and `codex123-manager.exe`, and creates `codex123` and `codex123 管理工具` shortcuts.

Build a local macOS installer from source:

```bash
cd /Users/jiangzengyan/Downloads/codex/codex123
cd apps/codex-plus-manager
npm ci
npm run vite:build
cd ../..
cargo build --release
BINARY_DIR="$PWD/target/release" bash scripts/installer/macos/package-dmg.sh 0.2.9 arm64
```

The generated file is `dist/macos/codex123-0.2.9-macos-arm64.dmg`. This first version uses ad-hoc signing and is not notarized with an Apple Developer ID.

Windows packaging is kept as a development build path and is not updated by macOS fix releases. Because the developer uses a Mac, the Windows build currently only covers code and packaging scripts; launch behavior, CDP injection, and remote-control-compatible relay behavior have not been verified on a real Windows environment. Local Windows builds require the Rust MSVC toolchain, Node.js 22, and NSIS:

```powershell
cd C:\path\to\codex123
cd apps\codex-plus-manager
npm ci
npm run vite:build
cd ..\..
cargo build --release --target x86_64-pc-windows-msvc
New-Item -ItemType Directory -Force dist\windows\app
Copy-Item target\x86_64-pc-windows-msvc\release\codex123.exe dist\windows\app\codex123.exe
Copy-Item target\x86_64-pc-windows-msvc\release\codex123-manager.exe dist\windows\app\codex123-manager.exe
cd scripts\installer\windows
makensis /DVERSION=0.2.2 codex123.nsi
```

The Windows installer path is kept at `dist/windows/codex123-0.2.2-windows-x64-setup.exe` for the last development build. Windows is not updated by macOS-only fix releases unless explicitly requested.

## DeepSeek / Chat Completions Compatibility

Codex currently sends model requests in the OpenAI Responses API shape, while DeepSeek's official API and many relay providers expose an OpenAI Chat Completions-compatible interface. codex123 borrows ccswitch's Codex DeepSeek routing design: the local protocol proxy converts Codex Responses requests into Chat Completions requests, then converts upstream responses back into the Responses shape.

Recommended DeepSeek setup:

1. Create a new provider in the manager.
2. Set `Base URL` to DeepSeek or a compatible relay, such as `https://api.deepseek.com` or the relay's `/v1` root.
3. Choose `Chat Completions` as the upstream protocol.
4. When using remote-control-compatible relay mode, keep `wire_api = "responses"` and start Codex from the `codex123` entry point.

The current compatibility layer includes DeepSeek reasoning effort mapping, `reasoning_content` streaming conversion, a fallback `reasoning_content` for assistant tool-call history, and basic tool-call history conversion. This improves DeepSeek stability for long sessions and tool use, but it does not guarantee that every DeepSeek relay will work; the relay still needs compatible Chat Completions, streaming, and tool-call behavior.

## codex123 Pet Import

The manager includes a built-in pet source for installing Codex-compatible pet packages into the user data directory:

```text
~/.codex/pets/<slug>/
├── pet.json
└── spritesheet.webp or spritesheet.png
```

How to use it:

1. Open `codex123 管理工具`.
2. Go to `宠物导入`.
3. Click `刷新宠物源`.
4. Browse the list sorted by local heat score, then search and install the pet you want.
5. Open Codex and choose it manually in `Settings -> Appearance -> Pets`.

This feature only writes to `~/.codex/pets`. It does not modify the original Codex App installation and does not automatically change Codex's internal selected-pet state. Before installation, codex123 validates HTTPS, allowed asset hosts, slug safety, JSON format, PNG/WEBP spritesheets, file size limits, and uses a temporary directory for atomic installation. Each install writes `codex123-installed.json` metadata for update detection, and the local installed list can delete valid slug directories.

The default pet source is `codex123 Curated Pets`, currently containing 10 pet packages generated from [Google Noto Emoji](https://github.com/googlefonts/noto-emoji) image resources: cat, dog, fox, panda, rabbit, penguin, owl, hamster, unicorn, and dragon. The Noto Emoji README states that tools and most image resources are under the Apache License 2.0; codex123 keeps source, license, and packaging notes in `assets/pets/ATTRIBUTION.md` and each pet directory README.

The pet list currently uses a local `codex123` composite heat score so it is easier to pick fuller or already-maintained pets first. This is not a real download count, like count, or popularity metric. The Petdex website is no longer the default source; any future third-party source must have redistribution-friendly licensing before being bundled.

## Open Source and Thanks

codex123 is released under the MIT License. See [UPSTREAM.md](UPSTREAM.md) for upstream source and local change notes, and [NOTICE](NOTICE) for attribution and disclaimers.

Special thanks to [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus). Without CodexPlusPlus exploring the external launcher, manager, CDP injection, and installer experience, codex123 would not have taken shape this quickly.

Thanks also to [ccswitch](https://github.com/farion1231/cc-switch) for its local routing and protocol conversion work around Codex with DeepSeek / Chat Completions upstreams. codex123's DeepSeek compatibility improvements reference that design.

## Highlights

- Rust backend and silent launcher with no extra runtime requirement.
- Tauri + React manager with dark/light theme support.
- External CDP injection. No `app.asar` patching and no DLL writes into the Codex installation.
- Relay injection mode with multiple relay profiles, `codex123` provider configuration, and a one-click switch back to official ChatGPT login mode.
- Traditional enhancement mode that selects plugin marketplace unlock or plugin entry unlock by Codex App version, plus forced plugin install, session delete, Markdown export, project move, Timeline, and more.
- Independent user script management with startup injection.
- Provider Sync to keep historical sessions visible after switching providers, with either the current `config.toml` target or a manually selected provider id.
- Zed open entry detects remote SSH context and opens the matching remote file in Zed Remote Development from Codex.
- Upstream worktree creation: create new worktrees from `upstream/<base-branch>` after fetching the remote branch, reducing conflicts caused by stale local HEAD state.
- GitHub Release updates. Both the manager and silent launcher can detect available updates.
- Windows single instance, no console window, administrator manifest, NSIS installer, and system Desktop path detection.
- macOS arm64 DMG. The silent launcher hides its Dock icon.

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

In traditional enhancement mode, plugin-related controls are split into three independent switches:

- **Plugin marketplace unlock**: for newer Codex App versions, patching marketplace filtering, hidden lists, and install requests.
- **Force plugin entry unlock**: for older Codex App versions, forcing the sidebar plugin entry to show.
- **Special plugin forced install**: lifts frontend `App unavailable` / unavailable-state disabled install buttons.

The manager shows the detected Codex App version. The injected script then automatically chooses the legacy entry strategy or the modern marketplace strategy. If the version cannot be detected, codex123 conservatively tries both strategies and records a `plugin_unlock_strategy_selected` diagnostic event.

## Updates and Packages

codex123 publishes installers through GitHub Releases. Windows builds an NSIS x64 installer, while macOS builds an Apple Silicon arm64 DMG. The Windows installer is currently a development build and has not been real-machine verified.

The manager's About page can check and start updates. When the silent launcher finds a new version, it opens the manager directly on the update prompt.

## Data Locations

- Codex config: `~/.codex/config.toml`
- Codex auth state: `~/.codex/auth.json`
- Codex local database: `~/.codex/state_5.sqlite`
- codex123 state and logs: `~/.codex123/`
- Legacy compatibility: if `~/.codex123/settings.json` does not exist, the manager reads `~/.codex-session-delete/settings.json`; saving writes to the new path.
- Provider Sync backups: `~/.codex/backups_state/provider-sync`

## Provider Sync Target

Provider Sync uses the `model_provider` from `~/.codex/config.toml` by default and syncs historical session ownership to that provider. The manager's Historical Session Repair page shows both the automatic target and the target that will be used for the next repair.

When you need to repair history into a specific provider, switch to manual target mode and enter `openai`, `codex123`, or another provider id. A backup is created before rewriting data under `~/.codex/backups_state/provider-sync`.

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

### Does it support Windows?

There is a Windows x64 development build, but actual usability is not guaranteed yet. The developer uses a Mac, so the Windows installer is generated by GitHub Actions and has not been verified on a real Windows machine for launch behavior, CDP injection, shortcuts, uninstall behavior, or remote-control-compatible relay mode. The first Windows installer is not code-signed, so SmartScreen may require manual confirmation.

### Does it support Intel Macs?

The current release workflow only publishes `macos-arm64.dmg`. Intel Macs are not the first-priority release target.

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


## Notes

codex123 is an external enhancement tool and does not modify original Codex App files. If a future Codex App update changes page structure, the injection script may need updates.
