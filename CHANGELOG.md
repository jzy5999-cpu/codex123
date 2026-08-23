# 更新日志

## Unreleased

## 0.2.29 - 2026-08-23

- 选择性审计 CodexPlusPlus `v1.2.48`-`v1.2.52`，只合入与 codex123 现有 macOS、协议代理、Provider Sync 和外部 CDP 注入架构直接相关的稳定性修复。
- Chat Completions 转 Responses 的所有完成事件都会补齐 `output_tokens_details.reasoning_tokens`；上游未返回 reasoning usage 时写入 `0`，避免 Codex 把完整响应误判为完成事件解析失败或中途断流。
- 插件市场过滤补丁先执行原始 `Array.prototype.filter`，仅在确实隐藏官方插件时检查已缓存的回调源码并回退完整列表，减少 renderer 全局过滤开销并兼容新版压缩函数签名。
- Provider Sync 会从 rollout `session_meta`、`thread_spawn_edges`、`agent_job_items` 和线程 source 字段识别 subagent、guardian/memory consolidation 等非用户线程，不再改写这些内部线程的 provider、cwd 或 user-event 状态。
- 删除 Codex 会话时同步原子清理 `session_index.jsonl`，并把被移除的索引行纳入原有 undo 备份；撤销删除时去重恢复，避免侧栏残留无法打开的幽灵会话。
- 本轮不合入微信连接、会话分享站、Stepwise、Dream Skin、VLM、Windows 发布、上游赞助/交流群或品牌内容；不修改 Codex App、`app.asar`、官方登录态或 RemoteRelay Key 边界。

## 0.2.28 - 2026-08-12

- 选择性合入 CodexPlusPlus `v1.2.47` 中对 codex123 有用的当前 Codex 兼容性修复，继续只更新 macOS Apple Silicon 版。
- Fast/服务层级补丁兼容当前 `app-initial-*` 与 `vscode-api-*` 资源模块，并覆盖新版 `vscode://codex/*` fetch 消息封装；继承模式优先读取 Codex App 设置，再回退到 `config.toml`。
- 模型目录接口返回当前 `service_tier`，让前端能准确展示 `config.toml` 的有效服务层级。
- CDP 与用户脚本注入只允许官方 Codex/ChatGPT 桌面主页面，排除应用内嵌浏览器、Avatar Overlay 和 Quick Chat 辅助页面。
- 顶部菜单兼容当前 `ApplicationMenuTopBar`，分支菜单监听器只响应相关新增节点并清理旧 observer，减少全页面变更带来的重复扫描。
- HTTPS 客户端启用系统原生证书根，提升企业代理和自签发受信证书环境下的兼容性。
- 当前 Codex 仅导出函数式 App Server 请求桥时，不再把不可安全替换的旧客户端兼容层误报为失败；该场景会明确标记为跳过，并继续使用 Statsig、JSON 响应和 MCP 模型白名单补丁。
- 本轮不合入 Windows、Dream Skin、Stepwise、VLM、按模型路由、上游赞助/交流群或品牌内容；不改变外部 launcher + CDP 架构，也不修改官方 App 或 `app.asar`。

## 0.2.27 - 2026-08-09

- 选择性合入 CodexPlusPlus `v1.2.45` 中对 codex123 有用的兼容性修复，继续只更新 macOS Apple Silicon 版。
- 中转协议代理会忽略没有 `content` 的 Responses 元数据项，避免 DeepSeek 等 Chat Completions 上游收到空消息。
- Codex 新建会话和模型请求补丁提升版本并支持重新注入，继续使用新版 Codex 原生会话流程，不恢复旧 projectless 强制改写。
- 应用内更新增加连接/下载超时、分阶段诊断日志和更准确的长耗时提示。
- 脚本市场新增名称、作者、描述和标签搜索，并支持板块/列表两种展示方式。
- 新建纯 API 供应商不再写入 `requires_openai_auth`；已有纯 API 配置保持不变，远控兼容中转仍强制 `requires_openai_auth = true` 并保护官方 ChatGPT 登录态。
- 本轮不合入 Windows、Dream Skin、Stepwise、VLM、上游赞助/交流群或品牌内容。

## 0.2.26 - 2026-08-02

- 选择性合入 CodexPlusPlus `v1.2.43`-`v1.2.44` 中对 codex123 有用的稳定性修复，继续只更新 macOS 版。
- 重注入 watchdog 现在复用 launcher 创建的完整 BridgeContext，避免页面刷新或浏览器标识变化后 data bridge 退回 core fallback，影响会话移动、导出和脚本热重载等能力。
- Codex 26.707+ 缺少旧 `app-server-manager-signals-*` asset 时，注入脚本会从当前页面资源中发现候选 app-server request client，提升模型白名单和插件市场补丁兼容性。
- 官方混合 API / 远控相关模式会保持本地 protocol proxy 启动，并写入受管理的 `openai_base_url`；清除配置时只移除 codex123 管理的本地代理地址，不覆盖用户自定义地址。
- 插件市场在 API Key 模式遇到远程 catalog 认证错误时回退本地结果，并隔离 remote-only 查询，避免把本地 fallback 混入远程专用搜索。
- Provider Sync 默认路径现在尊重 `CODEX_HOME`，与 codex123 其他 Codex home 解析保持一致。
- CDP 注入目标排除 Quick Chat 辅助 renderer；模型白名单注入收窄到明确的模型响应和指定 Statsig 配置，降低误伤页面状态的风险。
- 本轮未合入 Windows launcher port 修复、Dream Skin companion、Sub2API 倍率显示、上游赞助/QQ群/微信群文案或上游品牌内容。

## 0.2.25 - 2026-07-25

- 选择性合入 CodexPlusPlus `v1.2.42` 中对 codex123 有用的稳定性修复：`CODEX_SQLITE_HOME` 场景下会话数据库、thread reference 数据库和日志数据库路径解析保持一致。
- 会话删除改为扫描候选本地数据库；同一会话存在于多个数据库时会全部删除，并生成 grouped undo token，撤销时按原数据库恢复。
- grouped undo 增加全量预检和允许路径校验，避免只恢复部分数据库，拒绝恢复到非候选本地数据库。
- 注入脚本中的删除确认弹窗支持长内容滚动，按钮固定在底部，避免标题过长时无法点击取消/删除。
- 本轮继续只更新 macOS 版；未合入 Dream Skin companion、Windows watcher、上游赞助/交流群入口或上游品牌内容。

## 0.2.24 - 2026-07-20

- 选择性合入 CodexPlusPlus `v1.2.39`-`v1.2.41` 中对 codex123 有用的稳定性修复：管理工具诊断日志改为只读取尾部内容，并支持一键清理，避免大日志拖慢界面。
- 启动路径识别更严格：拒绝把 `codex123.app`、`codex123 管理工具.app` 或 Codex++ 管理目录误判为官方 Codex/ChatGPT App，降低重启和启动时找错应用的风险。
- 供应商配置写入增加失败回滚：切换远控兼容中转或纯 API 时，如果写入/诊断失败，会尽量恢复原 `config.toml` 和 `auth.json`。
- 本轮继续只更新 macOS 版；未合入 Windows 皮肤编辑器、Windows 包识别、上游赞助/交流群入口或上游品牌内容。

## 0.2.23 - 2026-07-14

- 修复脚本市场偶发加载失败：清单和脚本下载改用 codex123 的系统代理客户端，增加请求超时与有限重试。
- 脚本市场清单联网刷新成功后保存本地缓存；GitHub Raw 暂时不可访问时回退最近一次有效缓存，并在界面中明确标注缓存状态。
- 安装或重新启用脚本后，管理工具会尝试通过当前 CDP 会话立即加载启用脚本；脚本市场新增“重新加载脚本”按钮。
- 禁用或删除脚本后明确提示：脚本已产生的界面效果需要重启 codex123 才能完全移除。
- 本次仍只构建和发布 macOS Apple Silicon 版，不更新 Windows 安装包或发布资产。

## 0.2.22 - 2026-07-11

- 选择性合入 CodexPlusPlus `v1.2.34` 的 macOS 启动兼容：找不到独立 `Codex.app` 时可回退识别 `ChatGPT.app`，并从 `Info.plist` 读取真实可执行文件名；仍优先使用 Codex App，保持外部 launcher + CDP 注入方案。
- 模型目录在当前中转 profile 没有模型时继续读取 `config.toml`；注入桥接返回 `not_configured` 时也会从管理工具中转配置回退读取模型列表。
- 修复带版本路径的供应商模型接口，例如 `/api/coding/v3` 现在请求 `/api/coding/v3/models`，不再错误追加 `/v1/models`。
- 新版 Codex 缺少旧 app-server 模块时，模型注入诊断在有限重试后安静降级，避免约每 120ms 重复记录失败；其他模型白名单注入层继续工作。
- 本轮继续沿用 codex123 已验证的官方远端插件缓存配置保护；未引入上游 `role-specific-plugins`、赞助、广告、交流群、微信桥接、许可证变更或 Windows 发布内容。

## 0.2.21 - 2026-07-06

- 依据 CodexPlusPlus `v1.2.32` 复核官方远端插件缓存：确认内置 `openai-curated-remote` 快照已与上游一致，并新增 Product Design 远端安装元数据回归测试，避免后续打包遗漏 `.codex-remote-plugin-install.json`。
- 新增供应商配置写入回归测试：带 UTF-8 BOM 的 `config.toml` 写回时会移除 BOM，并继续保留已缓存的 `[marketplaces.openai-curated-remote]` 注册，减少切换供应商后官方远端插件状态丢失。
- 本轮不合入上游赞助、广告、Windows/RDP、命令包装器移除或未在 codex123 中启用的图片覆盖层/Stepwise 独立注入改动；仍只更新并验证 macOS Apple Silicon 版。

## 0.2.20 - 2026-07-04

- 选择性合入 CodexPlusPlus `v1.2.29` - `v1.2.31` 的官方远端插件缓存思路：新增内置 `openai-curated-remote` 插件市场快照，可在管理工具“增强功能”页释放并注册，用于补齐 Product Design 等远端插件的本地市场状态。
- 供应商配置写入 `config.toml` 时会保留已注册的 `openai-curated-remote` marketplace，避免切换供应商后 Product Design 等插件重新显示为未安装。
- 管理工具新增“官方远端插件缓存”状态展示和“释放并注册内置缓存”按钮，显示缓存路径、插件数量、技能数量和注册状态。
- 插件列表自动展开改为仅在完整增强 `patch` 模式运行；兼容增强 / 远控中转模式下不会自动点击插件页按钮。
- 本次仍只更新并验证 macOS Apple Silicon 版；不合入上游赞助、交流群、二维码、Windows/RDP 或 mobile relay 相关改动。

## 0.2.19 - 2026-07-01

- 选择性合入 CodexPlusPlus `v1.2.23` - `v1.2.27` 中对 codex123 有价值的 macOS 相关修复：新版 Codex SQLite 多数据库场景下优先选择包含 `threads` 表的会话库，避免错误落到 inbox/automation 数据库。
- Provider Sync 历史会话修复会识别 `projectless-thread-ids`，不再把无项目会话的 cwd 重新写回 SQLite；同时保留多 `session_meta` 同步和目标 provider 选择能力。
- 会话项目移动兼容新版 Codex session 存储迁移：移动时会按候选数据库查找真实 thread，并在结果中记录命中的 `db_path`。
- 删除/撤销恢复增加备份文件路径白名单校验，只允许恢复备份中对应 thread 的 rollout 文件，降低异常备份内容造成误写的风险。
- 中转配置写入兼容带 UTF-8 BOM 的 `config.toml` 内容，写回时会移除 BOM。
- 管理工具和 Codex 内增强菜单新增“强制中文界面”和“插件列表自动展开”开关；强制中文默认关闭，开启或关闭后建议重启 codex123。
- 本次仍只更新并验证 macOS Apple Silicon 版；不合入上游赞助、交流群、二维码、Windows/RDP 或 installation-free 相关改动。

## 0.2.18 - 2026-06-28

- 选择性合入 CodexPlusPlus 上游 per-model context windows 思路：供应商模型列表支持为每个模型单独填写上下文窗口，并在应用配置时生成 `model_catalog_json`。
- 管理工具的模型列表从单个文本框改为“模型名 / 上下文窗口”两列；从上游获取模型列表时会清空旧窗口值，避免误套用到新模型。
- 保留旧 `model[1M]` 写法的迁移兼容，但保存后会拆成干净模型名和独立窗口映射，避免后缀泄漏到 Codex `model` 字段。
- 本次仍只验证并发布 macOS Apple Silicon 包；Windows 版本和发布资产不随本次更新变更。

## 0.2.17 - 2026-06-28

- 选择性合入 CodexPlusPlus 上游 macOS Computer Use 子进程清理思路：codex123 helper 运行期间会定期清理可能残留的 `SkyComputerUseClient` 进程，降低长时间使用 Computer Use 后的内存压力。
- 复核启动链路，继续保持 codex123 启动 Codex 时不自动应用当前 relay profile，避免无意重写 `config.toml` / `auth.json`。
- 本次仍只更新 macOS 相关功能路径；不合入 Windows installation-free watcher、上游赞助、交流群或二维码内容。

## 0.2.16 - 2026-06-24

- 选择性合入 CodexPlusPlus 上游粘贴修复：管理工具和 Codex 内增强菜单新增“粘贴修复”开关，开启后从 Word 等富文本来源粘贴到 Codex 输入框时只保留纯文本，减少误识别为图片或文件附件。
- 选择性合入 CodexPlusPlus 上游更新器架构选择修复：检查更新时优先选择当前 macOS 架构对应的 DMG，避免同一 Release 同时存在 x64 / arm64 资产时选错。
- 本次仍只更新 macOS 相关功能路径；不合入上游赞助、交流群、二维码等 Codex++ 项目展示内容。

## 0.2.15 - 2026-06-21

- 选择性合入 CodexPlusPlus `v1.2.18` 的 Responses 到 Chat Completions 协议修复：历史 tool call 的 `arguments` 为空、非 JSON、数组或纯文本时会规范化为合法 JSON，降低中转 API 报错概率。
- 选择性合入 CodexPlusPlus 上游 macOS companion binary 定位修复：`codex123.app` 与 `codex123 管理工具.app` 互相查找启动二进制时更稳定，并避免非 Applications 临时 bundle 影响默认安装目录。
- 本次仍只更新 macOS 本机版本；Windows 版本和发布资产不随本次更新变更。

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
