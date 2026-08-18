use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use domain::ports::{
    BroadcastReceiverInner, BroadcastSubscription, CanvasPersistence, CanvasStore, EventBroadcaster,
};
use domain::{BroadcastEvent, Canvas, Color, DomainError, Position, UserId};
use tokio::sync::broadcast;
use tracing::warn;

const INITIAL_CONNECTION_CAPACITY: usize = 128;

pub struct AppState {
    canvas: Box<dyn CanvasStore>,
    broadcaster: Box<dyn EventBroadcaster>,
    persistence: Option<Box<dyn CanvasPersistence>>,
    cooldowns: CooldownTracker,
    soldiers: SoldierTracker,
}

impl AppState {
    pub fn new(
        canvas: Box<dyn CanvasStore>,
        broadcaster: Box<dyn EventBroadcaster>,
        cooldown: Duration,
    ) -> Self {
        Self {
            canvas,
            broadcaster,
            persistence: None,
            cooldowns: CooldownTracker::new(cooldown),
            soldiers: SoldierTracker::new(),
        }
    }

    pub fn with_persistence(mut self, persistence: Box<dyn CanvasPersistence>) -> Self {
        self.persistence = Some(persistence);
        self
    }

    pub fn canvas(&self) -> &dyn CanvasStore {
        &*self.canvas
    }

    pub fn broadcaster(&self) -> &dyn EventBroadcaster {
        &*self.broadcaster
    }

    pub fn persistence(&self) -> Option<&dyn CanvasPersistence> {
        self.persistence.as_deref()
    }

    pub fn cooldowns(&self) -> &CooldownTracker {
        &self.cooldowns
    }

    pub fn soldiers(&self) -> &SoldierTracker {
        &self.soldiers
    }
}

pub struct InProcessBroadcaster {
    sender: broadcast::Sender<BroadcastEvent>,
}

impl InProcessBroadcaster {
    pub fn new(sender: broadcast::Sender<BroadcastEvent>) -> Self {
        Self { sender }
    }
}

impl EventBroadcaster for InProcessBroadcaster {
    fn publish(&self, event: BroadcastEvent) {
        let _ = self.sender.send(event);
    }

    fn subscribe(&self) -> BroadcastSubscription {
        BroadcastSubscription::new(Box::new(TokioBroadcastReceiver(self.sender.subscribe())))
    }
}

struct TokioBroadcastReceiver(broadcast::Receiver<BroadcastEvent>);

impl BroadcastReceiverInner for TokioBroadcastReceiver {
    fn recv_boxed(&mut self) -> Pin<Box<dyn Future<Output = Option<BroadcastEvent>> + Send + '_>> {
        Box::pin(async { self.0.recv().await.ok() })
    }
}

fn acquire_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| {
        warn!("Recovered from poisoned mutex");
        poisoned.into_inner()
    })
}

struct CanvasState {
    canvas: Canvas,
    snapshot: Option<Arc<[Color]>>,
}

pub struct InMemoryCanvasStore {
    state: Mutex<CanvasState>,
}

impl InMemoryCanvasStore {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            state: Mutex::new(CanvasState {
                canvas: Canvas::new(width, height),
                snapshot: None,
            }),
        }
    }
}

impl CanvasStore for InMemoryCanvasStore {
    fn pixels(&self) -> Arc<[Color]> {
        let mut state = acquire_lock(&self.state);
        if let Some(ref cached) = state.snapshot {
            return cached.clone();
        }
        let new_snapshot: Arc<[Color]> = Arc::from(state.canvas.pixels());
        state.snapshot = Some(new_snapshot.clone());
        new_snapshot
    }

    fn place_pixel(&self, position: Position, color: Color) -> Result<(), DomainError> {
        let mut state = acquire_lock(&self.state);
        state.canvas.place_pixel(position, color)?;
        state.snapshot = None;
        Ok(())
    }

    fn restore(&self, pixels: Vec<Color>) -> Result<(), DomainError> {
        let mut state = acquire_lock(&self.state);
        let new_canvas = Canvas::from_pixels(state.canvas.width(), state.canvas.height(), pixels)?;
        state.canvas = new_canvas;
        state.snapshot = None;
        Ok(())
    }
}

pub struct CooldownTracker {
    entries: Mutex<HashMap<UserId, Instant>>,
    cooldown: Duration,
}

impl CooldownTracker {
    pub fn new(cooldown: Duration) -> Self {
        Self {
            entries: Mutex::new(HashMap::with_capacity(INITIAL_CONNECTION_CAPACITY)),
            cooldown,
        }
    }

    pub fn is_on_cooldown(&self, user_id: &UserId) -> bool {
        let entries = acquire_lock(&self.entries);
        entries
            .get(user_id)
            .map(|last| last.elapsed() < self.cooldown)
            .unwrap_or(false)
    }

    pub fn record(&self, user_id: &UserId) {
        acquire_lock(&self.entries).insert(user_id.clone(), Instant::now());
    }

    pub fn remove(&self, user_id: &UserId) {
        acquire_lock(&self.entries).remove(user_id);
    }
}

pub struct SoldierTracker {
    connected: Mutex<HashSet<UserId>>,
}

impl SoldierTracker {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(HashSet::with_capacity(INITIAL_CONNECTION_CAPACITY)),
        }
    }

    pub fn add(&self, user_id: UserId) -> usize {
        let mut connected = acquire_lock(&self.connected);
        connected.insert(user_id);
        connected.len()
    }

    pub fn remove(&self, user_id: &UserId) -> usize {
        let mut connected = acquire_lock(&self.connected);
        connected.remove(user_id);
        connected.len()
    }
}
