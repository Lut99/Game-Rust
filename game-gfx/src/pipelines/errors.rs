/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:35:56
 * Last edited:
 *   27 Jul 2022, 13:32:49
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors for the triangle pipeline.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines the errors for this pipeline.
#[derive(Debug)]
pub enum TriangleError {
    /// Failed to create the PipelineLayout
    PipelineLayoutCreateError{ err: game_vk::layout::Error },
    /// Failed to create the RenderPass
    RenderPassCreateError{ err: game_vk::render_pass::Error },
    /// Failed to create a Vulkan pipeline
    VkPipelineCreateError{ err: game_vk::pipeline::Error },
    /// Failed to create a Framebuffer
    FramebufferCreateError{ err: game_vk::framebuffer::Error },
    /// Could not allocate a buffer
    BufferCreateError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not map the memory of a staging buffer
    BufferMapError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not flush a Buffer
    BufferFlushError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Failed to copy from one buffer to another.
    BufferCopyError{ src: &'static str, dst: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not allocate a new CommandBuffer
    CommandBufferAllocateError{ err: game_vk::pools::command::Error },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ err: game_vk::pools::command::Error },

    /// The swapchain of the pipeline's target needs to be rebuilt.
    SwapchainRebuildNeeded,
    /// Could not get the next swapchain image.
    SwapchainNextImageError{ err: game_vk::swapchain::Error },

    /// COuld not submit the command buffer for rendering
    SubmitError{ err: game_vk::queue::Error },
}

impl Display for TriangleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use TriangleError::*;
        match self {
            PipelineLayoutCreateError{ err }  => write!(f, "Failed to create empty PipelineLayout: {}", err),
            RenderPassCreateError{ err }      => write!(f, "Failed to create RenderPass: {}", err),
            VkPipelineCreateError{ err }      => write!(f, "Failed to create Vulkan Pipeline: {}", err),
            FramebufferCreateError{ err }     => write!(f, "Failed to create Framebuffer: {}", err),
            BufferCreateError{ what, err }    => write!(f, "Failed to create {} buffer: {}", what, err),
            BufferMapError{ what, err }       => write!(f, "Could not map memory for {} buffer: {}", what, err),
            BufferFlushError{ what, err }     => write!(f, "Could not flush host memory for {} buffer: {}", what, err),
            BufferCopyError{ src, dst, err }  => write!(f, "Could not copy {} buffer to {} buffer: {}", src, dst, err),
            CommandBufferAllocateError{ err } => write!(f, "Could not allocate a new CommandBuffer for the Triangle pipeline: {}", err),
            CommandBufferRecordError{ err }   => write!(f, "Could not record a new CommandBuffer for the Triangle pipeline: {}", err),
            
            SwapchainRebuildNeeded         => write!(f, "The pipeline's target's swapchain needs to be rebuilt"),
            SwapchainNextImageError{ err } => write!(f, "Could not get next swapchain image of pipeline's target: {}", err),

            SubmitError{ err } => write!(f, "Could not submit command buffer: {}", err),
        }
    }
}

impl Error for TriangleError {}
