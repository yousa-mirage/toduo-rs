<div align="center">

![ToDuo-rs](https://socialify.git.ci/Yousa-Mirage/ToDuo-rs/image?custom_description=%E4%BC%98%E9%9B%85%E3%80%81%E8%BD%BB%E9%87%8F%E3%80%81%E7%BA%AF%E7%B2%B9%E7%9A%84+TODO+%E7%AE%A1%E7%90%86%E5%B7%A5%E5%85%B7%EF%BC%8CTUI%2BGUI+%E5%8F%8C%E6%A8%A1%E5%BC%8F%EF%BC%8C%E5%9F%BA%E4%BA%8E+todo.txt&description=1&font=Bitter&forks=1&issues=1&language=1&logo=https%3A%2F%2Fgithub.com%2FYousa-Mirage%2FToDuo-rs%2Fblob%2Fmain%2Ftodo-gui%2Fsrc-tauri%2Ficons%2Ficon.png%3Fraw%3Dtrue&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

[English](README.md) | [简体中文](README_zh.md)

</div>

## 📖 简介

**ToDuo** (To-Do + Duo) 是一个高性能、跨平台的 TODO 任务管理工具，使用 Rust 编写。它基于经典的 `todo.txt` 纯文本协议，核心亮点在于 **TUI + GUI 的双模式**。

无论你是习惯鼠标拖拽的**效率办公者**，还是手指不离键盘的**硬核极客**，ToDuo 都能提供最适合你的交互方式：

* **GUI 模式**: 现代、优雅、直观。
* **TUI 模式**: 极速、纯粹、高效。

两者共享同一份 todo.txt 数据核心，无缝切换，如影随形。

| 图形界面 (GUI) | 终端界面 (TUI) |
| :---: | :---: |
| ![GUI Screenshot](./.github/gui-preview.png) | ![TUI Screenshot](./.github/tui-preview.png) |
| *现代、清爽、直观* | *硬核、高效、键盘驱动* |

## ✨ 核心特性 (Features)

* **⚡ 极致性能**: 基于 **Rust** 构建，启动速度快如闪电，内存占用极低。
* **🌗 双模形态**:
  * **ToDuo (GUI)**: 基于 **Tauri 2 + Vue 3**。极简的视觉设计，支持系统托盘、快捷键唤醒、深色模式。
  * **td (TUI)**: 基于 **Ratatui**。为终端而生，支持 Vim 风格键位，像写代码一样管理任务。
* **📄 数据自主**: 完全兼容标准 [todo.txt](http://todotxt.org/) 格式，数据为纯文本，本地优先。
* **🔧 极客友好**: 提供强大的 CLI 管道命令支持，轻松集成到你的脚本或工作流中。

## 📦 下载与安装

### 方式 1: 直接下载 (推荐)

请前往 [GitHub Releases](https://github.com/Yousa-Mirage/ToDuo-rs/releases) 页面下载适合您系统的最新版本。

### 方式 2: 从源码构建

需要安装 [Rust](https://www.rust-lang.org/) 、[Just](https://github.com/casey/just) 、[Bun](https://bun.sh/)。

```bash
# 克隆仓库
git clone https://github.com/Yousa-Mirage/ToDuo-rs.git
cd ToDuo-rs

# 安装依赖
just setup

# 构建程序
just build
```

构建产物位于 `target/release/` 目录下。

## 🚀 快速开始

本项目包含两个独立的应用程序，你可以根据场景选择使用：

### 🖥️ 图形界面 (GUI)

适合日常办公和桌面环境使用。
👉 **[查看 GUI 使用说明](./todo-gui/README_zh.md)**

* **启动**: 运行 `ToDuo.exe`
* **特性**: 过滤视图、日期选择器、系统托盘常驻。

### 📟 终端界面 (TUI)

适合开发者、服务器环境或纯键盘操作。
👉 **[查看 TUI 使用说明](./todo-tui/README_zh.md)**

* **启动**: 在终端运行 `td`
* **特性**: Vim 风格键位、鼠标支持、极速响应。

## 🤝 贡献

非常欢迎贡献代码！无论是修复 Bug、增加新功能，还是改进文档。请参考 `CONTRIBUTING.md` 了解详情。

---

<div align="center">

**MIT License** | **Made with ❤️ by <a href="https://github.com/Yousa-Mirage">Yousa-Mirage**</a>

</div>
