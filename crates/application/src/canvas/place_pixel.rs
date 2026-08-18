use domain::{BroadcastEvent, Color, PixelUpdate, Position, UserId};

use crate::{AppState, ApplicationError};

pub struct Command<'a> {
    pub user_id: &'a UserId,
    pub position: Position,
    pub color: Color,
}

pub enum Outcome {
    Placed(PixelUpdate),
    CooldownActive,
}

pub fn execute(state: &AppState, command: Command<'_>) -> Result<Outcome, ApplicationError> {
    if state.cooldowns().is_on_cooldown(command.user_id) {
        return Ok(Outcome::CooldownActive);
    }
    state
        .canvas()
        .place_pixel(command.position, command.color)?;
    state.cooldowns().record(command.user_id);
    let update = PixelUpdate::new(command.position, command.color);
    state
        .broadcaster()
        .publish(BroadcastEvent::PixelUpdated(update));
    Ok(Outcome::Placed(update))
}
