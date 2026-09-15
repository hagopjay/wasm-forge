#!/usr/bin/env bash
set -euo pipefail
command -v curl >/dev/null || { sudo apt-get update && sudo apt-get install -y curl build-essential pkg-config; }
if ! command -v rustup >/dev/null; then curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; fi
source "$HOME/.cargo/env"
rustup toolchain install stable --profile minimal
rustup default stable
rustup target add wasm32-unknown-unknown
command -v wasm-bindgen >/dev/null || cargo install wasm-bindgen-cli --locked
if ! command -v node >/dev/null; then echo "Install Node 20+ (recommended: fnm or nvm), then rerun."; exit 1; fi
(cd web && npm ci)
make build
echo "Ready. Run: ./target/server-release/wasmforge-server"
