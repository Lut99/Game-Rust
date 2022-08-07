//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:30:11
//  Last edited:
//    07 Aug 2022, 18:41:28
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the errors for the EventSystem.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use winit::window::WindowId;


/***** LIBRARY *****/
/// Errors that relate to the EventSystem as a whole.
#[derive(Debug)]
pub enum EventError {
    /// Failed to initiate the render process to a given window.
    RenderError{ id: WindowId, err: game_gfx::Error },

    /// Failed to wait for the Device to become idle while quitting.
    IdleError{ err: game_gfx::Error },
}

impl Display for EventError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use EventError::*;
        match self {
            RenderError{ id, err } => write!(f, "Failed to render to window with id '{:?}': {}", id, err),

            IdleError{ err } => write!(f, "Failed to wait for Device to become idle while quitting the Game: {}", err),
        }
    }
}

impl Error for EventError {}
