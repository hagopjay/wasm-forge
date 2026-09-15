SHELL := /bin/bash
.PHONY: setup wasm web server build dev test clean audit
setup:
	./scripts/wsl-setup.sh
wasm:
	cargo build --release -p wasmforge-engine --target wasm32-unknown-unknown
	wasm-bindgen --target web --out-dir web/public/pkg --out-name wasmforge_engine target/wasm32-unknown-unknown/release/wasmforge_engine.wasm
web:
	cd web && npm run build
server:
	cargo build --profile server-release -p wasmforge-server
build: wasm web server
dev:
	@echo "Terminal 1: cargo run -p wasmforge-server"; echo "Terminal 2: cd web && npm run dev"
test:
	cargo test --workspace
	cd web && npm run build
clean:
	cargo clean; rm -rf web/dist web/public/pkg
