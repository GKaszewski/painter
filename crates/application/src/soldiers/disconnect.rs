use domain::BroadcastEvent;

use crate::AppState;

pub fn execute(state: &AppState, user_id: &str) -> usize {
    state.cooldowns().remove(user_id);
    let count = state.soldiers().remove(user_id);
    state
        .broadcaster()
        .publish(BroadcastEvent::SoldierCountChanged(count));
    count
}
