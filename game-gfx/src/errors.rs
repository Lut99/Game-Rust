//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 18:08:27
//  Last edited:
//    30 Jul 2022, 20:14:23
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the possible errors that may arise within the `game-gfx`
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Defines errors that originate in the main code of the RenderSystem.
#[derive(Debug)]
pub enum RenderError {
    /// Failed to create a new Vulkan Instance
    InstanceCreateError{ err: game_vk::instance::Error },
    /// Failed to create a new Device
    DeviceCreateError{ err: game_vk::device::Error },
}

impl Display for RenderError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderError::*;
        match self {
            InstanceCreateError{ err } => write!(f, "Could not create a new Instance: {}", err),
            DeviceCreateError{ err }   => write!(f, "Could not create a new Device: {}", err),
        }
    }
}

impl Error for RenderError {}
