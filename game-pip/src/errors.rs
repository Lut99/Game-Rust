//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2022, 15:36:35
//  Last edited:
//    11 Aug 2022, 15:39:12
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the errors that may occur in the `game-pip` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Defines general errors that Pipelines may run into.
#[derive(Debug)]
pub enum RenderPipelineError {
    /// Failed to create the PipelineLayout
    PipelineLayoutCreateError{ name: &'static str, err: rust_vk::layout::Error },
    /// Failed to create the RenderPass
    RenderPassCreateError{ name: &'static str, err: rust_vk::render_pass::Error },
    /// Failed to create a Vulkan pipeline
    VkPipelineCreateError{ name: &'static str, err: rust_vk::pipeline::Error },
    /// Failed to create a Framebuffer
    FramebufferCreateError{ name: &'static str, err: rust_vk::framebuffer::Error },

    /// Could not allocate a buffer
    BufferCreateError{ name: &'static str, what: &'static str, err: rust_vk::pools::errors::MemoryPoolError },
    /// Could not map the memory of a staging buffer
    BufferMapError{ name: &'static str, what: &'static str, err: rust_vk::pools::errors::MemoryPoolError },
    /// Could not flush a Buffer
    BufferFlushError{ name: &'static str, what: &'static str, err: rust_vk::pools::errors::MemoryPoolError },
    /// Failed to copy from one buffer to another.
    BufferCopyError{ name: &'static str, src: &'static str, dst: &'static str, err: rust_vk::pools::errors::MemoryPoolError },

    /// Could not allocate a new CommandBuffer
    CommandBufferAllocateError{ name: &'static str, err: rust_vk::pools::command::Error },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ name: &'static str, err: rust_vk::pools::command::Error },

    /// Could not create a Fence
    FenceCreateError{ name: &'static str, err: rust_vk::sync::Error },
    /// Could not create a Semaphore
    SemaphoreCreateError{ name: &'static str, err: rust_vk::sync::Error },

    /// We failed to wait for the Device to become idle.
    IdleError{ name: &'static str, err: rust_vk::device::Error },

    /// Failed to poll a Fence
    FencePollError{ name: &'static str, err: rust_vk::sync::Error },
    /// Failed to get the next image of the target
    NextImageError{ name: &'static str, err: game_tgt::Error },
    /// Failed to rebuild Target
    TargetRebuildError{ name: &'static str, err: game_tgt::Error },
    /// Could not submit the command buffer for rendering
    SubmitError{ name: &'static str, err: rust_vk::queue::Error },
    /// Could not present the resulting frame
    PresentError{ name: &'static str, err: game_tgt::Error },

    /// A custom error occurred
    Custom{ name: &'static str, err: Box<dyn Error> },
}

impl Display for RenderPipelineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderPipelineError::*;
        match self {
            PipelineLayoutCreateError{ name, err }  => write!(f, "Failed to create empty PipelineLayout for {} pipeline: {}", name, err),
            RenderPassCreateError{ name, err }      => write!(f, "Failed to create RenderPass for {} pipeline: {}", name, err),
            VkPipelineCreateError{ name, err }      => write!(f, "Failed to create Vulkan Pipeline for {} pipeline: {}", name, err),
            FramebufferCreateError{ name, err }     => write!(f, "Failed to create Framebuffer for {} pipeline: {}", name, err),

            BufferCreateError{ name, what, err }    => write!(f, "Failed to create {} buffer for {} pipeline: {}", what, name, err),
            BufferMapError{ name, what, err }       => write!(f, "Could not map memory for {} buffer for {} pipeline: {}", what, name, err),
            BufferFlushError{ name, what, err }     => write!(f, "Could not flush host memory for {} buffer for {} pipeline: {}", what, name, err),
            BufferCopyError{ name, src, dst, err }  => write!(f, "Could not copy {} buffer to {} buffer for {} pipeline: {}", src, dst, name, err),

            CommandBufferAllocateError{ name, err } => write!(f, "Could not allocate a new CommandBuffer for {} pipeline: {}", name, err),
            CommandBufferRecordError{ name, err }   => write!(f, "Could not record a new CommandBuffer for {} pipeline: {}", name, err),

            FenceCreateError{ name, err }     => write!(f, "Could not create a new Fence for {} pipeline: {}", name, err),
            SemaphoreCreateError{ name, err } => write!(f, "Could not create a new Semaphore for {} pipeline: {}", name, err),

            IdleError{ name, err } => write!(f, "Failed to wait for Device to become idle in {} pipeline: {}", name, err),

            FencePollError{ name, err }     => write!(f, "Failed to poll fence for {} pipeline: {}", name, err),
            TargetRebuildError{ name, err } => write!(f, "Failed to rebuild target for {} pipeline: {}", name, err),
            NextImageError{ name, err }     => write!(f, "Could not get next image from target for {} pipeline: {}", name, err),
            SubmitError{ name, err }        => write!(f, "Could not submit command buffer for {} pipeline: {}", name, err),
            PresentError{ name, err }       => write!(f, "Could not present final frame for {} pipeline: {}", name, err),

            Custom{ err, .. } => write!(f, "{}", err),
        }
    }
}

impl Error for RenderPipelineError {}
