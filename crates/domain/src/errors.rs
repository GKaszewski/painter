use thiserror::Error;

use crate::Position;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("pixel position {0} is out of bounds")]
    PixelOutOfBounds(Position),

    #[error(
        "invalid canvas dimensions: expected {expected_width}x{expected_height}, got {actual} pixels"
    )]
    InvalidCanvasData {
        expected_width: u32,
        expected_height: u32,
        actual: usize,
    },

    #[error("persistence error: {0}")]
    Persistence(String),
}
