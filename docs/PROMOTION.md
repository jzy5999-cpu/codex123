# codex123 推广文案

## 简短版

我做了一个 Codex 增强工具：codex123。

它基于 CodexPlusPlus 改造，主要面向 macOS。核心目标是让 Codex 走中转 API 时，尽量保留官方 ChatGPT 登录态和手机远程控制 Codex 的前提，同时继续通过外部 launcher + Chromium DevTools Protocol 注入增强脚本，不修改 Codex 原始安装文件。

新增能力包括远控兼容中转模式、远控前提诊断、DeepSeek / Chat Completions 本地协议转换、会话删除、Markdown 导出、项目移动、Timeline、用户脚本等。

GitHub： https://github.com/jzy5999-cpu/codex123

## V2EX / 社区版

标题建议：

```text
做了一个 Codex 增强工具：中转 API 下尽量保留手机 ChatGPT 远控能力
```

正文：

```text
最近把自己使用 Codex 的一些需求整理成了一个开源项目：codex123。

项目基于 CodexPlusPlus 改造，首先感谢原项目对外部 launcher、管理工具、CDP 注入和安装包结构的探索。codex123 的目标不是替代 Codex，而是让 Codex 更适合个人开发工作流。

我遇到的核心问题是：使用中转 API / 第三方兼容 API 时，很容易把 Codex 切到纯 API Key 模式，进而破坏官方 ChatGPT 登录态。这样一来，手机 ChatGPT 里是否还能看到并远程控制 Codex 就更难排查。

codex123 做了一个“远控兼容中转模式”：

- 保留 auth.json 里的官方 ChatGPT 登录态
- 不把中转 Key 写入 OPENAI_API_KEY
- 中转 Key 写入 config.toml 的 provider 配置
- provider 保持 wire_api = "responses"
- 支持 requires_openai_auth = true
- 管理工具提供远控前提诊断

另外还加入了 DeepSeek / Chat Completions 兼容优化：Codex 发 Responses 请求，本地代理转换成 Chat Completions，再把上游响应转回 Responses 形态。

当前主要支持 macOS Apple Silicon。Windows 只保留开发构建链路，还没有真实机器验证。

需要说明的是，手机 ChatGPT 是否能显示 Codex 入口、账号是否有权限、远程会话是否可用，仍然由 OpenAI 官方控制。codex123 能做的是保证本地配置尽量不破坏这个前提。

GitHub：
https://github.com/jzy5999-cpu/codex123
```

## 即刻 / X 短帖

```text
做了一个 Codex 增强工具 codex123，基于 CodexPlusPlus 改造。

主要解决：Codex 走中转 API 时，尽量保留官方 ChatGPT 登录态和手机远控前提。

功能：
- 远控兼容中转模式
- 远控前提诊断
- DeepSeek / Chat Completions 兼容代理
- 外部 launcher + CDP 注入，不改 app.asar
- macOS Apple Silicon DMG

GitHub：
https://github.com/jzy5999-cpu/codex123
```

## README 首屏卖点

```text
codex123：基于 CodexPlusPlus 改造的 macOS Codex 增强工具。支持远控兼容中转、远控前提诊断、DeepSeek / Chat Completions 兼容、本地 CDP 注入增强。目标是在使用中转 API 时尽量保留 ChatGPT 官方登录态和手机远控 Codex 的前提。
```

## 适合配图 / GIF 的演示流程

1. 打开 `codex123 管理工具`。
2. 进入“供应商配置”。
3. 展示“远控兼容中转诊断”全部通过。
4. 打开“春风远控中转”或其他远控中转供应商。
5. 启动 `codex123`。
6. 展示 Codex 页面右上角出现 codex123 菜单。
7. 补一句说明：手机端入口仍取决于 OpenAI 官方账号权限。

