# Security posture

The Wasm sandbox reduces blast radius; it is not a substitute for validation. The native API validates request shape and size seams, uses panic containment, request IDs, secure MIME handling, compression and graceful shutdown. Before public deployment: restrict CORS, add authentication/authorization, body/time/concurrency limits, TLS at the proxy, persistent rate limiting, dependency policy (`cargo-deny`, `npm audit`), artifact signing, CSP/COOP/COEP headers, and structured audit events. Never embed secrets in browser Wasm: users can inspect its bytes and memory.
