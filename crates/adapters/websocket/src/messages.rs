use domain::PixelUpdate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "place-pixel")]
    PlacePixel { x: u32, y: u32, color: u32 },
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "pixel-updated")]
    PixelUpdated { x: u32, y: u32, color: u32 },
    #[serde(rename = "current_soldiers")]
    CurrentSoldiers { count: usize },
    #[serde(rename = "error")]
    Error { message: String },
}

impl From<PixelUpdate> for ServerMessage {
    fn from(update: PixelUpdate) -> Self {
        Self::PixelUpdated {
            x: update.position().x(),
            y: update.position().y(),
            color: update.color().as_u32(),
        }
    }
}
