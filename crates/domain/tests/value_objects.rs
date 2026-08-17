use domain::{Color, PixelUpdate, Position};

#[test]
fn color_roundtrips_through_u32() {
    for value in [0, 0xFF, 0xFF0000, 0xFFFFFFFF, 0xDEADBEEF] {
        assert_eq!(Color::new(value).as_u32(), value);
    }
}

#[test]
fn color_white_is_full_alpha() {
    assert_eq!(Color::white().as_u32(), 0xFFFFFFFF);
}

#[test]
fn color_equality() {
    assert_eq!(Color::new(42), Color::new(42));
    assert_ne!(Color::new(1), Color::new(2));
}

#[test]
fn collect_as_u32_preserves_values() {
    let colors = [Color::new(1), Color::new(2), Color::new(3)];
    assert_eq!(Color::collect_as_u32(&colors), vec![1, 2, 3]);
}

#[test]
fn collect_as_bytes_length() {
    let colors = vec![Color::white(); 10];
    assert_eq!(Color::collect_as_bytes(&colors).len(), 40);
}

#[test]
fn collect_as_bytes_roundtrips() {
    let original = vec![Color::new(0x01020304), Color::new(0xAABBCCDD)];
    let bytes = Color::collect_as_bytes(&original);
    let restored: Vec<Color> = bytes
        .chunks_exact(4)
        .map(|c| Color::new(u32::from_ne_bytes([c[0], c[1], c[2], c[3]])))
        .collect();
    assert_eq!(original, restored);
}

#[test]
fn position_accessors() {
    let pos = Position::new(42, 17);
    assert_eq!(pos.x(), 42);
    assert_eq!(pos.y(), 17);
}

#[test]
fn position_display() {
    assert_eq!(Position::new(3, 7).to_string(), "(3, 7)");
}

#[test]
fn position_equality() {
    assert_eq!(Position::new(1, 2), Position::new(1, 2));
    assert_ne!(Position::new(1, 2), Position::new(2, 1));
}

#[test]
fn pixel_update_carries_position_and_color() {
    let pos = Position::new(5, 10);
    let color = Color::new(0xFF00FF);
    let update = PixelUpdate::new(pos, color);
    assert_eq!(update.position(), pos);
    assert_eq!(update.color(), color);
}
