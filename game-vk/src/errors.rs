/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 14:09:56
 * Last edited:
 *   26 Mar 2022, 18:04:27
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors for the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that occur when setting up an Instance.
#[derive(Debug)]
pub enum InstanceError {
    /// Could not load the Vulkan library at runtime
    LoadError{ err: ash::LoadingError },
    /// Could not create the Instance
    CreateError{ err: ash::vk::Result },
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            InstanceError::LoadError{ err }   => write!(f, "Could not load the Vulkan library: {}", err),
            InstanceError::CreateError{ err } => write!(f, "Could not create Vulkan instance: {}", err),
        }
    }
}

impl Error for InstanceError {}
