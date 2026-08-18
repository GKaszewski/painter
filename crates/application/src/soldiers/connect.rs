use domain::{BroadcastEvent, UserId};

use crate::AppState;

pub fn execute(state: &AppState, user_id: UserId) -> usize {
    let count = state.soldiers().add(user_id);
    state
        .broadcaster()
        .publish(BroadcastEvent::SoldierCountChanged(count));
    count
}
