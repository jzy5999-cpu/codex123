# 更新日志

## Unreleased

## 0.2.14 - 2026-06-18

- 选择性合入 CodexPlusPlus `v1.2.14` 的代理 User-Agent 透传修复：Chat Completions、Responses 转发和 Models 代理在未手动配置 User-Agent 时会保留 Codex 客户端原始 User-Agent。
- 供应商高级选项新增 `User-Agent` 字段；留空表示透传 Codex 原始 User-Agent，填写后则按供应商配置覆盖。
- 本次仍只更新 macOS 相关功能路径；Windows 版本不随本次更新变更。

## 0.2.13 - 2026-06-16

- 选择性合入 CodexPlusPlus 上游模型白名单注入修复：模型解锁扫描会跳过工作区侧边栏等 Codex 主界面区域，降低误伤 UI 的风险。
- 选择性合入启动配置保护修复：启动或重启 codex123 时不再自动应用当前中转 profile，避免无意重写 `config.toml` / `auth.json`。
- Provider Sync 历史会话修复兼容新版 Codex SQLite 目录：除旧版 `state_5.sqlite` 外，也会处理 `~/.codex/sqlite/codex-dev.db` 等会话数据库，并一并备份 sidecar 文件。
- 本次仍只更新 macOS 本机版本；Windows 版本不随本次更新变更。

## 0.2.12 - 2026-06-10

- 宠物导入默认来源从 Petdex 站点切换为 `codex123 Curated Pets`，避免 Petdex/Vercel 403 导致刷新不可用。
- 新增 10 个基于 Google Noto Emoji 开源图片资源生成的 Codex-compatible 宠物包，并在仓库中保留来源、许可证和生成说明。
- 管理工具宠物页文案改为通用“宠物源”，不再把 Petdex 作为默认市场来源。

## 0.2.11 - 2026-06-09

- 选择性合入 CodexPlusPlus `v1.2.4` 的 Provider Sync 多 `session_meta` 修复：同一个 rollout JSONL 中的所有 `session_meta` 现在都会同步到目标 provider，并在失败时回滚完整文件内容。
- 选择性合入 CodexPlusPlus `v1.2.4` 的 macOS DMG bundle 结构修复：补全 `PkgInfo`、关键 `Info.plist` 字段、两步 ad-hoc 签名和 bundle 校验，同时保留 `codex123` 品牌、arm64-only 打包和可执行文件重命名。

## 0.2.10 - 2026-06-09

- 选择性合入 CodexPlusPlus 上游插件解锁策略更新：桥接层会暴露当前启动的 Codex App 版本，注入脚本按版本自动选择旧版“插件入口解锁”或新版“插件市场解锁”路径。
- 管理工具和注入菜单拆分插件相关三项开关：插件市场解锁、强制解锁插件入口、特殊插件强制安装，便于单独关闭高风险或不适配的策略。
- 插件解锁诊断新增 `plugin_unlock_strategy_selected` 事件，记录 Codex App 版本、策略和开关状态，便于后续排查插件入口不可见或市场列表异常。
- Provider Sync 历史会话修复新增目标选择：默认继续使用当前 `config.toml` 的 `model_provider`，也可手动指定 `openai`、`codex123` 或其他 provider id，并在 UI 中显示本次修复目标。
- 本轮仍只更新 macOS 相关功能路径；Windows 发布流程和安装包不随本次功能更新变更。

## 0.2.9 - 2026-06-04

- Petdex 宠物市场新增“综合热度”排序：基于已安装、可更新、作者、描述、主页和标签完整度计算本地推荐顺序，并在卡片中显示热度标签与原因说明。
- 选择性合入 CodexPlusPlus 上游插件市场兼容改进：增强插件市场列表解锁、隐藏过滤绕过、插件安装请求修正和诊断日志。
- 插件市场增强保留 `codex123` 品牌和中性 OpenAI 插件文案，不合入 Codex++ 交流群、赞助、二维码或可见品牌信息。
- 管理工具保存设置时跳过空配置的官方登录 profile 存储归一化，减少默认中转 profile 的无效解析日志。
- CDP 注入目标选择会跳过 Codex 宠物 `avatar-overlay` 页面，避免宠物覆盖层抢走主界面增强脚本注入。
- 本次仍只更新 macOS 版；Windows 构建链路不随本次 macOS 修复同步更新。

## 0.2.8 - 2026-06-04

- 宠物导入页面新增 spritesheet 缩略图懒加载预览，并限制远程列表首屏最多渲染 80 个结果，避免一次性加载大量图片。
- 宠物安装时写入 `codex123-installed.json` 元数据，记录来源、URL 和安装时间。
- Petdex 刷新时根据安装元数据检测已安装宠物是否可更新。
- 本地已安装列表新增删除按钮，只允许删除 `~/.codex/pets/<slug>` 下合法 slug 目录。
- 宠物卡片增加“可更新”状态，安装按钮会切换为“更新”。

## 0.2.7 - 2026-06-03

- 管理工具新增“宠物导入”页面，第一版支持 Petdex manifest 刷新、搜索、安装和覆盖安装。
- 新增 Codex 宠物包安装器：下载 `pet.json` 与 `spritesheet` 后写入 `~/.codex/pets/<slug>/`，不修改 Codex App 原始安装目录。
- 安装前增加安全校验：HTTPS、资源域名允许列表、slug 校验、JSON 校验、PNG/WEBP 校验、大小限制、临时目录写入与原子安装。
- 支持读取本地已安装宠物，并可从管理工具打开 `~/.codex/pets` 目录。
- README / README_EN 增加 Petdex MVP 使用方式和边界说明。

## 0.2.6 - 2026-06-03

- 选择性合入 CodexPlusPlus 上游 launcher 稳定性改进：Codex 已启动但 CDP 注入暂未就绪时进入 `running_degraded`，不再立即关闭已启动的 Codex。
- 注入流程增加更长重试窗口，页面或 CDP 启动较慢时继续等待增强脚本就绪。
- 已有 Codex 实例路径也会启动 helper 并尝试注入，改善重复启动/重启时增强未恢复的问题。
- 保留 codex123 的代理环境注入、RemoteRelay 保护和品牌文案；不合入上游交流群图片、Codex++ 可见品牌或 Windows 发布改动。
- README / README_EN 增加个人使用、开源供同好借鉴和不用于攻击/绕过安全机制的项目边界说明。

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
