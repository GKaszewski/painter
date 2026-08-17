mod handlers;

use std::sync::Arc;

use application::AppState;
use socketioxide::{SocketIo, extract::SocketRef};

pub fn setup_namespaces(io: &SocketIo, state: Arc<AppState>) {
    io.ns("/", move |socket: SocketRef| async move {
        handlers::on_connect(socket, state).await;
    });
}
