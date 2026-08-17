use domain::PixelUpdate;
use serde::{Deserialize, Serialize};

pub mod events {
    pub const CANVAS_STATE: &str = "canvas_state";
    pub const PIXEL_UPDATED: &str = "pixel-updated";
    pub const PLACE_PIXEL: &str = "place-pixel";
    pub const CURRENT_SOLDIERS: &str = "current_soldiers";
    pub const ERROR: &str = "error";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelUpdatePayload {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

impl From<PixelUpdate> for PixelUpdatePayload {
    fn from(update: PixelUpdate) -> Self {
        Self {
            x: update.position().x(),
            y: update.position().y(),
            color: update.color().as_u32(),
        }
    }
}
