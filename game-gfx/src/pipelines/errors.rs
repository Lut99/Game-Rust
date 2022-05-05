/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:35:56
 * Last edited:
 *   05 May 2022, 12:57:39
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors for the triangle pipeline.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use game_vk::layout::Error as PipelineLayoutError;
use game_vk::render_pass::Error as RenderPassError;
use game_vk::pipeline::Error as VkPipelineError;


/***** ERRORS *****/
/// Defines the errors for this pipeline.
#[derive(Debug)]
pub enum TriangleError {
    /// Failed to create the PipelineLayout
    PipelineLayoutCreateError{ err: PipelineLayoutError },
    /// Failed to create the RenderPass
    RenderPassCreateError{ err: RenderPassError },
    /// Failed to create a Vulkan pipeline
    VkPipelineCreateError{ err: VkPipelineError },
    /// Failed to create a Framebuffer
    FramebufferCreateError{ err: game_vk::framebuffer::Error },

    /// Could not allocate a new CommandBuffer
    CommandBufferAllocateError{ err: game_vk::pools::command::Error },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ err: game_vk::pools::command::Error },
}

impl Display for TriangleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use TriangleError::*;
        match self {
            PipelineLayoutCreateError{ err } => write!(f, "Failed to create empty PipelineLayout: {}", err),
            RenderPassCreateError{ err }     => write!(f, "Failed to create RenderPass: {}", err),
            VkPipelineCreateError{ err }     => write!(f, "Failed to create Vulkan Pipeline: {}", err),
            FramebufferCreateError{ err }    => write!(f, "Failed to create Framebuffer: {}", err),

            CommandBufferAllocateError{ err } => write!(f, "Could not allocate a new CommandBuffer for the Triangle pipeline: {}", err),
            CommandBufferRecordError{ err }   => write!(f, "Could not record a new CommandBuffer for the Triangle pipeline: {}", err),
        }
    }
}

impl Error for TriangleError {}
