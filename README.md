
# WasmForge

**A Rust + WebAssembly systems laboratory built for WSL2.** It is a working reference architecture—not a static mock—for the “product shell + performance core” pattern.

![Rust](https://img.shields.io/badge/Rust-1.98-orange) ![Wasm](https://img.shields.io/badge/WebAssembly-browser--native-654ff0) ![WSL](https://img.shields.io/badge/WSL2-Ubuntu-4eaa25)

## What is real in the demo

| Surface | Runtime | Demonstrates |
|---|---|---|
| Text intelligence | Rust → browser Wasm | Unicode tokenization, frequency analysis, stable fingerprint, one batched JSON return |
| Monte Carlo π | Rust → browser Wasm | Seeded deterministic simulation with up to 5M iterations per call |
| Procedural terrain | Rust → browser Wasm | 237,600 generated pixels returned as one contiguous typed RGBA array |
| Signal processor | Rust → browser Wasm | Typed-array DSP ABI ready for audio/telemetry demos |
| Sort kernel | Rust → browser Wasm | Numeric buffer transform with defined NaN ordering |
| Health/system API | Native Rust on WSL | Axum, Tokio and host capability access |
| Job queue | Native Rust on WSL | Validated async task dispatch with status transitions |
| Live stream | Native Rust on WSL | WebSocket telemetry with reconnect-friendly message envelopes |
| Plugin contract | WASI 0.3/WIT seam | Future async streaming Component Model architecture |

The dashboard measures its own Wasm startup and operation latency. Results shown after the engine boots are produced on your machine.

## Quick start on WSL2

```bash
# From the repository root inside Ubuntu/WSL
./scripts/wsl-setup.sh
./target/server-release/wasmforge-server
# Open http://localhost:3000 in Windows
```

If Rust and Node 20+ are already installed:

```bash
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.128 --locked
npm ci --prefix web
make build
./target/server-release/wasmforge-server
```

See **[docs/WSL.md](docs/WSL.md)** for the Windows-first walkthrough.

## Repository map

```text
crates/engine/       dependency-light Rust compute core compiled to Wasm
apps/server/         native Axum/Tokio service running in Linux/WSL
web/                 TypeScript product shell, visual lab and benchmark UI
components/*/wit/    forward-looking WASI 0.3 async component contracts
docs/                architecture, extension map and WSL runbook
scripts/              reproducible environment bootstrap
```

## Commands

```bash
make wasm             # compile Rust Wasm + generate ESM bindings
make web              # typecheck and optimize Vite frontend
make server           # optimized native service
make build            # all three in dependency order
make test             # Rust tests + TypeScript production build
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

For the two-process development loop, run `cargo run -p wasmforge-server` and `npm run dev --prefix web`. Vite proxies HTTP and WebSockets to Axum.

## API

```text
GET  /api/health       liveness, version, uptime and runtime identity
GET  /api/system       WSL/Linux host capability snapshot
GET  /api/jobs         current process-local demonstration jobs
POST /api/jobs         queue an async demonstration job
POST /api/echo         structured native-boundary round trip
GET  /api/stream       WebSocket telemetry channel
```

Example:

```bash
curl -s http://localhost:3000/api/health | jq
curl -s -X POST http://localhost:3000/api/jobs \
  -H 'content-type: application/json' \
  -d '{"kind":"thumbnail-batch","payload":{"count":24}}' | jq
```

## Design rules embodied here

1. **Cross the boundary less.** The expensive loop or full parse stays inside Wasm; results cross once.
2. **Use buffers for bulk data.** Pixels and signals use generated typed-array bindings rather than per-element calls.
3. **Keep capabilities native.** Files, secrets, databases, sockets and durable queues stay in the server.
4. **Make determinism testable.** Seeded workloads and stable output make regression tests meaningful.
5. **Ship graceful degradation.** The shell explains a missing Wasm build; APIs expose explicit health.
6. **Keep deployment boring.** One native binary serves optimized static assets, `.wasm`, APIs and WebSockets.
7. **Treat future standards as seams, not blockers.** WIT/WASI 0.3 is modeled without destabilizing today’s browser build.

## Cutting-edge path from this baseline

The current stable app intentionally favors broad browser support. The next experimental branch can add:

- **Web Workers + transferable buffers** for off-main-thread kernels.
- **Wasm SIMD and relaxed SIMD** for image/audio/vector workloads, with runtime feature detection.
- **Wasm threads** under COOP/COEP, backed by shared memory and a compatibility artifact.
- **Wasmtime component hosting** with resource limits, fuel, epoch interruption and explicit WASI capabilities.
- **WASI 0.3 async components** implementing `components/analytics/wit/world.wit` once your selected Rust toolchain supports the desired target ergonomically.
- **Tauri 2 shell** reusing this exact frontend and moving privileged commands to the Rust desktop core.
- **Edge artifact** for Cloudflare Workers, Fastly or Fermyon Spin, sharing pure Rust domain crates.
- **WebGPU interop** where Wasm prepares compact buffers and the GPU performs massively parallel visualization.

Read **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** for a production hardening and extension checklist.

## Container

```bash
docker compose up --build
```

The final image runs as a non-root user with a read-only filesystem recommendation, health check and single exposed port.

## Reality checks

WebAssembly is not automatically faster than optimized JavaScript, boundary crossings are not free, and browser Wasm cannot hide secrets. Benchmark representative payloads, include instantiation and transfer costs, and choose Rust where predictability, correctness, portability or dense computation justify it.

Licensed under MIT OR Apache-2.0.
