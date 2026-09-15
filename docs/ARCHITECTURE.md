# Architecture and extension map

```text
Browser / WebView
┌──────────────────────────────────────────────────────────┐
│ TypeScript product shell                                 │
│   DOM, network, orchestration, observability             │
│        │ coarse calls + typed arrays                     │
│        ▼                                                 │
│ Rust → wasm32-unknown-unknown                            │
│   parsing · simulation · DSP · generation · transforms   │
└──────────────────────┬───────────────────────────────────┘
                       │ HTTP + WebSocket
WSL2 / Linux           ▼
┌──────────────────────────────────────────────────────────┐
│ Axum + Tokio native Rust service                         │
│   jobs · host capabilities · persistence seam · events   │
└──────────────────────────────────────────────────────────┘
                       │ future: Wasmtime component host
                       ▼
                sandboxed WIT plugins
```

## Why this split

* **Browser Wasm is a compute core, not a DOM replacement.** Calls are chunky. Text analysis returns one payload; terrain returns one contiguous RGBA buffer; simulations remain in Rust for their entire loop.
* **Native Rust owns capabilities.** Networking, durable state, secrets, filesystem and long-running work belong on the Axum side.
* **WIT is the next isolation seam.** `components/analytics/wit/world.wit` sketches a WASI 0.3 async streaming plugin contract. It is intentionally not coupled to the stable app build while WASIp3 Rust target ergonomics evolve.
* **The shell remains replaceable.** The generated ESM ABI works with React, Svelte, Vue, Tauri webviews, Electron, or plain TypeScript.

## Production extension points

1. Add a `Web Worker` and transfer buffers for workloads above ~16 ms to preserve UI responsiveness.
2. Enable COOP/COEP and shared Wasm memory when adopting threads; keep a non-threaded artifact for compatibility.
3. Add Wasmtime with fuel, epoch interruption, memory limits and explicit capability linking for untrusted plugins.
4. Replace the in-memory jobs vector with PostgreSQL/SQLx and a durable queue.
5. Add OpenTelemetry tracing, Prometheus metrics, and W3C trace context across fetch/WebSocket boundaries.
6. Sign Wasm artifacts (Sigstore), emit SBOM/provenance, pin the Rust toolchain and run `cargo-deny`.
7. Benchmark boundary shape, not just algorithm loops. Track cold instantiation, steady-state, transfer cost and memory growth.
