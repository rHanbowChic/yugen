# Yugen

以 Minecraft 背景音乐体验为核心的桌面氛围播放器。

Yugen 不会持续不断地播歌，而是模拟游戏中的节奏：随机抽取曲目，休息一段时间后再播放下一首，让音乐在“不经意间”出现。

一个适合长期挂在托盘、低打扰、偶尔给人惊喜的氛围工具。

## 适用场景

- 日常办公、学习、写作时的环境氛围
- 喜欢 Minecraft 原生音乐节奏，而非连续播放
- 想要一个常驻后台、偶尔响起一首的“陪伴型播放器”

## 核心特性

- **贴近游戏节奏的播放逻辑**
  - 每次从当前 Collection 中随机选曲
  - 一首播放结束后，按 `minWait ~ maxWait` 随机等待下一首
  - Overworld 默认即为长间隔（常见 10~20 分钟）
- **Collection 选择**
  - 提供 Overworld / Nether / End / Menu 等多种集合
- **按需缓存 + 批量缓存**
  - 首次播放某曲目时自动下载并本地缓存
  - 支持一键缓存当前 Collection 全部音乐
- **托盘常驻**
  - 主窗口关闭时，隐藏到系统托盘
  - 支持托盘菜单
- **单实例运行**
  - 重复启动会唤醒已有窗口，避免多开冲突
- **数据安全**
  - 应用只访问 Mojang 官方资源服务器

## 使用方式

1. 启动应用，进入主页点击播放按钮
2. 在设置页选择 Collection，并按需要调整 `minWait` / `maxWait`（秒）
3. 可先执行“缓存当前 Collection 全部音乐”减少首次播放等待
4. 日常使用时直接关闭窗口即可，应用会继续在托盘中运行

## 技术架构

项目采用 **Tauri 2 + Vue 3 + TypeScript + Rust** 的混合架构。

### 1) 前端层（Vue）

- 负责界面交互、配置管理与播放循环调度
- 配置保存在浏览器端 `localStorage`（键：`yugen_conf`）
- 播放流程：
  1. 读取配置并获取 Collection 对应曲目列表
  2. 随机选曲，调用后端下载保障接口
  3. 通过本地 HTTP 音频地址进行播放
  4. 播放结束后随机等待，再进入下一轮

### 2) 桌面宿主层（Tauri / Rust）

- 提供系统能力与安全边界：
  - 系统托盘、菜单事件、窗口隐藏/显示
  - 单实例控制
  - Tauri command（供前端 invoke）
- 关键命令：
  - `request_music_async`：按 `music.json` 映射查 hash，下载曲目并落盘缓存
  - `get_music_bytes`：读取本地缓存文件字节（当前主流程主要使用 HTTP 方式播放）

### 3) 资源与缓存层

- 随包分发 `resources/music.json`，维护曲目路径与 hash 信息
- 下载源为 Mojang 资源服务器（基于 hash 规则拼接 URL）
- 缓存目录在应用数据目录下的 `assets` 子目录（按资源路径分层）
- 已缓存文件会进行 hash 校验，通过则跳过重复下载

### 4) 本地音频服务层（Axum）

- 应用启动时由 Rust 后台线程启动 `127.0.0.1:10454`
- 提供 `/assets/*path` 音频访问接口
- 支持 Range 请求，便于音频流式播放
- 前端使用类似 `http://127.0.0.1:10454/assets/<musicPath>` 的 URL 播放 OGG

## 播放逻辑说明（为何“像游戏”）

传统播放器强调“连续播放”，而 Minecraft 的 BGM 更像“环境事件”：

- 音乐出现不是高频、不是固定周期
- 同一场景中，安静时段比音乐更长
- 曲目出现更像“偶遇”

Yugen 通过“随机选曲 + 随机长等待”复现这种节奏，因此它更适合长期陪伴，而非短时高频听歌。

## 本地开发

### 环境要求

- Node.js（建议 LTS）
- Rust stable
- Tauri 2 构建依赖（按官方文档安装系统依赖）

### 常用命令

```bash
npm install
npm run tauri dev
```

打包：

```bash
npm run tauri build
```

## 项目结构（简要）

```text
src/
  App.vue
  views/
    MainContentPage.vue      # 主播放页（随机播放循环）
    SettingsPage.vue         # 配置与批量缓存
  components/
    AudioPlayer.vue          # 音频播放封装
  utils/
    utils.ts                 # collection映射、默认等待、invoke封装

src-tauri/src/
  main.rs                    # 程序入口（启动本地HTTP服务 + Tauri）
  lib.rs                     # 托盘、窗口行为、命令注册
  http.rs                    # 本地音频HTTP服务（Axum）
  commands/
    request_music_async.rs   # 下载与缓存逻辑
    get_music_bytes.rs       # 读取缓存字节
```

## 说明与免责声明

- 本项目为非官方的休闲氛围工具，不隶属于 Mojang / Microsoft。
- Minecraft 相关音乐与素材版权归其权利方所有，请仅用于个人、非商业、合规用途。

