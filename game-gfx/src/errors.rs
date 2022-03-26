/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   26 Mar 2022, 18:13:40
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors for the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines the errors that happen at the base system itself.
#[derive(Debug)]
pub enum RenderSystemError {
    /// Could not instantiate the Vulkan instance
    InstanceCreateError{ err: game_vk::errors::InstanceError },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            RenderSystemError::InstanceCreateError{ err } => write!(f, "Could not initialize graphics Instance: {}", err),
        }
    }
}

impl Error for RenderSystemError {}
