# Windows + WSL2 runbook

## One-time Windows setup (PowerShell as Administrator)

```powershell
wsl --install -d Ubuntu-24.04
wsl --update
wsl --set-default-version 2
```

Restart when Windows asks. Open **Ubuntu**, then keep the repository inside the Linux filesystem (`~/code/wasmforge`), not `/mnt/c/...`; Cargo and Node file I/O are much faster there.

## Bootstrap inside WSL

```bash
sudo apt update
sudo apt install -y build-essential curl git pkg-config
# Clone/copy this repository, then:
cd ~/code/wasmforge
./scripts/wsl-setup.sh
./target/server-release/wasmforge-server
```

Open <http://localhost:3000> from Windows. WSL2 forwards localhost automatically on current Windows builds.

## Development loop

```bash
# terminal 1 — native Rust API
cargo run -p wasmforge-server

# terminal 2 — UI with API + WebSocket proxy
cd web && npm run dev
```

Open <http://localhost:5173>. After editing the Wasm crate, run `make wasm`; Vite will serve the refreshed package.

## Troubleshooting

* `wasm-bindgen` versions must match the crate in `Cargo.lock`. Reinstall with the exact lockfile version if a schema mismatch appears.
* If Windows cannot connect, verify `wsl hostname -I`, Windows firewall policy, and that the process binds `0.0.0.0`.
* Do not expose the permissive development CORS policy publicly. Replace it with an origin allowlist.
* For VS Code use **WSL: Open Folder in WSL** so rust-analyzer, terminals, and extensions execute in Linux.
