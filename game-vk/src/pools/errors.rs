/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   05 May 2022, 10:44:39
 * Last edited:
 *   04 Jun 2022, 15:44:38
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains errors that relate to the pools.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use crate::auxillary::{DeviceMemoryTypeFlags, MemoryAllocatorKind, MemoryPropertyFlags};


/***** ERRORS *****/
/// Defines errors for MemoryPools / Buffers.
#[derive(Debug)]
pub enum MemoryPoolError {
    /// Could not allocate a new continious block of memory due to some kind of out-of-memory error.
    OutOfMemoryError{ kind: MemoryAllocatorKind, size: usize, free: usize, fragmented: bool },

    /// Failed to create a new VkBuffer object.
    BufferCreateError{ err: ash::vk::Result },
    /// Could not find a memory type with all of the supported requirements and properties.
    UnsupportedMemoryRequirements{ name: String, types: DeviceMemoryTypeFlags, props: MemoryPropertyFlags },
}

impl Display for MemoryPoolError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use MemoryPoolError::*;
        match self {
            OutOfMemoryError{ kind, size, free, fragmented } => write!(f, "Could not allocate new block of {} bytes on a {} allocator: largest free block is only {} bytes (caused by fragmentation: {})", size, kind, free, if *fragmented { "yes" } else { "no" }),

            BufferCreateError{ err }                            => write!(f, "Could not create Buffer: {}", err),
            UnsupportedMemoryRequirements{ name, types, props } => write!(f, "Device '{}' has no memory type that supports memory requirements '{:#b}' and memory properties {}", name, u32::from(*types), props),
        }
    }
}

impl Error for MemoryPoolError {}



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
