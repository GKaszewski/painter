use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::Router;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{Bin, Data, SocketRef},
    SocketIo,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Serialize, Deserialize, Debug)]
struct PixelUpdate {
    x: u32,
    y: u32,
    color: u32,
}

struct AppState {
    canvas: Mutex<Vec<Vec<u32>>>,
    last_update: Mutex<HashMap<String, DateTime<Utc>>>,
}

async fn on_connect(socket: SocketRef, state: Arc<AppState>) {
    info!("Socket connected: {:?} {:?}", socket.ns(), socket.id,);

    let canvas_state = state.canvas.lock().unwrap();
    let serialized_canvas = serde_json::to_string(&*canvas_state).unwrap();
    socket.emit("init-canvas", &serialized_canvas).ok();

    let canvas_state = state.clone();
    socket.on(
        "place-pixel",
        move |socket: SocketRef, Data::<PixelUpdate>(update), Bin(_bin)| {
            let now = Utc::now();
            let mut last_update = canvas_state.last_update.lock().unwrap();

            let socket_id = socket.id.to_string();
            if let Some(&last_time) = last_update.get(&socket_id) {
                if now < last_time + Duration::minutes(1) {
                    let _ = socket.emit(
                        "error",
                        Value::String("You can only place one pixel per minute".to_string()),
                    );
                    return;
                }
            }

            last_update.insert(socket_id.clone(), now);

            info!("Received pixel update: {:?}", update);
            let mut canvas = canvas_state.canvas.lock().unwrap();
            canvas[update.y as usize][update.x as usize] = update.color;

            info!("Emitting pixel update");
            socket.emit("pixel-updated", &update).ok(); // Send to the user who placed the pixel
            socket.broadcast().emit("pixel-updated", &update).ok(); // Send to all other users
        },
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let (layer, io) = SocketIo::new_layer();

    let app_state = Arc::new(AppState {
        canvas: Mutex::new(vec![vec![0xFFFFFFFF; 500]; 500]),
        last_update: Mutex::new(HashMap::new()),
    });

    let app_state_for_ns = app_state.clone();

    io.ns("/", move |socket: SocketRef| {
        tokio::spawn(on_connect(socket, app_state_for_ns.clone()));
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .fallback_service(ServeDir::new("painter-js/dist"))
        .layer(layer)
        .layer(cors);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
