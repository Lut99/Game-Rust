/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:14:10
 * Last edited:
 *   02 Apr 2022, 14:27:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the errors for this subsystem.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that occur during the initialization of the subsystem.
#[derive(Debug)]
pub enum CreateError {
    /// Could not create the one window
    WindowCreateError{ err: game_win::Error },
}

impl Display for CreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            CreateError::WindowCreateError{ err } => write!(f, "Could not create window: {}", err),
        }
    }
}

impl Error for CreateError {}
