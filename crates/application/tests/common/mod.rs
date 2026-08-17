use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use application::{AppState, InMemoryCanvasStore};
use domain::ports::{
    BroadcastReceiverInner, BroadcastSubscription, CanvasPersistence, EventBroadcaster,
};
use domain::{BroadcastEvent, Color, DomainError};

#[derive(Clone)]
pub struct SpyBroadcaster {
    events: Arc<Mutex<Vec<BroadcastEvent>>>,
}

impl SpyBroadcaster {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn events(&self) -> Vec<BroadcastEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn event_count(&self) -> usize {
        self.events.lock().unwrap().len()
    }
}

impl EventBroadcaster for SpyBroadcaster {
    fn publish(&self, event: BroadcastEvent) {
        self.events.lock().unwrap().push(event);
    }

    fn subscribe(&self) -> BroadcastSubscription {
        BroadcastSubscription::new(Box::new(NoopReceiver))
    }
}

struct NoopReceiver;

impl BroadcastReceiverInner for NoopReceiver {
    fn recv_boxed(&mut self) -> Pin<Box<dyn Future<Output = Option<BroadcastEvent>> + Send + '_>> {
        Box::pin(async { None })
    }
}

pub struct FakePersistence {
    saved: Arc<Mutex<Vec<Vec<Color>>>>,
    to_load: Mutex<Option<Vec<Color>>>,
}

impl FakePersistence {
    pub fn empty() -> Self {
        Self {
            saved: Arc::new(Mutex::new(Vec::new())),
            to_load: Mutex::new(None),
        }
    }

    pub fn with_snapshot(pixels: Vec<Color>) -> Self {
        Self {
            saved: Arc::new(Mutex::new(Vec::new())),
            to_load: Mutex::new(Some(pixels)),
        }
    }

    pub fn saved_ref(&self) -> Arc<Mutex<Vec<Vec<Color>>>> {
        self.saved.clone()
    }
}

impl CanvasPersistence for FakePersistence {
    fn save(&self, pixels: &[Color]) -> Result<(), DomainError> {
        self.saved.lock().unwrap().push(pixels.to_vec());
        Ok(())
    }

    fn load_latest(&self) -> Result<Option<Vec<Color>>, DomainError> {
        Ok(self.to_load.lock().unwrap().clone())
    }
}

pub fn test_state() -> (Arc<AppState>, SpyBroadcaster) {
    test_state_sized(10, 10)
}

pub fn test_state_sized(width: u32, height: u32) -> (Arc<AppState>, SpyBroadcaster) {
    let spy = SpyBroadcaster::new();
    let state = AppState::new(
        Box::new(InMemoryCanvasStore::new(width, height)),
        Box::new(spy.clone()),
        Duration::from_secs(10),
    );
    (Arc::new(state), spy)
}

pub fn test_state_no_cooldown() -> (Arc<AppState>, SpyBroadcaster) {
    let spy = SpyBroadcaster::new();
    let state = AppState::new(
        Box::new(InMemoryCanvasStore::new(10, 10)),
        Box::new(spy.clone()),
        Duration::ZERO,
    );
    (Arc::new(state), spy)
}
