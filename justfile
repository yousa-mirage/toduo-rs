# 1. 变量定义：根据操作系统自动判断二进制后缀
bin_ext := if os() == "windows" { ".exe" } else { "" }

default:
    @just --list

# ==========================================
# 🚀 构建命令 (Build)
# ==========================================

# 一键构建所有
build: build-tui build-gui
    @echo "🎉 All builds finished!"
    @just show-paths

# 构建 TUI
build-tui:
    @echo "🦀 Building TUI (CLI) Release..."
    cargo build --release -p todo-tui

# 构建 GUI
build-gui:
    @echo "🖥️  Building GUI (Tauri) Release..."
    cd todo-gui && bun tauri build

# ==========================================
# 🛠️ 辅助命令 (Utils)
# ==========================================

# 清理缓存
clean:
    @echo "🧹 Cleaning artifacts..."
    cargo clean
    -rm -rf todo-gui/dist

# 格式化与检查
check:
    @echo "🔍 Running checks..."
    cargo fmt --all
    cargo clippy --workspace --all-targets -- -D warnings
    cd todo-gui && bun run lint
    @echo "✅ All checks passed!"

# 显示路径 (利用变量 bin_ext 实现跨平台显示)
show-paths:
    @echo ""
    @echo "📂 Output Locations:"
    @echo "   👉 TUI: target/release/td{{bin_ext}}"
    @echo "   👉 GUI: target/release/ToDuo{{bin_ext}}"
    @echo "   👉 Installer: target/release/bundle/"