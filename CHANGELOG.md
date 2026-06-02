# 更新日志

## 0.2.5 - 2026-06-02

- 管理工具新增远控兼容中转诊断卡片，集中检查 ChatGPT 登录态、`auth.json` API Key 清理、provider、`base_url`、`wire_api = "responses"`、`requires_openai_auth = true` 和 `experimental_bearer_token`。
- 诊断报告新增结构化 `remoteRelay` 字段，并支持导出脱敏 JSON 文件，方便排查手机 ChatGPT 看不到 Codex 的配置前提问题。
- 关于页更新检查文案区分“本机运行版本”和“线上 Release 版本”，避免本机手动安装开发版时被线上版本状态误导。

## 0.2.4 - 2026-06-02

- 从 CodexPlusPlus 最新上游选择性合入管理工具文件选择器权限修复，避免 Tauri dialog 插件缺少 capability 时文件/目录选择器打不开。
- 选择性合入 launcher 旧实例恢复与已有 Codex 拉起逻辑，重复启动时会尝试拉起/激活 Codex，并记录诊断日志。
- 增强 Chat Completions 本地协议代理：支持 `/v1/v1` 与 `/codex/v1` 代理路径，支持 Chat Completions 透传入口，并把上游非 2xx 响应归一化为 Responses API 错误结构。
- 会话行的导出与移动操作改为“更多操作”菜单，减少按钮拥挤并修复 hover 菜单交互。
- 明确后续默认只更新 macOS 版本；没有明确要求时不更新 Windows 安装包或 Windows Release 资产。

## 0.2.3 - 2026-06-02

- 修复远控兼容中转切换时可能使用供应商中保存的旧 `auth.json`，导致重新登录后的官方 ChatGPT 账号被旧账号覆盖、手机端看不到 Codex 的问题。
- RemoteRelay 切换现在始终以当前 `~/.codex/auth.json` 的实时官方登录态为准，并只把 `OPENAI_API_KEY` 归一化为 `null`。

## 0.2.2 - 2026-06-01

- 修复 GitHub Actions Windows Release job 中 NSIS `makensis` 未进入 PATH 导致发布失败的问题。
- 重新发布安装包链路，用于生成可供“检查更新”读取的 `latest.json`。

## 0.2.1 - 2026-06-01

- 修复远控兼容中转供应商切换后 `experimental_bearer_token` 被迁移走，导致回切时提示未填写 API Key 的问题。
- 更新检查在未发布可用 GitHub Release 安装包时，明确提示“尚未发布可用安装包”，避免误导为已经安装最新源码版本。
- 继续包含 DeepSeek / Chat Completions 兼容优化：DeepSeek reasoning effort 映射、tool-call 历史 `reasoning_content` 兜底，以及 README 中对 ccswitch 的致谢说明。

## 0.2.0 - 2026-06-01

- 新增 Windows x64 NSIS 安装包发布链路，产物命名为 `codex123-<version>-windows-x64-setup.exe`。
- Windows 安装入口、快捷方式、安装目录、卸载注册表项和发布包命名统一改为 `codex123`。
- GitHub Release workflow 同时构建 macOS arm64 DMG 和 Windows x64 安装包，并把二者写入 `latest.json`。
- README / README_EN 更新为 macOS + Windows 双平台说明。
- 明确 Windows 版目前仅作为开发构建和 CI 产物提供，开发者使用 Mac，尚未在真实 Windows 设备上验证可用性。

## 0.1.3 - 2026-06-01

- 移除 README 顶部旧 Codex++ 图标。
- 移除 README 和管理工具/注入菜单中的赞助商展示，只保留普通推荐内容。

## 0.1.2 - 2026-06-01

- 移除 README 和注入菜单中的原 Codex++ 交流、Discord 与赞赏入口。
- 管理工具中的 `auth.json` 默认隐藏 token 和 API Key，需要手动显示后才能编辑。
- 用户脚本配置目录迁移为 `codex123`，并兼容迁移旧 `Codex++` 目录。
- macOS DMG 打包完成后自动清理临时 stage 目录，避免本地残留重复 bundle id。

## 0.1.1 - 2026-06-01

- 将 Codex 页面右上角注入菜单和弹窗中的用户可见品牌从 `Codex++` 改为 `codex123`。
- 将注入菜单的问题反馈链接改为 `jzy5999-cpu/codex123`。

## 0.1.0 - 2026-06-01

- codex123 初始版本，基于 CodexPlusPlus，面向 macOS Apple Silicon 本机安装使用。
- 新增远控兼容中转模式，保持 ChatGPT 官方登录态，同时把中转 Key 写入 provider 配置。
- 修复 macOS CDP 连接稳定性，启动时指定 `127.0.0.1` 调试地址，并在查询目标时兼容 IPv4 / IPv6 loopback。
- 将 codex123 状态目录迁移到 `~/.codex123/`，并兼容读取旧 `~/.codex-session-delete/settings.json`。
- 更新检查在尚未发布 GitHub Release 时显示无可用更新，而不是直接报错。

## 1.1.8 - 2026-05-26

- 新增上游分支 worktree 支持，可从上游仓库/分支创建和选择独立工作区。
- 新增上游分支列表获取、默认值处理、远端解析和 worktree 创建相关接口与测试。
- 优化供应商同步逻辑，保留 rollout 文件 mtime，减少同步后不必要的会话状态变化。
- 新增独立的「工具与插件」页面，用于统一管理 Codex++ / Codex 的 MCP、skills、plugins，不再绑定到单个供应商。
- 切换供应商时会合并当前启用的工具与插件配置，同时避免把供应商专属配置误写入通用配置。
- 工具与插件列表改为从当前 Codex 配置实时读取启用状态，支持直接开关和删除条目。
- 调整通用配置提取逻辑，改为手动提取，减少自动覆盖和配置污染。
- 修复供应商切换隔离问题，避免 `model_catalog_json`、旧 `model_provider`、历史 provider 表和旧 `auth.json` 被带到新供应商。
- 修复纯 API 模式下 `auth.json` 没有写入 API Key 的问题，并固定供应商 provider 名称为 `CodexPlusPlus`。
- 优化模型目录写入方式，支持与原始模型目录合并，并在预览中显示真实路径。
- 供应商配置页新增模型插入方式、模型列表、上下文大小、压缩上下文大小、目标功能等配置项。
- 官方模式下隐藏仅混入 API Key 场景使用的模型列表和模型插入方式。
- 将 Base URL、API Key、上游协议移动到模型列表之前，测试模型和上下文选项收进「更多选项」。
- 修复 `model_reasoning_effort`、`plan_mode_reasoning_effort` 重复写入导致 TOML 解析失败的问题。
- 修复重复插件表、空配置体、布尔值解析等导致配置文件解析失败的问题。
- 优化供应商详情页布局，保持顶部返回和提示区域固定，增大默认窗口尺寸并减少顶部缝隙。
- 移除脚本安装时的 checksum 阻断，避免市场脚本校验不一致导致安装失败。
- 清理关于页和状态页中不需要展示的登录、当前供应商、配置文件路径等信息。
- 调整提示信息居中显示，避免遮挡重启按钮。
- 更新讨论群二维码、README 说明和 macOS DMG 打包脚本。
