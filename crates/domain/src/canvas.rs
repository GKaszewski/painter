use crate::{Color, DomainError, PixelUpdate, Position};

pub struct Canvas {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: vec![Color::white(); (width * height) as usize],
            width,
            height,
        }
    }

    pub fn from_pixels(width: u32, height: u32, pixels: Vec<Color>) -> Result<Self, DomainError> {
        let expected = (width * height) as usize;
        if pixels.len() != expected {
            return Err(DomainError::InvalidCanvasData {
                expected_width: width,
                expected_height: height,
                actual: pixels.len(),
            });
        }
        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn place_pixel(
        &mut self,
        position: Position,
        color: Color,
    ) -> Result<PixelUpdate, DomainError> {
        if position.x() >= self.width || position.y() >= self.height {
            return Err(DomainError::PixelOutOfBounds(position));
        }
        self.pixels[(position.y() * self.width + position.x()) as usize] = color;
        Ok(PixelUpdate::new(position, color))
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
