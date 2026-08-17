use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::{BroadcastEvent, Color, DomainError, Position};

pub trait CanvasStore: Send + Sync {
    fn pixels(&self) -> Arc<[Color]>;
    fn place_pixel(&self, position: Position, color: Color) -> Result<(), DomainError>;
    fn restore(&self, pixels: Vec<Color>) -> Result<(), DomainError>;
}

pub trait CanvasPersistence: Send + Sync {
    fn save(&self, pixels: &[Color]) -> Result<(), DomainError>;
    fn load_latest(&self) -> Result<Option<Vec<Color>>, DomainError>;
}

pub trait EventBroadcaster: Send + Sync {
    fn publish(&self, event: BroadcastEvent);
    fn subscribe(&self) -> BroadcastSubscription;
}

pub trait BroadcastReceiverInner: Send {
    fn recv_boxed(&mut self) -> Pin<Box<dyn Future<Output = Option<BroadcastEvent>> + Send + '_>>;
}

pub struct BroadcastSubscription {
    inner: Box<dyn BroadcastReceiverInner>,
}

impl BroadcastSubscription {
    pub fn new(inner: Box<dyn BroadcastReceiverInner>) -> Self {
        Self { inner }
    }

    pub async fn recv(&mut self) -> Option<BroadcastEvent> {
        self.inner.recv_boxed().await
    }
}
