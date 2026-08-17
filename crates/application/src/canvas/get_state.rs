use std::sync::Arc;

use domain::Color;

use crate::AppState;

pub fn execute(state: &AppState) -> Arc<[Color]> {
    state.canvas().pixels()
}
