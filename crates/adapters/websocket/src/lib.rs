mod handler;
mod messages;

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use application::AppState;
use axum::{Router, routing::get};

pub(crate) struct WsState {
    app_state: Arc<AppState>,
    connection_counter: AtomicU64,
}

pub fn build_router(state: Arc<AppState>) -> Router {
    let ws_state = Arc::new(WsState {
        app_state: state,
        connection_counter: AtomicU64::new(0),
    });

    Router::new()
        .route("/ws", get(handler::ws_upgrade))
        .with_state(ws_state)
}
