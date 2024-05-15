use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{routing::get, Extension, Json, Router};
use chrono::{DateTime, Duration, Utc};
use memory_stats::memory_stats;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{Bin, Data, SocketRef},
    SocketIo,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

type Canvas = Arc<Mutex<Vec<u32>>>;
type LastUpdate = Arc<Mutex<HashMap<String, DateTime<Utc>>>>;
type Soldiers = Arc<Mutex<HashSet<String>>>;

#[derive(Serialize, Deserialize, Debug)]
struct PixelUpdate {
    x: u32,
    y: u32,
    color: u32,
}

async fn on_connect(
    socket: SocketRef,
    canvas: Canvas,
    last_update: LastUpdate,
    soldiers: Soldiers,
) {
    info!("Socket connected: {:?} {:?}", socket.ns(), socket.id,);

    let socket_id = socket.id.to_string();

    {
        let mut soldiers = soldiers.lock().unwrap();
        soldiers.insert(socket_id.clone());
        let soldiers_num = soldiers.len();

        socket.emit("current_soldiers", &soldiers_num).ok();
        socket
            .broadcast()
            .emit("current_soldiers", &soldiers_num)
            .ok();
    }

    info!("Memory usage after connection and before cloning state: ");
    print_memory_usage();

    let canvas_clone = canvas.clone();
    let last_update_clone = last_update.clone();

    info!("Memory usage after connection and after cloning state: ");
    print_memory_usage();

    socket.on(
        "place-pixel",
        move |socket: SocketRef, Data::<PixelUpdate>(update), Bin(_bin)| {
            let socket_id = socket.id.to_string();
            let last_update = last_update_clone.clone();
            let canvas = canvas_clone.clone();

            {
                let now = Utc::now();
                let mut last_update = last_update.lock().unwrap();

                if let Some(&last_time) = last_update.get(&socket_id) {
                    if now < last_time + Duration::seconds(10) {
                        let _ = socket.emit(
                            "error",
                            Value::String("You can only place one pixel per minute".to_string()),
                        );
                        return;
                    }
                }

                last_update.insert(socket_id.clone(), now);
            }

            info!("Received pixel update: {:?}", update);
            let mut canvas = canvas.lock().unwrap();
            canvas[update.y as usize * 500 + update.x as usize] = update.color;

            info!("Emitting pixel update");
            socket.emit("pixel-updated", &update).ok(); // Send to the user who placed the pixel
            socket.broadcast().emit("pixel-updated", &update).ok(); // Send to all other users

            info!("Memory usage after pixel update: ");
            print_memory_usage();
        },
    );

    socket.on_disconnect(move |socket: SocketRef| {
        info!("Socket disconnected: {:?}", socket.id);
        info!("Memory usage after disconnection: ");
        print_memory_usage();
        let socket_id = socket.id.to_string();

        {
            let mut soldiers = soldiers.lock().unwrap();
            soldiers.remove(&socket_id.to_string());
            info!("Soldiers: {:?}", soldiers.len());

            let soldiers_num = soldiers.len();
            socket
                .broadcast()
                .emit("current_soldiers", &soldiers_num)
                .ok();
        }

        {
            let mut last_update = last_update.lock().unwrap();
            last_update.remove(&socket_id);

            info!("Last update: {:?}", last_update.len());
        }

        info!("Memory usage after disconnection and cleanup: ");
        print_memory_usage();
    });
}

fn print_memory_usage() {
    if let Some(usage) = memory_stats() {
        info!(
            "Current physical memory usage: {} MB",
            usage.physical_mem as f64 / 1024.0 / 1024.0
        );
        info!(
            "Current virtual memory usage: {} MB",
            usage.virtual_mem as f64 / 1024.0 / 1024.0
        );
    }
}

async fn get_canvas_state(Extension(canvas): Extension<Canvas>) -> Json<Vec<u32>> {
    let canvas = canvas.lock().unwrap();
    Json(canvas.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;

    let rate_governor = Arc::new(
        GovernorConfigBuilder::default()
            .burst_size(10)
            .per_second(10)
            .finish()
            .unwrap(),
    );

    let governor = rate_governor.limiter().clone();
    let interval = std::time::Duration::from_secs(1);

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            governor.retain_recent();
        }
    });

    let (layer, io) = SocketIo::new_layer();

    let canvas = Arc::new(Mutex::new(vec![0xFFFFFFFF; 500 * 500]));
    let last_update = Arc::new(Mutex::new(HashMap::new()));
    let soldiers = Arc::new(Mutex::new(HashSet::new()));

    let used_memory = std::mem::size_of_val(&*canvas)
        + std::mem::size_of_val(&*last_update)
        + std::mem::size_of_val(&*soldiers);
    info!(
        "Used memory of state: {} bytes, {} KB, {} MB",
        used_memory,
        used_memory / 1024,
        used_memory / 1024 / 1024
    );

    info!("Memory usage after state setup and before socket.io setup: ");
    print_memory_usage();

    let canvas_for_socket = canvas.clone();

    io.ns("/", move |socket: SocketRef| {
        tokio::spawn(on_connect(
            socket,
            canvas_for_socket.clone(),
            last_update.clone(),
            soldiers.clone(),
        ));
    });

    info!("Memory usage after socket.io setup: ");
    print_memory_usage();

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
        .route("/canvas/", get(get_canvas_state))
        .route("/check/", get(|| async { "OK" }))
        .fallback_service(ServeDir::new("dist/"))
        .layer(GovernorLayer {
            config: rate_governor,
        })
        .layer(layer)
        .layer(Extension(canvas.clone()));

    let app = if let Some(cors) = cors {
        app.layer(cors)
    } else {
        app
    };

    let server_address = format!("{}:{}", address, port);
    info!("Starting server on {}", server_address);
    let listener = tokio::net::TcpListener::bind(server_address).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
