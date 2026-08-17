pub mod canvas;
pub mod errors;
pub mod events;
pub mod ports;
pub mod value_objects;

pub use canvas::Canvas;
pub use errors::DomainError;
pub use events::BroadcastEvent;
pub use ports::BroadcastSubscription;
pub use value_objects::{Color, PixelUpdate, Position};

pub const COOLDOWN_MESSAGE: &str = "You can only place one pixel per minute";
