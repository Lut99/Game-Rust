/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 16:56:20
 * Last edited:
 *   30 Apr 2022, 17:37:37
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements a very simple pipeline that only renders a triangle to the
 *   screen.
**/

use std::error;
use std::sync::Arc;

use log::warn;

use game_vk::auxillary::{AttachmentDescription, AttachmentLoadOp, AttachmentRef, AttachmentStoreOp, BindPoint, ImageFormat, ImageLayout, SampleCount, ShaderStage, SubpassDescription};
use game_vk::device::Device;
use game_vk::shader::Shader;
use game_vk::layout::PipelineLayout;
use game_vk::render_pass::{RenderPass, RenderPassBuilder};
use game_vk::pipeline::{Pipeline as VkPipeline, PipelineBuilder as VkPipelineBuilder};

pub use crate::pipelines::errors::TriangleError as Error;
use crate::spec::{RenderPipeline, RenderPipelineBuilder};


/***** LIBRARY *****/
/// The Triangle Pipeline, which implements a simple pipeline that only renders a hardcoded triangle to the screen.
pub struct Pipeline {
    
}

impl RenderPipelineBuilder<'static> for Pipeline {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo = ();


    /// Constructor for the RenderPipeline.
    /// 
    /// This initializes a new RenderPipeline. Apart from the custom arguments per-target, there is also a large number of arguments given that are owned by the RenderSystem.
    /// 
    /// # Arguments
    /// - `device`: The Device that may be used to initialize parts of the RenderPipeline.
    /// - `format`: The ImageFormat of the target frame where this pipeline will render to.
    /// - `create_info`: The CreateInfo struct specific to the backend RenderPipeline, which we use to pass target-specific arguments.
    /// 
    /// # Returns
    /// A new instance of the backend RenderPipeline.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn new(device: Arc<Device>, format: ImageFormat, create_info: Self::CreateInfo) -> Result<Self, Box<dyn error::Error>> {
        // Build the pipeline layout
        let layout = match PipelineLayout::new(device.clone(), &[]) {
            Ok(layout) => layout,
            Err(err)   => { return Err(Box::new(Error::PipelineLayoutCreateError{ err })); }
        };

        // Build the render pass
        let render_pass: Arc<RenderPass> = match RenderPassBuilder::new()
            // Define the colour attachment (no special depth stuff yet)
            .attachment(None, AttachmentDescription {
                format,
                samples : SampleCount::One,

                on_load  : AttachmentLoadOp::Clear,
                on_store : AttachmentStoreOp::Store,

                on_stencil_load  : AttachmentLoadOp::DontCare,
                on_stencil_store : AttachmentStoreOp::DontCare,

                start_layout : ImageLayout::Undefined,
                end_layout   : ImageLayout::Present,
            })
            .subpass(None, SubpassDescription {
                bind_point : BindPoint::Graphics,

                input_attaches    : vec![],
                colour_attaches   : vec![AttachmentRef{ index: 0, layout: ImageLayout::ColourAttachment }],
                resolve_attaches  : vec![],
                preserve_attaches : vec![],

                depth_stencil : None,
            })
            .build(device.clone())
        {
            Ok(render_pass) => render_pass,
            Err(err)        => { return Err(Box::new(Error::RenderPassCreateError{ err })); }
        };

        // Now, prepare the static part of the Pipeline
        let pipeline: Arc<VkPipeline> = match VkPipelineBuilder::new()
            .try_shader(ShaderStage::VERTEX, Shader::from_path())
            .build(device, layout, render_pass)
        {
            Ok(pipeline) => pipeline,
            Err(err)     => { return Err(Box::new(Error::VkPipelineCreateError{ err })); }
        };

        // Done
        Ok(Self {})
    }
}

impl RenderPipeline for Pipeline {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderTarget to perform a render pass.
    /// 
    /// You can assume that the synchronization with e.g. swapchains is already been done.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn error::Error>> {
        warn!("TrianglePipeline::render() is not yet implemented.");
        Ok(())
    }
}
