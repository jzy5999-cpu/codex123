# Upstream

`codex123` is based on the upstream CodexPlusPlus project.

- Upstream repository: https://github.com/BigPizzaV3/CodexPlusPlus
- Imported branch: `main`
- Imported commit: `67fa2a6b1de5a32455c0c64c2861caf9beb4ffe8`
- Imported at: `2026-05-31T14:10:57Z`

## Local Changes

- Added an explicit `RemoteRelay` / "远控兼容中转" mode.
- Generated relay provider defaults now use `codex123` for new profiles.
- RemoteRelay keeps official ChatGPT auth state and writes relay credentials only to `config.toml`.
- RemoteRelay continues to use the upstream external launcher and Chromium DevTools Protocol injection path, without modifying Codex App original installation files.
