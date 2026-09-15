FROM rust:1.98-bookworm AS rust-build
WORKDIR /src
RUN rustup target add wasm32-unknown-unknown && cargo install wasm-bindgen-cli --version 0.2.128 --locked
COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY apps apps
RUN cargo build --release -p wasmforge-engine --target wasm32-unknown-unknown && \
    wasm-bindgen --target web --out-dir web-pkg --out-name wasmforge_engine target/wasm32-unknown-unknown/release/wasmforge_engine.wasm && \
    cargo build --profile server-release -p wasmforge-server
FROM node:20-bookworm AS web-build
WORKDIR /src/web
COPY web/package*.json ./
RUN npm ci
COPY web ./
COPY --from=rust-build /src/web-pkg ./public/pkg
RUN npm run build
FROM debian:bookworm-slim
RUN useradd -r -u 10001 forge
WORKDIR /app
COPY --from=rust-build /src/target/server-release/wasmforge-server /usr/local/bin/wasmforge
COPY --from=web-build /src/web/dist ./web/dist
USER forge
ENV PORT=3000 STATIC_DIR=web/dist RUST_LOG=wasmforge_server=info
EXPOSE 3000
HEALTHCHECK CMD ["/bin/sh","-c","wget -q -O- http://127.0.0.1:3000/api/health || exit 1"]
ENTRYPOINT ["wasmforge"]
