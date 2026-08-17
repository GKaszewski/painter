mod common;

use domain::BroadcastEvent;

#[test]
fn connect_increments_count() {
    let (state, _) = common::test_state();
    assert_eq!(
        application::soldiers::connect::execute(&state, "a".into()),
        1
    );
    assert_eq!(
        application::soldiers::connect::execute(&state, "b".into()),
        2
    );
    assert_eq!(
        application::soldiers::connect::execute(&state, "c".into()),
        3
    );
}

#[test]
fn disconnect_decrements_count() {
    let (state, _) = common::test_state();
    application::soldiers::connect::execute(&state, "a".into());
    application::soldiers::connect::execute(&state, "b".into());

    assert_eq!(application::soldiers::disconnect::execute(&state, "a"), 1);
    assert_eq!(application::soldiers::disconnect::execute(&state, "b"), 0);
}

#[test]
fn disconnect_unknown_user_is_harmless() {
    let (state, _) = common::test_state();
    application::soldiers::connect::execute(&state, "a".into());
    assert_eq!(
        application::soldiers::disconnect::execute(&state, "unknown"),
        1
    );
}

#[test]
fn connect_publishes_soldier_count() {
    let (state, spy) = common::test_state();
    application::soldiers::connect::execute(&state, "a".into());

    let events = spy.events();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], BroadcastEvent::SoldierCountChanged(1)));
}

#[test]
fn disconnect_publishes_soldier_count() {
    let (state, spy) = common::test_state();
    application::soldiers::connect::execute(&state, "a".into());
    application::soldiers::disconnect::execute(&state, "a");

    let events = spy.events();
    assert_eq!(events.len(), 2);
    assert!(matches!(events[1], BroadcastEvent::SoldierCountChanged(0)));
}

#[test]
fn disconnect_clears_cooldown() {
    let (state, _) = common::test_state();
    application::soldiers::connect::execute(&state, "user-1".into());

    application::canvas::place_pixel::execute(
        &state,
        application::canvas::place_pixel::Command {
            user_id: "user-1",
            position: domain::Position::new(0, 0),
            color: domain::Color::new(0xFF),
        },
    )
    .unwrap();

    application::soldiers::disconnect::execute(&state, "user-1");

    // Reconnect with same ID — cooldown should be gone
    application::soldiers::connect::execute(&state, "user-1".into());
    let result = application::canvas::place_pixel::execute(
        &state,
        application::canvas::place_pixel::Command {
            user_id: "user-1",
            position: domain::Position::new(1, 1),
            color: domain::Color::new(0xAA),
        },
    )
    .unwrap();

    assert!(matches!(
        result,
        application::canvas::place_pixel::Outcome::Placed(_)
    ));
}
