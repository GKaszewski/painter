use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use domain::{BroadcastEvent, BroadcastSubscription, Color, Position};
use flate2::Compression;
use flate2::write::GzEncoder;
use futures::{SinkExt, StreamExt, stream::SplitSink};
use tokio::sync::{Semaphore, mpsc};
use tracing::{error, info};

use crate::WsState;
use crate::messages::{ClientMessage, ServerMessage};
use application::AppState;
use application::canvas::place_pixel;

type WsSender = SplitSink<WebSocket, Message>;

pub async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_connection(socket, state))
}

async fn handle_connection(socket: WebSocket, state: Arc<WsState>) {
    let (mut sender, mut receiver) = socket.split();

    let connection_id = state
        .connection_counter
        .fetch_add(1, Ordering::Relaxed)
        .to_string();

    info!("WebSocket connected: {connection_id}");

    // Subscribe before snapshotting to avoid missing updates
    let subscription = state.app_state.broadcaster().subscribe();

    if !send_canvas_snapshot(&mut sender, &state.app_state, &state.canvas_send_semaphore).await {
        return;
    }

    application::soldiers::connect::execute(&state.app_state, connection_id.clone());

    let (error_sender, error_receiver) = mpsc::unbounded_channel::<String>();

    let mut send_task = tokio::spawn(run_send_loop(sender, subscription, error_receiver));

    let app_state = state.app_state.clone();
    let recv_connection_id = connection_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(text) = message {
                handle_client_message(&app_state, &error_sender, &recv_connection_id, &text);
            } else if let Message::Close(_) = message {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    info!("WebSocket disconnected: {connection_id}");
    application::soldiers::disconnect::execute(&state.app_state, &connection_id);
}

async fn send_canvas_snapshot(
    sender: &mut WsSender,
    state: &AppState,
    semaphore: &Semaphore,
) -> bool {
    let Ok(_permit) = semaphore.acquire().await else {
        return false;
    };

    let pixels = application::canvas::get_state::execute(state);
    let raw_bytes = Color::collect_as_bytes(&pixels);

    let Some(compressed) = gzip_compress(&raw_bytes) else {
        error!("Failed to compress canvas snapshot");
        return false;
    };

    info!(
        "Sending canvas snapshot: {}KB raw -> {}KB gzip",
        raw_bytes.len() / 1024,
        compressed.len() / 1024
    );

    sender
        .send(Message::Binary(compressed.into()))
        .await
        .is_ok()
}

fn gzip_compress(data: &[u8]) -> Option<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(data).ok()?;
    encoder.finish().ok()
}

async fn run_send_loop(
    mut sender: WsSender,
    mut subscription: BroadcastSubscription,
    mut error_receiver: mpsc::UnboundedReceiver<String>,
) {
    loop {
        tokio::select! {
            Some(event) = subscription.recv() => {
                let Some(json) = serialize_broadcast_event(&event) else { continue };
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
            Some(error_json) = error_receiver.recv() => {
                if sender.send(Message::Text(error_json.into())).await.is_err() {
                    break;
                }
            }
            else => break,
        }
    }
}

fn serialize_broadcast_event(event: &BroadcastEvent) -> Option<String> {
    let message = match event {
        BroadcastEvent::PixelUpdated(update) => ServerMessage::from(*update),
        BroadcastEvent::SoldierCountChanged(count) => {
            ServerMessage::CurrentSoldiers { count: *count }
        }
    };
    serde_json::to_string(&message)
        .inspect_err(|err| error!("Failed to serialize broadcast event: {err}"))
        .ok()
}

fn handle_client_message(
    state: &AppState,
    error_sender: &mpsc::UnboundedSender<String>,
    connection_id: &str,
    text: &str,
) {
    let Ok(message) = serde_json::from_str::<ClientMessage>(text) else {
        return;
    };

    match message {
        ClientMessage::PlacePixel { x, y, color } => {
            handle_place_pixel(state, error_sender, connection_id, x, y, color);
        }
    }
}

fn handle_place_pixel(
    state: &AppState,
    error_sender: &mpsc::UnboundedSender<String>,
    connection_id: &str,
    x: u32,
    y: u32,
    color: u32,
) {
    let command = place_pixel::Command {
        user_id: connection_id,
        position: Position::new(x, y),
        color: Color::new(color),
    };

    match place_pixel::execute(state, command) {
        Ok(place_pixel::Outcome::Placed(_)) => {}
        Ok(place_pixel::Outcome::CooldownActive) => {
            send_error(error_sender, domain::COOLDOWN_MESSAGE);
        }
        Err(err) => {
            send_error(error_sender, &err.to_string());
        }
    }
}

fn send_error(error_sender: &mpsc::UnboundedSender<String>, message: &str) {
    let error_message = ServerMessage::Error {
        message: message.to_string(),
    };
    match serde_json::to_string(&error_message) {
        Ok(json) => {
            let _ = error_sender.send(json);
        }
        Err(err) => error!("Failed to serialize error message: {err}"),
    }
}
