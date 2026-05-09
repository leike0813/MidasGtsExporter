# MidasGtsExporter Rust/Tauri 版

这是 `MidasGtsExporter` 的 Rust + Tauri 桌面端复刻实现，用于读取 Midas GTS NX 导出的 `.fpn` 网格数据，并导出为 FLAC3D、Abaqus 或 LS-DYNA 格式。

原 C# WPF 项目地址：

https://gitee.com/wdhust/MidasGtsExporter/tree/master

本项目优先复刻原项目已经实现的行为，包含部分原项目兼容性细节。已知兼容行为和约束记录在 `docs/` 与 `openspec/` 中。

## 当前功能

- 解析 Midas GTS `.fpn` 文件。
- 保留原项目的节点、单元、分组解析行为。
- 导出 FLAC3D：
  - `<output>_Mesh.f3grid`
  - 有 1D 单元/分组时生成 `<output>_Geom.geom`
- 导出 Abaqus：
  - `<output>.inp`
- 导出 LS-DYNA：
  - 主 `.k` 文件
  - include 文件：`_Part.k`、`_Node.k`、`_Elem.k`、`_NodeList.k`、`_Segment.k`
- 提供 Tauri 桌面 GUI：
  - 选择输入 `.fpn`
  - 选择输出目录
  - 设置输出文件名
  - 选择导出格式
  - 显示进度、日志和耗时

## 暂不支持

- Ansys 导出。
- 对原项目兼容性 quirks 的主动修正。
- 真实 C# 导出结果的逐字节 golden 对比。
- 自动修复格式异常的 `.fpn` 文件。

## 项目结构

```text
src/                         React + TypeScript 前端
src-tauri/                   Tauri v2 + Rust 后端
src-tauri/src/fpn_reader.rs  .fpn 解析器
src-tauri/src/exporters/     FLAC3D / Abaqus / LS-DYNA 导出器
src-tauri/src/commands.rs    Tauri convert_mesh command
tests/fixtures/midas_gts/    测试用 .fpn 文件
docs/                        Roadmap、contracts、兼容性说明
openspec/                    OpenSpec change 与 spec 文档
```

`references/` 目录只用于本地对照原项目，不作为仓库必需内容。测试 fixture 已迁移到 `tests/fixtures/midas_gts/`。

## 开发环境

需要安装：

- Node.js LTS
- npm
- Rust stable
- Tauri v2 对应平台依赖

Windows 打包建议使用 MSVC 工具链：

```powershell
rustup default stable-x86_64-pc-windows-msvc
```

还需要安装：

- Visual Studio 2022 Build Tools，并勾选 `Desktop development with C++`
- Microsoft Edge WebView2 Runtime

## 安装依赖

```sh
npm install
```

## 测试

Rust 后端测试：

```sh
cargo test --manifest-path src-tauri/Cargo.toml
```

前端构建检查：

```sh
npm run build
```

OpenSpec 校验：

```sh
openspec validate implement-phase1-rust-core
openspec validate implement-phase2-exporters
openspec validate implement-phase3-tauri-gui
```

## 本地运行

开发模式：

```sh
npm run tauri -- dev --features app
```

注意：不要只运行 `npm run dev` 来测试完整功能，因为文件选择和转换 command 需要 Tauri 环境。

## 打包

Windows：

```powershell
npm run tauri -- build --features app
```

调试包：

```powershell
npm run tauri -- build --debug --features app
```

打包产物通常位于：

```text
src-tauri\target\release\bundle\
```

Linux 打包需要安装 WebKitGTK、libsoup 等 Tauri 系统依赖；如果只是为 Windows 发布，建议直接在 Windows 原生环境打包。

## 图标

应用图标由项目内 `app-icon.png` 生成：

```sh
npm run tauri -- icon app-icon.png
```

生成结果位于 `src-tauri/icons/`。

## 兼容性说明

本项目第一版以“行为复刻”为优先目标，不主动修正原 C# 项目中的兼容性细节。主要契约文档：

- `docs/contracts/fpn-parser-contract.md`
- `docs/contracts/exporter-contract.md`
- `docs/contracts/tauri-command-contract.md`
- `docs/compatibility/midas-gts-exporter-csharp.md`

如后续发现真实文件导出差异，应先补充 fixture 或 OpenSpec change，再决定是修复 Rust 端问题，还是保留原项目兼容行为。
