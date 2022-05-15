/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:53:47
 * Last edited:
 *   15 May 2022, 15:44:59
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors for the game-evt crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that relate to the EventLoop and its operation.
#[derive(Debug)]
pub enum EventSystemError {
    /// Failed to spawn a new thread
    ThreadSpawnError{ err: std::io::Error },
}

impl Display for EventSystemError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use EventSystemError::*;
        match self {
            ThreadSpawnError{ err } => write!(f, "Failed to spawn thread: {}", err),
        }
    }
}

impl Error for EventSystemError {}
