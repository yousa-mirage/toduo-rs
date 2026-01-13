<div align="center">

![ToDuo-rs](https://socialify.git.ci/Yousa-Mirage/ToDuo-rs/image?custom_description=Events%2C+Lightweight%2C+Pure+TODO+Tool.+TUI+%2B+GUI+Dual+Mode%2C+Based+on+todo.txt&description=1&font=Bitter&forks=1&issues=1&language=1&logo=https%3A%2F%2Fgithub.com%2FYousa-Mirage%2FToDuo-rs%2Fblob%2Fmain%2Ftodo-gui%2Fsrc-tauri%2Ficons%2Ficon.png%3Fraw%3Dtrue&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

[English](README.md) | [简体中文](README_zh.md)

</div>

## 📖 Introduction

**ToDuo** (To-Do + Duo) is a high-performance, cross-platform TODO management toolkit written in Rust.

It strictly adheres to the standard `todo.txt` plain text protocol. Its core highlight is offering perfectly synchronized **TUI (Terminal User Interface)** and **GUI (Graphical User Interface)** modes.

Whether you are a modern user who prefers mouse clicks or a terminal geek who lives by the keyboard, ToDuo provides the most suitable experience for you, seamlessly switching between the two.

| Graphical UI (GUI) | Terminal UI (TUI) |
| :---: | :---: |
| ![GUI Screenshot](./.github/gui-preview.png) | ![TUI Screenshot](./.github/tui-preview.png) |
| *Modern, Clean, Intuitive* | *Hardcore, Efficient, Keyboard-driven* |

## ✨ Key Features

* **⚡ Blazing Fast**: Built with **Rust**, ensuring instant startup and minimal resource usage.
* **🌗 Dual Modes**:
  * **GUI Mode (ToDuo)**: Modern, elegant, intuitive. Supports system tray, global shortcuts, and dark mode.
  * **TUI Mode (td)**: Fast, pure. Supports Vim-style keybindings and mouse drag-and-drop.
* **📄 Data Sovereignty**: Uses plain text [todo.txt](http://todotxt.org/) format. Your data is yours—easy to backup, migrate, and version control.
* **🔧 Geek Friendly**: Provides powerful CLI piping support, easy to integrate into your scripts or workflows.

## 📦 Download & Install

### Method 1: Direct Download (Recommended)

Visit the [GitHub Releases](https://github.com/Yousa-Mirage/ToDuo-rs/releases) page to download the latest version for your system.

### Method 2: Build from Source

Requires [Rust](https://www.rust-lang.org/) , [Just](https://github.com/casey/just) , and [Bun](https://bun.sh/) .

```bash
# Clone repository
git clone https://github.com/Yousa-Mirage/ToDuo-rs.git
cd ToDuo-rs

# Install dependencies
just setup

# Build the program
just build
```

Binaries will be available in `target/release/`.

## 🚀 Quick Start

This project includes two independent applications. Choose freely based on your scenario:

### 🖥️ GUI Application

Ideal for daily office work and desktop environments.
👉 **[Read GUI Documentation](./todo-gui/README.md)**

* **Launch**: Run `ToDuo.exe`
* **Highlight**: Filter views, date pickers, system tray integration.

### 📟 TUI Application

Ideal for developers, servers, or keyboard-driven workflows.
👉 **[Read TUI Documentation](./todo-tui/README.md)**

* **Launch**: Run `td` in your terminal
* **Highlight**: Vim-style keys, mouse support, instant response.

## 🤝 Contribution

Contributions are welcome! Whether it's fixing bugs, adding features, or improving documentation.

---

<div align="center">

**MIT License** | **Made with ❤️ by <a href="https://github.com/Yousa-Mirage">Yousa-Mirage**</a>

</div>
