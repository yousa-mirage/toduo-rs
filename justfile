# 1. 变量定义：根据操作系统自动判断二进制后缀

bin_ext := if os() == "windows" { ".exe" } else { "" }

default:
    @just --list

# ==========================================
# 🚀 构建命令 (Build)
# ==========================================

# 一键安装依赖
setup:
    @echo "📦 Installing frontend dependencies..."
    @cd todo-gui && bun install

# 一键构建所有
build: build-tui build-gui
    @echo "🎉 All builds finished!"
    @just show-paths

# 构建 TUI
build-tui:
    @echo "🦀 Building TUI (CLI) Release..."
    @cargo build --release --package todo-tui

# 构建 GUI
build-gui:
    @echo "🖥️  Building GUI (Tauri) Release..."
    @cd todo-gui && bun tauri build

# ==========================================
# 🛠️ 辅助命令 (Utils)
# ==========================================

# 清理缓存
clean:
    @echo "🧹 Cleaning artifacts..."
    @cargo clean
    @{{ if os() == "windows" { "powershell -c \"if (Test-Path todo-gui/dist) { rm -r -fo todo-gui/dist }\"" } else { "rm -rf todo-gui/dist" } }}
    @echo "✨ Cleaned!"

# 格式化与检查
check:
    @echo "🦀 Rust: Formatting & Clippy..."
    @cargo fmt --all
    @cargo clippy --workspace --all-targets --all-features -- -D warnings
    @echo "🖥️  Frontend: Biome Check & Type-check..."
    @cd todo-gui && bun run lint && bun run type-check

# 显示路径
show-paths:
    @echo ""
    @echo "📂 Output Locations:"
    @echo "   👉 TUI: target/release/td{{ bin_ext }}"
    @echo "   👉 GUI: target/release/ToDuo{{ bin_ext }}"
    @echo ""

# 启动开发环境
dev:
    @echo "🔥 Starting dev server..."
    @cd todo-gui && bun tauri dev
