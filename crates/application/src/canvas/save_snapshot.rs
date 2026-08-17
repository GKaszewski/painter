use crate::{AppState, ApplicationError};

pub fn execute(state: &AppState) -> Result<(), ApplicationError> {
    let Some(persistence) = state.persistence() else {
        return Ok(());
    };
    let pixels = state.canvas().pixels();
    persistence.save(&pixels)?;
    Ok(())
}
