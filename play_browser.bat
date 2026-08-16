@echo off
setlocal
cd /d "%~dp0"
echo ============================================================
echo   Starting Native Parallel Rust Music Backend Server
echo ============================================================

set BIN_PATH=target\release\rust_server.exe
if not exist "%BIN_PATH%" set BIN_PATH=rust_server\target\release\rust_server.exe
if not exist "%BIN_PATH%" set BIN_PATH=D:\temp\rust\target\release\rust_server.exe
if not exist "%BIN_PATH%" set BIN_PATH=target\debug\rust_server.exe
if not exist "%BIN_PATH%" set BIN_PATH=rust_server\target\debug\rust_server.exe
if not exist "%BIN_PATH%" (
    echo [INFO] Building Rust Server binary in release mode...
    cargo build --release --manifest-path rust_server\Cargo.toml
    if exist "target\release\rust_server.exe" set BIN_PATH=target\release\rust_server.exe
    if exist "rust_server\target\release\rust_server.exe" set BIN_PATH=rust_server\target\release\rust_server.exe
    if exist "D:\temp\rust\target\release\rust_server.exe" set BIN_PATH=D:\temp\rust\target\release\rust_server.exe
)

start "" http://localhost:80/
"%BIN_PATH%"
