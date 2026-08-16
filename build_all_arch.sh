#!/usr/bin/env bash
# ==============================================================================
# Sonar Multi-Architecture Cross-Compilation Script (Bash / Linux / macOS)
# Builds binaries for AMD64 (x86_64), ARM64 (aarch64), and RISC-V (riscv64gc)
# ==============================================================================

set -e

echo "============================================================"
echo " 🛠️  Sonar Multi-Architecture Compiler (AMD64 / ARM64 / RISC-V)"
echo "============================================================"

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "riscv64gc-unknown-linux-gnu"
)

# Check if cross is installed
if ! command -v cross &> /dev/null; then
    echo "[INFO] 'cross' tool not detected. Installing cross for containerized multi-arch builds..."
    cargo install cross --git https://github.com/cross-rs/cross
fi

for TARGET in "${TARGETS[@]}"; do
    echo ""
    echo "📦 Building release binaries for: $TARGET"
    rustup target add "$TARGET" || true
    cross build --release --target "$TARGET" --manifest-path rust_server/Cargo.toml --bins
done

echo ""
echo "✅ All multi-architecture binaries built successfully!"
