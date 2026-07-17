# OpenAI Build Week Submission Notes

Project: codex123

Recommended track: Developer Tools

Repository: https://github.com/jzy5999-cpu/codex123

Test build: https://github.com/jzy5999-cpu/codex123/releases/tag/v0.2.23

## What codex123 does

codex123 is an unofficial external launcher and manager for the Codex desktop app on macOS Apple Silicon. It improves the daily developer workflow around Codex without modifying the original Codex app files.

The project focuses on four practical problems:

- preserving the official ChatGPT login state while using relay or third-party compatible model providers;
- diagnosing the prerequisites for ChatGPT remote control of Codex sessions;
- improving compatibility with Responses API and Chat Completions style upstreams;
- adding external Codex enhancements through a launcher, manager UI, and Chromium DevTools Protocol injection.

## Build Week extension

codex123 existed before the OpenAI Build Week submission period. The Build Week submission should be evaluated on the work added during the submission period.

The main qualifying extension is release `v0.2.23`, committed on 2026-07-14:

- script market requests now use the codex123 system-proxy-aware HTTP client;
- manifest and script downloads have explicit timeouts and bounded retries;
- the manager saves the latest valid script market manifest and falls back to the local cache when GitHub Raw is temporarily unavailable;
- installing or re-enabling scripts attempts a hot reload into the current Codex session;
- the manager adds a dedicated reload action for currently enabled scripts;
- disabling or deleting a script now tells the user that a restart is required to fully remove UI effects already created by that script.

Evidence:

- Commit: `3ffe74b fix: make script market resilient and reloadable`
- Tag: `v0.2.23`
- Release date: 2026-07-14
- Release asset: `codex123-0.2.23-macos-arm64.dmg`

## How Codex and GPT-5.6 were used

Codex was used as the implementation environment for inspecting the existing Rust, Tauri, and React code paths, narrowing the Build Week scope to a real reliability problem, editing the manager and core script-market logic, and running validation commands.

GPT-5.6 should be described in the Devpost form and demo narration as the reasoning model used with Codex to:

- compare the existing script-market behavior with the desired failure modes;
- decide to add bounded retries, system proxy support, and last-known-good caching instead of only increasing timeouts;
- update the user-facing manager flow so enabled scripts can be reloaded into the current session;
- keep the project within the existing macOS Apple Silicon and external-enhancement boundaries.

Use the `/feedback` command in the Codex thread where the majority of the Build Week work was done, then paste that session ID into the Devpost submission form.

## Installation and testing

Judges can test without rebuilding from source:

1. Download `codex123-0.2.23-macos-arm64.dmg` from the release page.
2. Install the two apps into `/Applications`.
3. Start `codex123 Manager`.
4. Open the Enhancements or script market area.
5. Refresh the script market, enable a script, and use the reload action to load enabled scripts into the current Codex session.

Supported platform:

- macOS Apple Silicon.

Known limits:

- The app is ad-hoc signed and not Apple Developer ID notarized.
- Windows builds are development artifacts and are not the Build Week test target.
- ChatGPT mobile remote-control availability depends on OpenAI account eligibility and official feature rollout state.

## Local validation performed

The following checks passed before preparing this submission note:

```bash
cargo fmt --check
cd apps/codex-plus-manager && npm run check
cargo test -p codex-plus-core script_market -- --nocapture
git diff --check
```

Sensitive-field scanning found only documentation references and test fixtures using fake tokens such as `sk-test`; no real API keys were found in the repository scan.

## Suggested Devpost description

codex123 is an unofficial macOS Apple Silicon launcher and manager for the Codex desktop app. It helps developers keep their official ChatGPT login state while configuring relay or third-party compatible model providers, and it adds external Codex workflow enhancements without modifying the original Codex app files.

For Build Week, I extended codex123's script market reliability. The new release uses a system-proxy-aware request client, adds explicit timeouts and bounded retries, saves the last valid script manifest for offline fallback, and adds a reload flow so newly enabled scripts can be applied to the current Codex session when possible. This turns a fragile network-dependent feature into a more dependable developer workflow inside Codex.

I used Codex with GPT-5.6 to inspect the existing Rust/Tauri/React implementation, choose the reliability strategy, implement the script-market changes, update the manager UI behavior, and validate the changes with formatting, frontend type checks, focused Rust tests, and release checks.

## Suggested demo video outline

Keep the video under three minutes.

1. Show the problem: the script market depends on remote manifest and script downloads, which can fail under proxy or temporary GitHub Raw issues.
2. Show codex123 Manager and the script market page.
3. Demonstrate refresh, cached fallback messaging, enabling a script, and the reload action.
4. Explain that codex123 is external to the Codex app and does not modify `app.asar` or original app files.
5. Mention Codex/GPT-5.6 usage: scope selection, implementation, UI decision, and validation.
