use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(u32);

impl Color {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn white() -> Self {
        Self(0xFFFFFFFF)
    }

    pub fn collect_as_u32(colors: &[Color]) -> Vec<u32> {
        colors.iter().map(|color| color.as_u32()).collect()
    }

    pub fn collect_as_bytes(colors: &[Color]) -> Vec<u8> {
        colors
            .iter()
            .flat_map(|color| color.as_u32().to_ne_bytes())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> u32 {
        self.x
    }

    pub fn y(self) -> u32 {
        self.y
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PixelUpdate {
    position: Position,
    color: Color,
}

impl PixelUpdate {
    pub fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }

    pub fn position(self) -> Position {
        self.position
    }

    pub fn color(self) -> Color {
        self.color
    }
}
