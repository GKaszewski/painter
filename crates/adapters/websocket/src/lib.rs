mod handler;
mod messages;

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use application::AppState;
use axum::{Router, routing::get};
use tokio::sync::Semaphore;

pub(crate) struct WsState {
    app_state: Arc<AppState>,
    connection_counter: AtomicU64,
    canvas_send_semaphore: Arc<Semaphore>,
}

pub fn build_router(state: Arc<AppState>, max_concurrent_canvas_sends: usize) -> Router {
    let ws_state = Arc::new(WsState {
        app_state: state,
        connection_counter: AtomicU64::new(0),
        canvas_send_semaphore: Arc::new(Semaphore::new(max_concurrent_canvas_sends)),
    });

    Router::new()
        .route("/ws", get(handler::ws_upgrade))
        .with_state(ws_state)
}
