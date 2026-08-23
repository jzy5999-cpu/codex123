# Upstream

`codex123` is based on the upstream CodexPlusPlus project.

- Upstream repository: https://github.com/BigPizzaV3/CodexPlusPlus
- Imported branch: `main`
- Imported commit: `67fa2a6b1de5a32455c0c64c2861caf9beb4ffe8`
- Imported at: `2026-05-31T14:10:57Z`

## Upstream Checks

- Last checked upstream `main`: `9c3daab`
- Last checked upstream release: `v1.2.52` (`4af66b8`)
- Checked at: `2026-08-23`
- Selection policy: codex123 reviews CodexPlusPlus changes case by case and
  imports only the parts that fit this project's macOS-first, personal-use,
  development-experience-focused scope.
- Selected imports from this check: Responses reasoning usage completion,
  lower-overhead plugin filtering, non-user thread isolation in Provider Sync,
  and delete/undo synchronization for `session_index.jsonl`.
- Deferred or excluded: Weixin connection, hosted session sharing, Stepwise,
  Dream Skin, VLM, Windows release changes, sponsor/community content, and
  broad upstream branding/runtime replacement.

## Local Changes

- Added an explicit `RemoteRelay` / "远控兼容中转" mode.
- Generated relay provider defaults now use `codex123` for new profiles.
- RemoteRelay keeps official ChatGPT auth state and writes relay credentials only to `config.toml`.
- RemoteRelay continues to use the upstream external launcher and Chromium DevTools Protocol injection path, without modifying Codex App original installation files.
- Rebranded the macOS installable app, bundle identifiers, updater metadata, and release asset names to `codex123`.
- Reduced first-party packaging to macOS Apple Silicon only for the initial release.

## Acknowledgement

CodexPlusPlus is the foundation of this project. codex123 keeps the upstream lineage explicit and thanks the original project for the external launcher, manager, CDP injection path, installation workflow, and practical Codex enhancement ideas.

codex123 is an independent project focused on better Codex development experience. It also borrows ideas from broader open-source developer tooling, while keeping its first supported platform limited to macOS Apple Silicon.
