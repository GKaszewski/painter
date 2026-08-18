mod common;

use application::canvas::place_pixel::{Command, Outcome};
use domain::{BroadcastEvent, Color, Position, UserId};

fn uid(id: &str) -> UserId {
    UserId::new(id.to_string())
}

macro_rules! place {
    ($state:expr, $user:expr, $x:expr, $y:expr, $color:expr) => {
        application::canvas::place_pixel::execute(
            &$state,
            Command {
                user_id: &uid($user),
                position: Position::new($x, $y),
                color: Color::new($color),
            },
        )
    };
}

#[test]
fn successful_placement_returns_update() {
    let (state, _) = common::test_state();
    let result = place!(state, "user-1", 3, 4, 0xFF0000).unwrap();

    let Outcome::Placed(update) = result else {
        panic!("expected Placed outcome");
    };
    assert_eq!(update.position(), Position::new(3, 4));
    assert_eq!(update.color(), Color::new(0xFF0000));
}

#[test]
fn placement_updates_canvas() {
    let (state, _) = common::test_state();
    place!(state, "user-1", 5, 5, 0xAA).unwrap();

    let pixels = application::canvas::get_state::execute(&state);
    let idx = 5 * 10 + 5;
    assert_eq!(pixels[idx], Color::new(0xAA));
}

#[test]
fn placement_publishes_broadcast_event() {
    let (state, spy) = common::test_state();
    place!(state, "user-1", 0, 0, 0xFF).unwrap();

    let events = spy.events();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], BroadcastEvent::PixelUpdated(_)));
}

#[test]
fn cooldown_blocks_rapid_placement() {
    let (state, _) = common::test_state();
    place!(state, "user-1", 0, 0, 0xFF).unwrap();

    let result = place!(state, "user-1", 1, 1, 0xAA).unwrap();
    assert!(matches!(result, Outcome::CooldownActive));
}

#[test]
fn cooldown_is_per_user() {
    let (state, _) = common::test_state();
    place!(state, "user-1", 0, 0, 0xFF).unwrap();

    let result = place!(state, "user-2", 1, 1, 0xAA).unwrap();
    assert!(matches!(result, Outcome::Placed(_)));
}

#[test]
fn zero_cooldown_allows_rapid_placement() {
    let (state, _) = common::test_state_no_cooldown();
    place!(state, "user-1", 0, 0, 0xFF).unwrap();

    let result = place!(state, "user-1", 1, 1, 0xAA).unwrap();
    assert!(matches!(result, Outcome::Placed(_)));
}

#[test]
fn out_of_bounds_returns_error() {
    let (state, _) = common::test_state();
    assert!(place!(state, "user-1", 99, 99, 0xFF).is_err());
}

#[test]
fn failed_placement_does_not_trigger_cooldown() {
    let (state, _) = common::test_state();
    let _ = place!(state, "user-1", 99, 99, 0xFF);

    let result = place!(state, "user-1", 0, 0, 0xFF).unwrap();
    assert!(matches!(result, Outcome::Placed(_)));
}
