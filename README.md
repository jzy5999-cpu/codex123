# codex123

<p align="center">
  中文 | <a href="README_EN.md">English</a>
</p>

<p align="center">
  <img alt="Release" src="https://img.shields.io/github/v/release/jzy5999-cpu/codex123">
  <img alt="Stars" src="https://img.shields.io/github/stars/jzy5999-cpu/codex123">
  <img alt="License" src="https://img.shields.io/github/license/jzy5999-cpu/codex123">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Tauri" src="https://img.shields.io/badge/tauri-2.x-24C8DB">
</p>

codex123 是一个非官方、开发体验优先的 Codex App 外部增强启动器和管理工具。项目目标是更好地使用 Codex：保留官方 ChatGPT 登录态，尽量降低中转配置对远程控制能力的破坏，同时继续使用外部 launcher 和 Chromium DevTools Protocol 注入增强脚本，不直接修改 Codex App 原始安装文件。

本项目致敬并基于 [CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus)。CodexPlusPlus 提供了外部 launcher、管理工具、CDP 注入、安装包结构等核心基础；`codex123` 在此基础上加入远控兼容中转模式，并把第一版交付重点放在 macOS Apple Silicon。项目也参考了其他开源工具的实践，尤其借鉴了 [ccswitch](https://github.com/farion1231/cc-switch) 在 Codex 接入 DeepSeek / Chat Completions 上游时的本地路由与协议转换思路，优先服务个人开发工作流。

`codex123` 不是 OpenAI 官方项目，也不与 OpenAI 存在隶属、赞助或背书关系。Codex、ChatGPT 及相关名称归其各自权利方所有。

## 当前边界

- 目前主要支持 macOS Apple Silicon；Windows x64 版仅作为开发构建和 CI 产物提供。
- 开发者使用 Mac 电脑，Windows 版尚未在真实 Windows 设备上验证，不承诺实际可用性。
- macOS 第一版使用本地构建和 ad-hoc 签名，不做 Apple Developer ID 公证；Windows 第一版使用 NSIS 安装包，暂不做代码签名。
- 手机 ChatGPT 远程控制入口、账号资格和远程会话能力由 OpenAI 官方控制，`codex123` 不能承诺 100% 可用。
- 本项目只保证本地配置尽量不破坏官方 ChatGPT 登录态，并保持中转请求与 CDP 注入增强同时可用。

## codex123 远控兼容中转

`codex123` 基于 CodexPlusPlus，新增明确的“远控兼容中转模式”。这个模式用于同时满足三件事：

- 保留官方 ChatGPT 登录态，避免把 Codex 切成纯 API Key 模式。
- 让 Codex 模型请求走自定义中转 Base URL。
- 继续通过外部 launcher + Chromium DevTools Protocol 注入增强脚本，不修改 Codex App 原始安装文件。

推荐使用方式：

1. 先用原版 Codex 登录官方 ChatGPT 账号。
2. 手机 ChatGPT 登录同一个账号。
3. 打开 codex123 管理工具。
4. 在供应商配置里选择“远控兼容中转”。
5. 填写中转 `Base URL` 和 `Key`，上游协议选择 `Responses API`。
6. 从 `codex123` 入口启动，不要从原版 Codex 图标启动。

该模式会生成类似配置：

```toml
model_provider = "codex123"

[model_providers.codex123]
name = "codex123"
base_url = "https://your-relay.example.com/v1"
wire_api = "responses"
experimental_bearer_token = "sk-..."
requires_openai_auth = true
```

同时会保留并修正 `~/.codex/auth.json` 的官方登录态，不会把中转 Key 写入 `OPENAI_API_KEY`：

```json
{
  "auth_mode": "chatgpt",
  "OPENAI_API_KEY": null
}
```

注意：`codex123` 能保证本地配置不破坏官方远程控制所需的 ChatGPT 登录态，但手机端 Codex 入口是否可见、Free 账号是否可用、远程会话是否能建立，仍取决于 OpenAI 官方账号权限和功能状态。

## DeepSeek / Chat Completions 兼容优化

Codex 当前主要按 OpenAI Responses API 形态请求模型，而 DeepSeek 官方 API 和很多中转站实际提供的是 OpenAI Chat Completions 兼容接口。`codex123` 借鉴 ccswitch 的 Codex DeepSeek 路由设计，在本地协议代理中把 Codex 的 Responses 请求转换为 Chat Completions 请求，再把上游响应转换回 Responses 形态。

推荐 DeepSeek 使用方式：

1. 在管理工具中新建供应商。
2. `Base URL` 填 DeepSeek 或中转站地址，例如 `https://api.deepseek.com` 或兼容服务的 `/v1` 根地址。
3. 上游协议选择 `Chat Completions`。
4. 选择“远控兼容中转”时仍保持 `wire_api = "responses"`，并从 `codex123` 入口启动 Codex。

当前优化包括 DeepSeek reasoning effort 映射、`reasoning_content` 流式返回转换、assistant tool-call 历史的 `reasoning_content` 兜底，以及基础工具调用历史转换。该能力提高 DeepSeek 长会话和工具调用稳定性，但不保证所有 DeepSeek 中转站 100% 可用；中转站仍需要兼容 OpenAI Chat Completions、流式输出和工具调用。

## 快速使用

Release 产物为：

- macOS Apple Silicon：`codex123-*-macos-arm64.dmg`
- Windows x64：`codex123-*-windows-x64-setup.exe`（开发构建，未实机验证）

安装后会有两个入口：

- `codex123`：静默启动入口，不显示管理界面，只负责启动 Codex 并注入增强功能。
- `codex123 管理工具`：Tauri 控制面板，用于启动、检查、修复、更新、配置中转注入、管理增强功能和用户脚本。

macOS DMG 会安装 `/Applications/codex123.app` 和 `/Applications/codex123 管理工具.app`。Windows 安装包会安装 `codex123.exe` 和 `codex123-manager.exe`，并创建 `codex123` 与 `codex123 管理工具` 快捷方式。

本机从源码构建 macOS 安装包：

```bash
cd /Users/jiangzengyan/Downloads/codex/codex123
cd apps/codex-plus-manager
npm ci
npm run vite:build
cd ../..
cargo build --release
BINARY_DIR="$PWD/target/release" bash scripts/installer/macos/package-dmg.sh 0.2.2 arm64
```

生成文件位于 `dist/macos/codex123-0.2.2-macos-arm64.dmg`。第一版使用 ad-hoc 签名，不做 Apple Developer ID 公证；如果 macOS 首次打开提示无法验证开发者，请右键打开，或在“系统设置 -> 隐私与安全性”中允许打开。

Windows 安装包由 GitHub Actions 在 Windows runner 上构建。由于开发者使用 Mac 电脑，Windows 版目前只完成代码、打包脚本和 CI 构建链路，尚未确认在真实 Windows 环境中的启动、注入和远控兼容行为。本机 Windows 构建需要 Rust MSVC 工具链、Node.js 22 和 NSIS：

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

生成文件位于 `dist/windows/codex123-0.2.2-windows-x64-setup.exe`。

## 开源与致谢

`codex123` 使用 MIT License 开源。上游来源和本地改动记录见 [UPSTREAM.md](UPSTREAM.md)，项目归属和免责声明见 [NOTICE](NOTICE)。

特别感谢 [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus)。没有 CodexPlusPlus 对外部启动、管理工具、CDP 注入和安装体验的探索，`codex123` 不会这么快成形。

同时感谢 [ccswitch](https://github.com/farion1231/cc-switch) 对 Codex 接入 DeepSeek / Chat Completions 上游的本地路由和协议转换实践，`codex123` 的 DeepSeek 兼容优化参考了这部分设计。

## 主要功能

- Rust 后端和静默 launcher，启动时不依赖额外运行时。
- Tauri + React 管理工具，支持深色/浅色切换。
- 外部 CDP 注入，不改 `app.asar`，不向 Codex 安装目录写入 DLL。
- 中转注入模式：支持多个中转配置，写入 `codex123` provider，并可切回官方 ChatGPT 登录态。
- 传统增强模式：插件入口解锁、特殊插件强制安装、会话删除、Markdown 导出、项目移动、Timeline 等。
- 用户脚本独立管理，可在启动时注入自定义脚本。
- Provider 同步：启动前同步本地会话 metadata，切换供应商后旧会话仍可见。
- Zed 打开入口：识别远程 SSH 上下文后，可从 Codex 直接打开对应文件到 Zed Remote Development。
- Upstream worktree 创建：可从 `upstream/<base-branch>` 创建新 worktree，创建前自动 fetch 远端分支，降低从陈旧本地 HEAD 派生导致的冲突风险。
- GitHub Release 自动更新，管理工具和静默启动器都会检测可用更新。
- Windows 单实例、无黑框启动、管理员权限清单、NSIS 安装包和系统桌面路径识别。
- macOS arm64 DMG，静默入口隐藏 Dock 图标。

## 痛点与解决

API Key 登录模式下，Codex 原生插件入口会提示需要登录 ChatGPT，导致插件功能无法正常使用：

![API Key 模式下插件入口不可用](docs/images/pain-plugin-disabled.png)

Codex 原生会话列表只有归档入口，没有真正的删除按钮：

![原生会话列表缺少删除能力](docs/images/pain-no-delete-button.png)

codex123 启动后会解锁插件入口，并在会话列表悬停时显示删除按钮：

![codex123 解锁插件入口并添加删除按钮](docs/images/solution-plugin-and-delete.png)

顶部菜单栏会出现 `codex123`，可以查看后端状态并打开设置面板：

![codex123 后端状态指示灯](docs/images/backend-status-indicator.png)
![codex123 设置面板](docs/images/settings-panel.png)

## 中转注入

中转注入适合已经在 Codex/ChatGPT 中完成官方账号登录，同时希望把模型请求转到自定义兼容 API 的场景。

在管理工具的“中转注入”页面：

1. 确认已经检测到 ChatGPT 登录状态。
2. 添加一个或多个中转配置，填写 Base URL 和 Key。
3. 选择当前配置并应用中转注入。
4. 启动 `codex123`。

codex123 会在 `~/.codex/config.toml` 中写入类似配置：

```toml
model_provider = "codex123"

[model_providers.codex123]
name = "codex123"
wire_api = "responses"
requires_openai_auth = true
base_url = "https://example.com/v1"
experimental_bearer_token = "sk-..."
```

如果需要回到官方登录态，在“中转注入”页面点击清除 API 模式即可移除 `OPENAI_API_KEY` 相关配置并切回官方 ChatGPT 登录模式。

## 增强功能

增强功能在管理工具中统一开关。默认开启增强注入；关闭后不会注入 codex123 菜单和脚本。

如果启用中转注入模式，插件入口解锁和强制安装不再需要，界面会提示“中转注入模式下无需开启”。会话删除、导出、移动、Timeline 和用户脚本等增强仍可继续使用。

## 自动更新与安装包

codex123 通过 GitHub Release 发布安装包。目前发布 macOS Apple Silicon arm64 DMG 和 Windows x64 NSIS 安装包；其中 Windows 安装包仅作为开发构建提供，尚未实机验证。

管理工具的“关于”页可以检查并启动更新。静默启动器发现新版本时会拉起管理工具并进入更新提示。

## 数据位置

- Codex 配置：`~/.codex/config.toml`
- Codex 登录状态：`~/.codex/auth.json`
- Codex 本地数据库：`~/.codex/state_5.sqlite`
- codex123 状态与日志：`~/.codex123/`
- 旧版兼容读取：如果 `~/.codex123/settings.json` 不存在，管理工具会读取旧路径 `~/.codex-session-delete/settings.json`；保存后会写入新路径。
- Provider 同步备份：`~/.codex/backups_state/provider-sync`

## 常见问题

### codex123 菜单没出现

确认是从 `codex123` 入口启动，而不是原版 Codex。也可以打开管理工具的“诊断”和“日志”页面查看注入状态。

### 插件内显示后端连不上

先在浏览器或 PowerShell 里测试：

```powershell
Invoke-RestMethod -Method Post -Uri http://127.0.0.1:57321/backend/status -Body "{}" -ContentType "application/json"
```

如果接口正常，但插件仍显示超时，通常是 Codex 页面里的 CDP bridge 或脚本缓存问题。重启 codex123，或在管理工具里查看日志中的 `renderer.script_loaded`、`bridge.request`、`bridge.response`。

### Upstream worktree 和 Codex 原生创建有什么区别

codex123 的 Upstream worktree 功能等价于先更新远端分支，再执行：

```bash
git worktree add -b <new-branch> <worktree-path> upstream/<base-branch>
```

这样新 worktree 从最新的远端跟踪分支开始，而不是从当前会话所在的本地 HEAD 开始。如果 codex123 无法安全识别当前 Codex 版本的原生 worktree 创建表单，请从 codex123 菜单中手动填写仓库路径、分支名、worktree 路径、remote 和 base branch。

### macOS 提示无法打开或已损坏

当前安装包未签名/未公证时，macOS Gatekeeper 可能拦截。可以在“系统设置 - 隐私与安全性”中允许打开。正式分发建议配置 Apple Developer ID 签名和 notarization。

### Windows 能用吗

目前只能说“有 Windows x64 开发构建”，不能承诺实际可用。开发者使用 Mac 电脑，Windows 安装包由 GitHub Actions 生成，尚未在真实 Windows 设备上验证启动、CDP 注入、快捷方式、卸载和远控兼容中转行为。第一版 Windows 安装包暂不做代码签名，首次安装可能出现 SmartScreen 提示，需要手动确认来源后继续。

### macOS Intel 能用吗

当前发布链路只提供 `macos-arm64.dmg`。Intel Mac 暂不作为第一优先级发布目标。

## 开发

```bash
# 前端检查
cd apps/codex-plus-manager
npm install
npm run check
npm run vite:build

# Rust 检查
cd ../..
cargo fmt --check
cargo test
cargo build --release
```

主要结构：

```text
apps/
  codex-plus-launcher/          静默启动入口
  codex-plus-manager/           Tauri 管理工具
assets/inject/
  renderer-inject.js            注入到 Codex 渲染端的增强脚本
crates/
  codex-plus-core/              启动、注入、配置、更新、安装、桥接等核心逻辑
  codex-plus-data/              会话数据、导出、Provider 同步
scripts/installer/
  windows/codex123.nsi     Windows NSIS 安装包
  macos/package-dmg.sh          macOS DMG 打包
```

## 友情链接

- [LINUX DO](https://linux.do)

## 说明

codex123 是外部增强工具，不修改 Codex App 原始文件。Codex App 更新后，如果页面结构变化，可能需要更新注入脚本。
