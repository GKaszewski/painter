use domain::{BroadcastEvent, UserId};

use crate::AppState;

pub fn execute(state: &AppState, user_id: &UserId) -> usize {
    state.cooldowns().remove(user_id);
    let count = state.soldiers().remove(user_id);
    state
        .broadcaster()
        .publish(BroadcastEvent::SoldierCountChanged(count));
    count
}
