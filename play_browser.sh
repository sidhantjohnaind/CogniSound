#!/usr/bin/env bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

echo "============================================================"
echo "   Starting Native Parallel Rust Music Backend Server       "
echo "============================================================"

BIN_PATH="rust_server/target/release/rust_server"
if [ ! -f "$BIN_PATH" ]; then
    echo "[INFO] Building Rust Server binary in release mode..."
    cargo build --release --manifest-path rust_server/Cargo.toml
fi

if command -v xdg-open > /dev/null; then
    xdg-open http://localhost:80/ &
fi

"$BIN_PATH"