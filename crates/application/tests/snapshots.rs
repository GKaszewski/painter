mod common;

use std::sync::Arc;

use application::AppState;
use domain::{Color, UserId};

fn state_with_persistence(persistence: common::FakePersistence) -> Arc<AppState> {
    let spy = common::SpyBroadcaster::new();
    let state = AppState::new(
        Box::new(application::InMemoryCanvasStore::new(10, 10)),
        Box::new(spy),
        std::time::Duration::from_secs(10),
    )
    .with_persistence(Box::new(persistence));
    Arc::new(state)
}

#[test]
fn save_snapshot_persists_current_canvas() {
    let persistence = common::FakePersistence::empty();
    let saved_ref = persistence.saved_ref();
    let state = state_with_persistence(persistence);

    let user = UserId::new("user".to_string());
    application::canvas::place_pixel::execute(
        &state,
        application::canvas::place_pixel::Command {
            user_id: &user,
            position: domain::Position::new(0, 0),
            color: Color::new(0xFF),
        },
    )
    .unwrap();

    application::canvas::save_snapshot::execute(&state).unwrap();

    let snapshots = saved_ref.lock().unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0][0], Color::new(0xFF));
    assert_eq!(snapshots[0].len(), 100);
}

#[test]
fn restore_snapshot_loads_canvas() {
    let mut snapshot = vec![Color::white(); 100];
    snapshot[0] = Color::new(0xDEAD);
    snapshot[99] = Color::new(0xBEEF);

    let state = state_with_persistence(common::FakePersistence::with_snapshot(snapshot));

    let restored = application::canvas::restore_snapshot::execute(&state).unwrap();
    assert!(restored);

    let pixels = application::canvas::get_state::execute(&state);
    assert_eq!(pixels[0], Color::new(0xDEAD));
    assert_eq!(pixels[99], Color::new(0xBEEF));
}

#[test]
fn restore_returns_false_when_no_snapshot() {
    let state = state_with_persistence(common::FakePersistence::empty());
    assert!(!application::canvas::restore_snapshot::execute(&state).unwrap());
}

#[test]
fn save_without_persistence_is_noop() {
    let (state, _) = common::test_state();
    assert!(application::canvas::save_snapshot::execute(&state).is_ok());
}

#[test]
fn restore_without_persistence_returns_false() {
    let (state, _) = common::test_state();
    assert!(!application::canvas::restore_snapshot::execute(&state).unwrap());
}
