# codex123 Agent Instructions

请默认使用简体中文和 Markdown 输出。

## 项目定位

- `codex123` 是非官方 Codex App 外部增强工具，目标是更好地使用 Codex，开发经验优先。
- 本项目致敬并基于 `BigPizzaV3/CodexPlusPlus`，需要保留上游来源、致谢和独立项目声明。
- 当前第一版只面向 macOS Apple Silicon；不要默认补 Windows/Linux 支持，除非用户明确要求。
- 后续版本默认只更新 macOS 版；没有用户明确要求时，不更新 Windows 版本、Windows 安装包、Windows Release 资产或 Windows 发布流程。
- 不直接修改 Codex App 原始安装文件，不修改 `app.asar`；增强能力应继续走外部 launcher 和 Chromium DevTools Protocol 注入路径。

## 工作原则

- 修改代码前先检查现有实现、测试和打包脚本，优先沿用当前 Rust/Tauri/React 结构。
- 涉及远控兼容中转模式时，必须保护官方 ChatGPT 登录态，不要把中转 Key 写入 `auth.json` 的 `OPENAI_API_KEY`。
- 涉及配置样例、测试 Key、日志和发布产物时，默认做脱敏检查，避免提交真实 token、账号、Base URL 或本地隐私路径。
- 涉及开源信息时，保持 `LICENSE`、`NOTICE`、`UPSTREAM.md`、`README.md` 和 `README_EN.md` 一致。

## 高权限动作与审批失败

- 当使用中转 API/API 驱动时，如果联网下载、写 `/Applications`、打开 App、挂载 DMG、创建或修改 GitHub Release、修改仓库可见性等动作触发自动审查失败，并出现 `codex-auto-review`、`403 Forbidden`、模型权限不足、model-permission errors、automatic review rejection 等错误，不要继续把问题归因于 App、DMG、GitHub 包或目标文件损坏。
- 遇到上述失败时，应明确提示用户改用手动审批/手动授权后再重试，并说明具体需要用户批准的动作。
- 如果同一类高权限动作连续失败，停止重复重试，给出手动操作路径或等待用户重新授权。

## 常用验证

- 前端检查：`cd apps/codex-plus-manager && npm run check`
- 前端构建：`cd apps/codex-plus-manager && npm run vite:build`
- Rust 格式：`cargo fmt --check`
- Rust 测试：`cargo test`
- macOS 打包：`BINARY_DIR="$PWD/target/release" bash scripts/installer/macos/package-dmg.sh <version> arm64`
- 公开前检查：`git diff --check`，并扫描 `sk-`、`OPENAI_API_KEY`、`access_token`、`refresh_token`、`Bearer` 等敏感字段。
