//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 13:01:25
//  Last edited:
//    11 Aug 2022, 15:49:57
//  Auto updated?
//    Yes
// 
//  Description:
//!   Collects all errors for the crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines the errors that happen at the base system itself.
#[derive(Debug)]
pub enum RenderSystemError {
    /// Could not instantiate the Vulkan instance
    InstanceCreateError{ err: rust_vk::errors::InstanceError },
    /// Could not instantiate the Gpu
    DeviceCreateError{ err: rust_vk::errors::DeviceError },
    /// Could not create the CommandPool
    CommandPoolCreateError{ err: rust_vk::pools::errors::CommandPoolError },
    /// Could not create a new window
    WindowCreateError{ err: game_tgt::Error },
    /// Could not initialize a new render pipeline.
    RenderPipelineCreateError{ name: &'static str, err: game_pip::Error },
    /// Failed to create a Semaphore
    SemaphoreCreateError{ err: rust_vk::sync::Error },
    /// Failed to create a Fence
    FenceCreateError{ err: rust_vk::sync::Error },

    /// Could not render one of the Pipelines
    RenderError{ name: &'static str, err: game_pip::Error },

    /// Could not wait for the Device to become idle
    IdleError{ err: rust_vk::device::Error },

    /// Could not auto-select a GPU
    DeviceAutoSelectError{ err: rust_vk::errors::DeviceError },
    /// Could not list the GPUs
    DeviceListError{ err: rust_vk::errors::DeviceError },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderSystemError::*;
        match self {
            InstanceCreateError{ err }             => write!(f, "Could not initialize graphics Instance: {}", err),
            DeviceCreateError{ err }               => write!(f, "Could not initialize Device: {}", err),
            CommandPoolCreateError{ err }          => write!(f, "Could not initialize CommandPool: {}", err),
            WindowCreateError{ err }               => write!(f, "Could not initialize Window: {}", err),
            RenderPipelineCreateError{ name, err } => write!(f, "Could not initialize render pipeline '{}': {}", name, err),
            SemaphoreCreateError{ err }            => write!(f, "Failed to create Semaphore: {}", err),
            FenceCreateError{ err }                => write!(f, "Failed to create Fence: {}", err),

            RenderError{ name, err } => write!(f, "Could not render to pipeline '{}': {}", name, err),

            IdleError{ err } => write!(f, "{}", err),

            DeviceAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            DeviceListError{ err }       => write!(f, "Could not list GPUs: {}", err),
        }
    }
}

impl Error for RenderSystemError {}
