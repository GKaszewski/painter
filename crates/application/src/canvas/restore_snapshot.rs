use crate::{AppState, ApplicationError};

pub fn execute(state: &AppState) -> Result<bool, ApplicationError> {
    let Some(persistence) = state.persistence() else {
        return Ok(false);
    };
    match persistence.load_latest()? {
        Some(pixels) => {
            state.canvas().restore(pixels)?;
            Ok(true)
        }
        None => Ok(false),
    }
}
