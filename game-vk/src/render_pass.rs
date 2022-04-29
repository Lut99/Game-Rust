/* RENDER PASS.rs
 *   by Lut99
 *
 * Created:
 *   29 Apr 2022, 17:57:08
 * Last edited:
 *   29 Apr 2022, 18:50:43
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines a RenderPass for use in pipelines.
**/

use std::ptr;
use std::sync::Arc;

use ash::vk;

pub use crate::errors::RenderPassError as Error;
use crate::auxillary::{};
use crate::device::Device;


/***** LIBRARY *****/
/// Defines a builder for a render pass.
pub struct RenderPassBuilder {
    /// Collects errors until build() gets called.
    error  : Option<Error>,
}

impl RenderPassBuilder {
    /// Constructor for the RenderPassBuilder.
    /// 
    /// Spawns a new RenderPassBuilder with no attachments or subpasses whatsoever.
    /// 
    /// Use other function calls to add these. When done, call RenderPassBuilder::build() to get the RenderPass. Any errors that occur mid-build will be propagated until that function.
    #[inline]
    pub fn new() -> Self {
        Self {
            error : None,
        }
    }



    /// Defines a new attachment for this RenderPass.
    /// 
    /// # Arguments
    /// - `attachment`: The attachment to, well, attach.
    pub fn attachment(mut self, attachment: ) -> Self {
        
    }
}



/// Defines a render pass, i.e., a single run through a/the pipeline.
pub struct RenderPass {
    /// The device where the RenderPass will live.
    device : Arc<Device>,

    /// The Vulkan RenderPass which we wrap.
    render_pass : vk::RenderPass,
}

impl RenderPass {
    /// Private constructor for the RenderPass.
    /// 
    /// # Arguments
    /// - `device`: The Device where the RenderPass will live.
    /// - `render_pass`: The already contsructed VkRenderPass around which to wrap this struct.
    #[inline]
    fn new(device: Arc<Device>, render_pass: vk::RenderPass) -> Self {
        Self {
            device,
            render_pass,
        }
    }
}
