use std::sync::Arc;

use api_types::{self, PixelUpdatePayload};
use application::AppState;
use application::canvas::place_pixel;
use domain::{BroadcastEvent, BroadcastSubscription, Color, Position, UserId};
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

pub async fn on_connect(socket: SocketRef, state: Arc<AppState>) {
    info!("Socket connected: {:?} {:?}", socket.ns(), socket.id);

    let subscription = state.broadcaster().subscribe();

    send_canvas_state(&socket, &state);
    register_soldier(&state, &socket);
    spawn_broadcast_forwarder(socket.clone(), subscription);
    register_place_pixel_handler(&socket, state.clone());
    register_disconnect_handler(&socket, state);
}

fn send_canvas_state(socket: &SocketRef, state: &AppState) {
    let canvas_pixels = application::canvas::get_state::execute(state);
    let pixel_values = Color::collect_as_u32(&canvas_pixels);
    socket
        .emit(api_types::events::CANVAS_STATE, &pixel_values)
        .ok();
}

fn register_soldier(state: &AppState, socket: &SocketRef) {
    let user_id = UserId::new(socket.id.to_string());
    application::soldiers::connect::execute(state, user_id);
}

fn spawn_broadcast_forwarder(socket: SocketRef, mut subscription: BroadcastSubscription) {
    tokio::spawn(async move {
        while let Some(event) = subscription.recv().await {
            if forward_broadcast_event(&socket, &event).is_err() {
                break;
            }
        }
    });
}

fn forward_broadcast_event(socket: &SocketRef, event: &BroadcastEvent) -> Result<(), ()> {
    match event {
        BroadcastEvent::PixelUpdated(update) => {
            let payload = PixelUpdatePayload::from(*update);
            socket
                .emit(api_types::events::PIXEL_UPDATED, &payload)
                .map_err(|_| ())
        }
        BroadcastEvent::SoldierCountChanged(count) => socket
            .emit(api_types::events::CURRENT_SOLDIERS, count)
            .map_err(|_| ()),
    }
}

fn register_place_pixel_handler(socket: &SocketRef, state: Arc<AppState>) {
    socket.on(
        api_types::events::PLACE_PIXEL,
        move |socket: SocketRef, Data::<PixelUpdatePayload>(payload)| async move {
            handle_place_pixel(&socket, &state, payload);
        },
    );
}

fn handle_place_pixel(socket: &SocketRef, state: &AppState, payload: PixelUpdatePayload) {
    let position = Position::new(payload.x, payload.y);
    let color = Color::new(payload.color);

    info!("Received pixel update: {position} color={}", color.as_u32());

    let user_id = UserId::new(socket.id.to_string());
    let command = place_pixel::Command {
        user_id: &user_id,
        position,
        color,
    };

    match place_pixel::execute(state, command) {
        Ok(place_pixel::Outcome::Placed(_)) => {}
        Ok(place_pixel::Outcome::CooldownActive) => {
            emit_error(socket, domain::COOLDOWN_MESSAGE);
        }
        Err(err) => {
            emit_error(socket, &err.to_string());
        }
    }
}

fn register_disconnect_handler(socket: &SocketRef, state: Arc<AppState>) {
    socket.on_disconnect(move |socket: SocketRef| async move {
        handle_disconnect(&socket, &state);
    });
}

fn handle_disconnect(socket: &SocketRef, state: &AppState) {
    info!("Socket disconnected: {:?}", socket.id);
    let user_id = UserId::new(socket.id.to_string());
    application::soldiers::disconnect::execute(state, &user_id);
}

fn emit_error(socket: &SocketRef, message: &str) {
    let _ = socket.emit(
        api_types::events::ERROR,
        &serde_json::Value::String(message.to_string()),
    );
}
