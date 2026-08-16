# ==============================================================================
# CogniSound Multi-Architecture Cross-Compilation Script (PowerShell / Windows)
# Builds binaries for AMD64 (x86_64), ARM64 (aarch64), and RISC-V (riscv64gc)
# ==============================================================================

param(
    [string]$Target = "all"
)

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " 🛠️  CogniSound Multi-Architecture Compiler (AMD64 / ARM64 / RISC-V)" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

$targets = @{
    "amd64-windows" = "x86_64-pc-windows-msvc"
    "arm64-windows" = "aarch64-pc-windows-msvc"
    "amd64-linux"   = "x86_64-unknown-linux-gnu"
    "arm64-linux"   = "aarch64-unknown-linux-gnu"
    "riscv64-linux" = "riscv64gc-unknown-linux-gnu"
}

# 1. Check if 'cross' is available for non-host Linux targets
$hasCross = (Get-Command cross -ErrorAction SilentlyContinue) -ne $null

foreach ($key in $targets.Keys) {
    $triple = $targets[$key]
    
    if ($Target -ne "all" -and $Target -ne $key -and $Target -ne $triple) {
        continue
    }

    Write-Host "`n📦 Compiling for $key ($triple)..." -ForegroundColor Yellow

    # Add rustup target if needed
    rustup target add $triple 2>$null

    if ($triple -like "*windows*") {
        cargo build --release --target $triple --manifest-path rust_server\Cargo.toml --bins
    } else {
        if ($hasCross) {
            cross build --release --target $triple --manifest-path rust_server\Cargo.toml --bins
        } else {
            Write-Host " ℹ️ Native Linux / RISC-V target requires 'cross' (cargo install cross) with Docker/Podman." -ForegroundColor DarkYellow
            cargo check --target $triple --manifest-path rust_server\Cargo.toml
        }
    }
}

Write-Host "`n✅ Build process completed!" -ForegroundColor Green
