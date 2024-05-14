use std::{
    collections::{HashMap, HashSet},
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
    soldiers: Mutex<HashSet<String>>,
}

async fn on_connect(socket: SocketRef, state: Arc<AppState>) {
    info!("Socket connected: {:?} {:?}", socket.ns(), socket.id,);

    let canvas_state = state.canvas.lock().unwrap();
    let serialized_canvas = serde_json::to_string(&*canvas_state).unwrap();
    socket.emit("init-canvas", &serialized_canvas).ok();
    let socket_id = socket.id.to_string();

    let mut soldiers = state.soldiers.lock().unwrap();
    soldiers.insert(socket_id.clone());
    let soldiers_num = soldiers.len();
    socket.emit("current_soldiers", &soldiers_num).ok();
    socket
        .broadcast()
        .emit("current_soldiers", &soldiers_num)
        .ok();

    let app_state = state.clone();
    let app_state_for_disconnect = state.clone();

    socket.on(
        "place-pixel",
        move |socket: SocketRef, Data::<PixelUpdate>(update), Bin(_bin)| {
            let now = Utc::now();
            let mut last_update = app_state.last_update.lock().unwrap();

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
            let mut canvas = app_state.canvas.lock().unwrap();
            canvas[update.y as usize][update.x as usize] = update.color;

            info!("Emitting pixel update");
            socket.emit("pixel-updated", &update).ok(); // Send to the user who placed the pixel
            socket.broadcast().emit("pixel-updated", &update).ok(); // Send to all other users
        },
    );

    socket.on_disconnect(move |socket: SocketRef| {
        info!("Socket disconnected: {:?}", socket.id);
        let mut soldiers = app_state_for_disconnect.soldiers.lock().unwrap();
        soldiers.remove(&socket.id.to_string());
        let soldiers_num = soldiers.len();
        socket
            .broadcast()
            .emit("current_soldiers", &soldiers_num)
            .ok();
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let (layer, io) = SocketIo::new_layer();

    let app_state = Arc::new(AppState {
        canvas: Mutex::new(vec![vec![0xFFFFFFFF; 500]; 500]),
        last_update: Mutex::new(HashMap::new()),
        soldiers: Mutex::new(HashSet::new()),
    });

    let app_state_for_ns = app_state.clone();

    io.ns("/", move |socket: SocketRef| {
        tokio::spawn(on_connect(socket, app_state_for_ns.clone()));
    });

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Invalid port");
    let enable_cors = std::env::var("ENABLE_CORS").unwrap_or_else(|_| "true".to_string()) == "true";

    let cors = if enable_cors {
        Some(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
    } else {
        None
    };

    let app = Router::new()
        .fallback_service(ServeDir::new("dist"))
        .layer(layer);

    let app = if let Some(cors) = cors {
        app.layer(cors)
    } else {
        app
    };

    let server_address = format!("{}:{}", address, port);
    info!("Starting server on {}", server_address);
    let listener = tokio::net::TcpListener::bind(server_address).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
