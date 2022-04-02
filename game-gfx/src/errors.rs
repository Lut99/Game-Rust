/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   02 Apr 2022, 13:14:57
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

    /// The given extra ID is invalid
    InvalidExtraId{ value: usize },
    /// The given subsystem already exists
    DuplicateSubsystem{ type_name: &'static str, extra_id: usize },
    /// Could not initialize a new render system.
    SubsystemCreateError{ type_name: &'static str, err: String },
    
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

            RenderSystemError::InvalidExtraId{ value }                   => write!(f, "Given extra_id value {} (usize::MAX) is reserved; choose another", value),
            RenderSystemError::DuplicateSubsystem{ type_name, extra_id } => {
                // Write that which is common always
                write!(f, "Already registered a subsystem of type '{}'", type_name)?;

                // Switch on the extra_id value
                if *extra_id < usize::MAX {
                    write!(f, " and extra_id {}", extra_id)
                } else {
                    write!(f, " (use the extra_id field to distinguish between subsystems of the same type)")
                }
            },
            RenderSystemError::SubsystemCreateError{ type_name, err }    => write!(f, "Could not create subsystem of type '{}': {}", type_name, err),

            RenderSystemError::GpuAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            RenderSystemError::GpuListError{ err }       => write!(f, "Could not list GPUs: {}", err),
        }
    }
}

impl Error for RenderSystemError {}
