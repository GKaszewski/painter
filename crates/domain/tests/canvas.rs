use domain::{Canvas, Color, DomainError, Position};

macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position::new($x, $y)
    };
}

macro_rules! color {
    ($v:expr) => {
        Color::new($v)
    };
}

macro_rules! assert_pixel {
    ($canvas:expr, $x:expr, $y:expr, $expected:expr) => {{
        let (x, y): (u32, u32) = ($x, $y);
        let idx = y as usize * $canvas.width() as usize + x as usize;
        let expected = color!($expected);
        assert_eq!($canvas.pixels()[idx], expected, "pixel at ({x}, {y})");
    }};
}

fn small_canvas() -> Canvas {
    Canvas::new(10, 10)
}

#[test]
fn new_canvas_is_all_white() {
    let canvas = small_canvas();
    assert_eq!(canvas.pixels().len(), 100);
    assert!(canvas.pixels().iter().all(|&c| c == Color::white()));
}

#[test]
fn dimensions_match_construction() {
    let canvas = Canvas::new(42, 17);
    assert_eq!(canvas.width(), 42);
    assert_eq!(canvas.height(), 17);
    assert_eq!(canvas.pixels().len(), 42 * 17);
}

#[test]
fn place_pixel_updates_correct_position() {
    let mut canvas = small_canvas();
    let update = canvas.place_pixel(pos!(3, 4), color!(0xFF0000)).unwrap();

    assert_pixel!(canvas, 3, 4, 0xFF0000);
    assert_eq!(update.position(), pos!(3, 4));
    assert_eq!(update.color(), color!(0xFF0000));
}

#[test]
fn place_pixel_does_not_affect_neighbors() {
    let mut canvas = small_canvas();
    canvas.place_pixel(pos!(5, 5), color!(0xFF)).unwrap();

    for (x, y) in [(4, 5), (6, 5), (5, 4), (5, 6)] {
        assert_pixel!(canvas, x, y, 0xFFFFFFFF);
    }
}

#[test]
fn place_pixel_overwrites_previous() {
    let mut canvas = small_canvas();
    canvas.place_pixel(pos!(0, 0), color!(0xAA)).unwrap();
    canvas.place_pixel(pos!(0, 0), color!(0xBB)).unwrap();
    assert_pixel!(canvas, 0, 0, 0xBB);
}

#[test]
fn place_pixel_at_boundary() {
    let mut canvas = small_canvas();
    assert!(canvas.place_pixel(pos!(9, 9), color!(0xFF)).is_ok());
    assert!(canvas.place_pixel(pos!(0, 0), color!(0xFF)).is_ok());
    assert!(canvas.place_pixel(pos!(9, 0), color!(0xFF)).is_ok());
    assert!(canvas.place_pixel(pos!(0, 9), color!(0xFF)).is_ok());
}

#[test]
fn place_pixel_out_of_bounds() {
    let mut canvas = small_canvas();

    for (x, y) in [(10, 0), (0, 10), (10, 10), (100, 100)] {
        let result = canvas.place_pixel(pos!(x, y), color!(0xFF));
        assert!(
            matches!(result, Err(DomainError::PixelOutOfBounds(_))),
            "({x}, {y}) should be out of bounds"
        );
    }
}

#[test]
fn from_pixels_with_correct_size() {
    let pixels = vec![color!(0xAA); 25];
    let canvas = Canvas::from_pixels(5, 5, pixels).unwrap();
    assert_eq!(canvas.width(), 5);
    assert_eq!(canvas.height(), 5);
    assert!(canvas.pixels().iter().all(|&c| c == color!(0xAA)));
}

#[test]
fn from_pixels_with_wrong_size() {
    let too_few = vec![color!(0); 10];
    let too_many = vec![color!(0); 30];

    for pixels in [too_few, too_many] {
        assert!(
            matches!(
                Canvas::from_pixels(5, 5, pixels),
                Err(DomainError::InvalidCanvasData { .. })
            ),
            "should reject pixel vec that doesn't match dimensions"
        );
    }
}

#[test]
fn from_pixels_preserves_content() {
    let mut pixels = vec![Color::white(); 9];
    pixels[4] = color!(0xFF0000); // center pixel of 3x3
    let canvas = Canvas::from_pixels(3, 3, pixels).unwrap();
    assert_pixel!(canvas, 1, 1, 0xFF0000);
    assert_pixel!(canvas, 0, 0, 0xFFFFFFFF);
}
