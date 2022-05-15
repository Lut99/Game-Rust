/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   05 May 2022, 10:44:39
 * Last edited:
 *   05 May 2022, 12:47:09
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains errors that relate to the pools.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors for CommandPools / CommandBuffers.
#[derive(Debug)]
pub enum CommandPoolError {
    /// Could not create the new VkCommandPool.
    CommandPoolCreateError{ err: ash::vk::Result },

    /// Could not allocate one or more new command buffers.
    CommandBufferAllocateError{ n: u32, err: ash::vk::Result },

    /// Could not reset the command pool(s).
    CommandPoolResetError{ err: ash::vk::Result },

    /// Could not begin a command buffer.
    CommandBufferBeginError{ err: ash::vk::Result },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ err: ash::vk::Result },
}

impl Display for CommandPoolError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use CommandPoolError::*;
        match self {
            CommandPoolCreateError{ err } => write!(f, "Could not create CommandPool: {}", err),
            
            CommandBufferAllocateError{ n, err } => write!(f, "Could not allocate {} CommandBuffer{}: {}", n, if *n == 1 { "" } else { "s" }, err),

            CommandPoolResetError{ err } => write!(f, "Could not reset CommandPool: {}", err),

            CommandBufferBeginError{ err }  => write!(f, "Could not begin CommandBuffer: {}", err),
            CommandBufferRecordError{ err } => write!(f, "Failed to record CommandBuffer: {}", err),
        }
    }
}

impl Error for CommandPoolError {}
