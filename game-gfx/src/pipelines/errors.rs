/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:35:56
 * Last edited:
 *   03 Jul 2022, 14:56:26
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
    /// Could not allocate memory for a new buffer
    BufferAllocateError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not allocate a new CommandBuffer
    CommandBufferAllocateError{ err: game_vk::pools::command::Error },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ err: game_vk::pools::command::Error },

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
            BufferAllocateError{ what, err }  => write!(f, "Could not allocate memory for {} buffer: {}", what, err),
            CommandBufferAllocateError{ err } => write!(f, "Could not allocate a new CommandBuffer for the Triangle pipeline: {}", err),
            CommandBufferRecordError{ err }   => write!(f, "Could not record a new CommandBuffer for the Triangle pipeline: {}", err),
            
            SubmitError{ err }     => write!(f, "Could not submit command buffer: {}", err),
        }
    }
}

impl Error for TriangleError {}
