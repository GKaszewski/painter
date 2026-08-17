use config::{
    AppConfig, BroadcastConfig, CanvasConfig, ConfigError, ConfigSource, CooldownConfig,
    RateLimitConfig, ServerConfig, SnapshotConfig,
};

const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_CANVAS_WIDTH: u32 = 500;
const DEFAULT_CANVAS_HEIGHT: u32 = 500;
const DEFAULT_COOLDOWN_SECS: u64 = 10;
const DEFAULT_RATE_LIMIT_BURST: u32 = 10;
const DEFAULT_RATE_LIMIT_PER_SECOND: u64 = 10;
const DEFAULT_BROADCAST_CAPACITY: usize = 1024;
const DEFAULT_SNAPSHOT_INTERVAL_SECS: u64 = 300;
const DEFAULT_SNAPSHOT_MAX: usize = 5;
const DEFAULT_SNAPSHOT_DIR: &str = "snapshots/";

pub struct EnvConfigSource;

impl ConfigSource for EnvConfigSource {
    fn load(&self) -> Result<AppConfig, ConfigError> {
        Ok(AppConfig {
            server: ServerConfig {
                address: env_or("ADDRESS", DEFAULT_ADDRESS),
                port: parse_env("PORT", DEFAULT_PORT)?,
                enable_cors: parse_bool_env("ENABLE_CORS", true),
            },
            canvas: CanvasConfig {
                width: parse_env("CANVAS_WIDTH", DEFAULT_CANVAS_WIDTH)?,
                height: parse_env("CANVAS_HEIGHT", DEFAULT_CANVAS_HEIGHT)?,
            },
            cooldown: CooldownConfig {
                placement_secs: parse_env("COOLDOWN_SECS", DEFAULT_COOLDOWN_SECS)?,
            },
            rate_limit: RateLimitConfig {
                burst_size: parse_env("RATE_LIMIT_BURST", DEFAULT_RATE_LIMIT_BURST)?,
                per_second: parse_env("RATE_LIMIT_PER_SECOND", DEFAULT_RATE_LIMIT_PER_SECOND)?,
            },
            broadcast: BroadcastConfig {
                channel_capacity: parse_env("BROADCAST_CAPACITY", DEFAULT_BROADCAST_CAPACITY)?,
            },
            snapshot: SnapshotConfig {
                enabled: parse_bool_env("SNAPSHOT_ENABLED", true),
                interval_secs: parse_env("SNAPSHOT_INTERVAL_SECS", DEFAULT_SNAPSHOT_INTERVAL_SECS)?,
                max_snapshots: parse_env("SNAPSHOT_MAX", DEFAULT_SNAPSHOT_MAX)?,
                directory: env_or("SNAPSHOT_DIR", DEFAULT_SNAPSHOT_DIR),
            },
        })
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_bool_env(key: &str, default: bool) -> bool {
    std::env::var(key)
        .map(|value| value == "true")
        .unwrap_or(default)
}

fn parse_env<T: std::str::FromStr>(key: &str, default: T) -> Result<T, ConfigError> {
    match std::env::var(key) {
        Ok(value) => value.parse().map_err(|_| ConfigError::InvalidValue {
            field: key.to_string(),
            reason: format!("'{value}' is not a valid {}", std::any::type_name::<T>()),
        }),
        Err(_) => Ok(default),
    }
}
