/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:53:47
 * Last edited:
 *   21 May 2022, 12:31:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors for the game-evt crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that relate to the EventHandler struct.
#[derive(Debug)]
pub enum ThreadedHandlerError {
    /// Failed to spawn a new thread.
    ThreadSpawnError{ err: std::io::Error },

    /// Could not mutex lock some object.
    LockError{ what: &'static str, err: String },
    /// Could not write lock some object.
    WriteLockError{ what: &'static str, err: String },
    /// Could not read lock some object.
    ReadLockError{ what: &'static str, err: String },
}

impl Display for ThreadedHandlerError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ThreadedHandlerError::*;
        match self {
            ThreadSpawnError{ err } => write!(f, "Failed to spawn thread: {}", err),

            LockError{ what, err }      => write!(f, "Could not get a lock on {}: {}", what, err),
            WriteLockError{ what, err } => write!(f, "Could not get a write lock on {}: {}", what, err),
            ReadLockError{ what, err }  => write!(f, "Could not get a read lock on {}: {}", what, err),
        }
    }
}

impl Error for ThreadedHandlerError {}



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
