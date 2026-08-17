use std::sync::Arc;

use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get};
use config::RateLimitConfig;
use rust_embed::Embed;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{Any, CorsLayer};

const RATE_LIMIT_CLEANUP_INTERVAL_SECS: u64 = 1;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../../../painter-js/dist/"]
struct StaticAssets;

async fn serve_static(path: axum::extract::Path<String>) -> Response {
    let path = path.0;
    serve_embedded_file(&path)
}

async fn serve_index() -> Response {
    serve_embedded_file("index.html")
}

fn serve_embedded_file(path: &str) -> Response {
    match StaticAssets::get(path) {
        Some(file) => {
            let content_type = file.metadata.mimetype();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type.to_string())],
                file.data,
            )
                .into_response()
        }
        None => serve_embedded_file("index.html"),
    }
}

pub fn build_router(
    enable_cors: bool,
    rate_limit_config: &RateLimitConfig,
) -> Result<Router, String> {
    let rate_governor = Arc::new(
        GovernorConfigBuilder::default()
            .burst_size(rate_limit_config.burst_size)
            .per_second(rate_limit_config.per_second)
            .finish()
            .ok_or("Invalid rate limit configuration")?,
    );

    let governor = rate_governor.limiter().clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(
                RATE_LIMIT_CLEANUP_INTERVAL_SECS,
            ))
            .await;
            governor.retain_recent();
        }
    });

    let router = Router::new()
        .route("/check/", get(|| async { "OK" }))
        .route("/{*path}", get(serve_static))
        .fallback(get(serve_index))
        .layer(GovernorLayer::new(rate_governor));

    Ok(if enable_cors {
        router.layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
    } else {
        router
    })
}
