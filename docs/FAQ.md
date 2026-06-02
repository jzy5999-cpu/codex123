# codex123 FAQ

## codex123 是什么？

codex123 是一个非官方 Codex App 外部增强工具，基于 CodexPlusPlus 改造。它使用外部 launcher 启动 Codex，并通过 Chromium DevTools Protocol 注入增强脚本，不直接修改 Codex 原始安装文件或 `app.asar`。

## 它解决什么问题？

主要解决三个问题：

- 使用中转 API 时，尽量保留官方 ChatGPT 登录态，降低手机 ChatGPT 远程控制 Codex 被本地配置破坏的概率。
- 通过本地协议代理兼容 DeepSeek / Chat Completions 上游。
- 提供 Codex 页面增强，例如会话删除、Markdown 导出、项目移动、Timeline、用户脚本等。

## 远控兼容中转模式能保证手机 100% 可远控吗？

不能。

codex123 能保证的是：本地配置尽量不破坏官方 ChatGPT 登录态，不把中转 Key 写入 `auth.json` 的 `OPENAI_API_KEY`，并检查 provider 是否满足远控兼容前提。

手机 ChatGPT 是否显示 Codex 入口、账号是否具备远控资格、远程会话是否可建立，仍由 OpenAI 官方控制。

## 远控兼容中转模式会如何写配置？

目标形态是：

```toml
model_provider = "codex123"

[model_providers.codex123]
name = "codex123"
base_url = "https://your-relay.example.com/v1"
wire_api = "responses"
experimental_bearer_token = "redacted"
requires_openai_auth = true
```

同时 `auth.json` 应保持：

```json
{
  "auth_mode": "chatgpt",
  "OPENAI_API_KEY": null
}
```

## 为什么不能把中转 Key 写进 `auth.json`？

`auth.json` 中的 `OPENAI_API_KEY` 会把 Codex 推向纯 API Key 模式。远控兼容中转的目标是保留官方 ChatGPT 登录态，所以中转 Key 应放在 provider 的 `experimental_bearer_token` 中。

## DeepSeek 可以直接用吗？

如果上游或中转站兼容 OpenAI Chat Completions，并且支持流式输出和工具调用，通常可以通过 codex123 的本地协议代理使用。推荐在供应商里选择 `Chat Completions` 上游协议。

如果中转站本身兼容 `/v1/responses`，也可以选择 Responses API。

## 为什么目前主要支持 macOS？

项目开发者使用 MacBook，当前主要目标是 macOS Apple Silicon 的本机可安装 App。Windows 构建链路保留为开发构建和 CI 产物，但没有在真实 Windows 设备上验证启动、注入、卸载和远控兼容行为。

## 会修改 Codex 原始安装文件吗？

不会。codex123 使用外部 launcher + CDP 注入增强脚本，不直接修改 Codex App 安装目录，不改 `app.asar`。

## 如何反馈中转站兼容性？

请优先使用 GitHub Issue 的“供应商 / 配置问题”模板，并脱敏：

- Base URL 可以只保留域名或服务名。
- 不要提交 API Key、access token、refresh token、账号邮箱等敏感信息。
- 附上管理工具诊断报告中的脱敏内容。

