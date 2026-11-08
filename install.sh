#!/usr/bin/env bash
# yami-view: build and install into ~/.local, no root needed. Needs Rust 1.89 or newer.
#   ./install.sh               build and install
#   ./install.sh --deps        what it needs from the system
#   ./install.sh --uninstall   remove it
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
PREFIX="${PREFIX:-$HOME/.local}"
DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
case "${1:-}" in
  --deps) echo "wayland, libxkbcommon"; exit 0 ;;
  --uninstall)
    rm -f "$PREFIX/bin/yami-view" "$DATA/applications/yami-view.desktop"
    echo "yami-view removed"; exit 0 ;;
esac
cargo build --release --locked
install -Dm755 target/release/yami-view "$PREFIX/bin/yami-view"
install -Dm644 config/yami-view.desktop "$DATA/applications/yami-view.desktop"
echo "yami-view installed to $PREFIX/bin"
