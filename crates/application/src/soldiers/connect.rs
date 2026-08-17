use domain::BroadcastEvent;

use crate::AppState;

pub fn execute(state: &AppState, user_id: String) -> usize {
    let count = state.soldiers().add(user_id);
    state
        .broadcaster()
        .publish(BroadcastEvent::SoldierCountChanged(count));
    count
}
