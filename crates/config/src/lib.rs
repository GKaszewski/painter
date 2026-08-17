use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid value for '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    #[error("failed to load config: {0}")]
    LoadFailed(String),
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub canvas: CanvasConfig,
    pub cooldown: CooldownConfig,
    pub rate_limit: RateLimitConfig,
    pub broadcast: BroadcastConfig,
    pub snapshot: SnapshotConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub enable_cors: bool,
}

#[derive(Debug, Clone)]
pub struct CanvasConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct CooldownConfig {
    pub placement_secs: u64,
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub burst_size: u32,
    pub per_second: u64,
}

#[derive(Debug, Clone)]
pub struct BroadcastConfig {
    pub channel_capacity: usize,
}

#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    pub enabled: bool,
    pub interval_secs: u64,
    pub max_snapshots: usize,
    pub directory: String,
}

pub trait ConfigSource {
    fn load(&self) -> Result<AppConfig, ConfigError>;
}
