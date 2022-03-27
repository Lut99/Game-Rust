/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   27 Mar 2022, 16:36:08
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
    /// Could not instantiate the Gpu
    GpuCreateError{ err: game_vk::errors::GpuError },
    
    /// Could not auto-select a GPU
    GpuAutoSelectError{ err: game_vk::errors::GpuError },
    /// Could not list the GPUs
    GpuListError{ err: game_vk::errors::GpuError },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            RenderSystemError::InstanceCreateError{ err } => write!(f, "Could not initialize graphics Instance: {}", err),
            RenderSystemError::GpuCreateError{ err }      => write!(f, "Could not initialize GPU: {}", err),
            
            RenderSystemError::GpuAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            RenderSystemError::GpuListError{ err }       => write!(f, "Could not list GPUs: {}", err),
        }
    }
}

impl Error for RenderSystemError {}
