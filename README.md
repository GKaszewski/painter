# Painter

A collaborative pixel canvas inspired by r/place. Users connect in real-time and place colored pixels on a shared 500x500 canvas.

## Quick Start

```bash
# Install frontend dependencies and build
cd painter-js && bun install && bun run build && cd ..

# Run the server (serves embedded frontend)
cargo run --release
```

Open `http://localhost:3000` in your browser.

## Configuration

All configuration is via environment variables (or `.env` file):

| Variable | Default | Description |
|---|---|---|
| `ADDRESS` | `0.0.0.0` | Bind address |
| `PORT` | `3000` | Bind port |
| `ENABLE_CORS` | `true` | Enable CORS headers |
| `CANVAS_WIDTH` | `500` | Canvas width in pixels |
| `CANVAS_HEIGHT` | `500` | Canvas height in pixels |
| `COOLDOWN_SECS` | `10` | Seconds between pixel placements per user |
| `RATE_LIMIT_BURST` | `10` | HTTP rate limit burst size |
| `RATE_LIMIT_PER_SECOND` | `10` | HTTP rate limit per second |
| `BROADCAST_CAPACITY` | `1024` | Broadcast channel buffer size |
| `SNAPSHOT_ENABLED` | `true` | Enable periodic canvas snapshots |
| `SNAPSHOT_INTERVAL_SECS` | `300` | Seconds between snapshots |
| `SNAPSHOT_MAX` | `5` | Maximum snapshot files to keep |
| `SNAPSHOT_DIR` | `snapshots/` | Snapshot storage directory |

## Transport Adapters

The server supports two real-time transport protocols, selected at compile time:

```bash
# Socket.IO (default)
cargo run --release

# Native WebSocket
cargo run --release --no-default-features --features websocket
```

The frontend auto-detects via `VITE_TRANSPORT` environment variable (`socketio` or `websocket`).

## Docker

```bash
# Build
docker build -t painter .

# Run
docker run -p 3000:3000 painter

# With custom config
docker run -p 3000:3000 \
  -e CANVAS_WIDTH=1000 \
  -e CANVAS_HEIGHT=1000 \
  -e COOLDOWN_SECS=30 \
  -v ./snapshots:/app/snapshots \
  painter
```

The Docker image is a single statically-linked binary on `scratch` — **~3MB** total, no OS, no runtime dependencies. The frontend is embedded at compile time.

The server shuts down gracefully on SIGTERM/SIGINT (Ctrl+C) — in-flight connections are drained and a final canvas snapshot is saved before exit.

## Development

```bash
# Backend (watches for changes)
RUST_LOG=debug cargo run

# Frontend (Vite dev server with HMR)
cd painter-js
VITE_IS_DEBUG=true bun run dev

# Run all checks (fmt + clippy + tests)
make check

# Format
make fmt
cd painter-js && bun run fmt
```

## Architecture

Hexagonal architecture with clean dependency boundaries. See [architecture.mmd](architecture.mmd) for the full diagram.

```
crates/
  config/              Config structs, ConfigSource trait
  domain/              Canvas, value objects, port traits, events
  application/         Use cases, AppState, InProcessBroadcaster
  api-types/           Shared DTOs, event constants
  adapters/
    config-env/        Environment variable config loader
    canvas-file/       File-based snapshot persistence
    http-axum/         HTTP routes, rate limiting, embedded static files
    socketio/          Socket.IO transport adapter
    websocket/         Native WebSocket transport adapter
  server/              Composition root
```

All infrastructure is behind port traits:

| Port | Adapter | Swappable to |
|---|---|---|
| `CanvasStore` | In-memory | Redis, mmap'd file |
| `CanvasPersistence` | File system | S3, SQLite |
| `EventBroadcaster` | tokio broadcast | NATS, Redis pub/sub |
| `ConfigSource` | Env vars | TOML, JSON, remote API |

## License

[MIT](LICENSE)
