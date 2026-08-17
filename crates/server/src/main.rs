#[cfg(not(any(feature = "socketio", feature = "websocket")))]
compile_error!("Enable either the `socketio` or `websocket` feature");

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use application::{AppState, InMemoryCanvasStore, InProcessBroadcaster};
use canvas_file::FileCanvasPersistence;
use config::{AppConfig, ConfigSource};
use config_env::EnvConfigSource;
use domain::BroadcastEvent;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;

    let config = EnvConfigSource.load()?;
    let state = build_state(&config)?;

    if config.snapshot.enabled {
        if let Err(err) = application::canvas::restore_snapshot::execute(&state) {
            warn!("Failed to restore canvas snapshot: {err}");
        }
        spawn_snapshot_scheduler(state.clone(), config.snapshot.interval_secs);
    }

    let app = build_app(state.clone(), &config)?;

    let server_address = format!("{}:{}", config.server.address, config.server.port);
    info!("Starting server on {server_address}");

    let listener = tokio::net::TcpListener::bind(server_address).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    info!("Shutting down gracefully...");

    if config.snapshot.enabled {
        info!("Saving final canvas snapshot...");
        if let Err(err) = application::canvas::save_snapshot::execute(&state) {
            error!("Failed to save final snapshot: {err}");
        }
    }

    info!("Server stopped");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => info!("Received Ctrl+C"),
        () = terminate => info!("Received SIGTERM"),
    }
}

fn build_state(config: &AppConfig) -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let (broadcast_tx, _) = broadcast::channel::<BroadcastEvent>(config.broadcast.channel_capacity);

    let canvas_store = InMemoryCanvasStore::new(config.canvas.width, config.canvas.height);
    let broadcaster = InProcessBroadcaster::new(broadcast_tx);
    let cooldown = Duration::from_secs(config.cooldown.placement_secs);

    let mut state = AppState::new(Box::new(canvas_store), Box::new(broadcaster), cooldown);

    if config.snapshot.enabled {
        let persistence = FileCanvasPersistence::new(&config.snapshot)?;
        state = state.with_persistence(Box::new(persistence));
    }

    Ok(Arc::new(state))
}

fn spawn_snapshot_scheduler(state: Arc<AppState>, interval_secs: u64) {
    let interval = Duration::from_secs(interval_secs);
    info!("Snapshot scheduler started (every {interval_secs}s)");

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            if let Err(err) = application::canvas::save_snapshot::execute(&state) {
                error!("Failed to save canvas snapshot: {err}");
            }
        }
    });
}

#[cfg(feature = "socketio")]
fn build_app(
    state: Arc<AppState>,
    config: &AppConfig,
) -> Result<axum::Router, Box<dyn std::error::Error>> {
    let (layer, io) = socketioxide::SocketIo::new_layer();
    socketio::setup_namespaces(&io, state);
    let router = http_axum::build_router(config.server.enable_cors, &config.rate_limit)?;
    Ok(router.layer(layer))
}

#[cfg(feature = "websocket")]
fn build_app(
    state: Arc<AppState>,
    config: &AppConfig,
) -> Result<axum::Router, Box<dyn std::error::Error>> {
    let ws_router = websocket::build_router(state);
    let http_router = http_axum::build_router(config.server.enable_cors, &config.rate_limit)?;
    Ok(ws_router.merge(http_router))
}
