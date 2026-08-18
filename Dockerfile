FROM oven/bun:1 AS frontend
WORKDIR /app/painter-js
COPY painter-js/package.json painter-js/bun.lock ./
RUN bun install --frozen-lockfile
COPY painter-js/ .
ENV VITE_TRANSPORT=websocket
RUN bun run build

FROM rust:1-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN mkdir -p painter-js/dist && echo '<html></html>' > painter-js/dist/index.html
RUN cargo build --release --bin server --no-default-features --features websocket 2>&1 || true

COPY --from=frontend /app/painter-js/dist ./painter-js/dist
RUN touch crates/adapters/http-axum/src/routes.rs && \
    cargo build --release --bin server --no-default-features --features websocket

FROM scratch
COPY --from=builder /app/target/release/server /painter
EXPOSE 3000
ENTRYPOINT ["/painter"]
