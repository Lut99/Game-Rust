/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 16:56:20
 * Last edited:
 *   05 May 2022, 13:03:07
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements a very simple pipeline that only renders a triangle to the
 *   screen.
**/

use std::error;
use std::rc::Rc;

use log::warn;

use game_vk::auxillary::{AttachmentDescription, AttachmentLoadOp, AttachmentRef, AttachmentStoreOp, BindPoint, CommandBufferFlags, CommandBufferLevel, CommandBufferUsageFlags, CullMode, DrawMode, Extent2D, FrontFace, ImageLayout, Offset2D, RasterizerState, Rect2D, SampleCount, ShaderStage, SubpassDescription, VertexInputState, ViewportState};
use game_vk::device::Device;
use game_vk::shader::Shader;
use game_vk::layout::PipelineLayout;
use game_vk::render_pass::{RenderPass, RenderPassBuilder};
use game_vk::pipeline::{Pipeline as VkPipeline, PipelineBuilder as VkPipelineBuilder};
use game_vk::pools::command::{Buffer as CommandBuffer, Pool as CommandPool};
use game_vk::image;
use game_vk::framebuffer::Framebuffer;

pub use crate::pipelines::errors::TriangleError as Error;
use crate::pipelines::triangle::Shaders;
use crate::spec::{RenderPipeline, RenderPipelineBuilder, RenderTarget};


/***** LIBRARY *****/
/// The Triangle Pipeline, which implements a simple pipeline that only renders a hardcoded triangle to the screen.
pub struct Pipeline {
    /// The Device where the pipeline runs.
    device       : Rc<Device>,
    /// The CommandPool from which we may allocate buffers.
    command_pool : Rc<CommandPool>,

    /// The VkPipeline we wrap
    pipeline        : Rc<VkPipeline>,
    /// The framebuffers for this pipeline
    framebuffers    : Vec<Rc<Framebuffer>>,
    /// The command buffers for this pipeline
    command_buffers : Vec<Rc<CommandBuffer>>,

    
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
    /// - `target`: The RenderTarget where this pipeline will render to.
    /// - `command_pool`: The RenderSystem's CommandPool struct that may be used to allocate command buffers (also later during rendering).
    /// - `create_info`: The CreateInfo struct specific to the backend RenderPipeline, which we use to pass target-specific arguments.
    /// 
    /// # Returns
    /// A new instance of the backend RenderPipeline.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn new(device: Rc<Device>, target: &dyn RenderTarget, command_pool: Rc<CommandPool>, _create_info: Self::CreateInfo) -> Result<Self, Box<dyn error::Error>> {
        // Make the command pool muteable
        let mut command_pool = command_pool;

        // Build the pipeline layout
        let layout = match PipelineLayout::new(device.clone(), &[]) {
            Ok(layout) => layout,
            Err(err)   => { return Err(Box::new(Error::PipelineLayoutCreateError{ err })); }
        };

        // Build the render pass
        let render_pass: Rc<RenderPass> = match RenderPassBuilder::new()
            // Define the colour attachment (no special depth stuff yet)
            .attachment(None, AttachmentDescription {
                format  : target.format(),
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
        let pipeline: Rc<VkPipeline> = match VkPipelineBuilder::new()
            .try_shader(ShaderStage::VERTEX, Shader::try_embedded(device.clone(), Shaders::get("vertex.spv")))
            .try_shader(ShaderStage::FRAGMENT, Shader::try_embedded(device.clone(), Shaders::get("fragment.spv")))
            .vertex_input(VertexInputState {
                attributes : vec![],
                bindings   : vec![],
            })
            .viewport(ViewportState {
                viewport : Rect2D::from_raw( Offset2D::new(0.0, 0.0), Extent2D::new(target.extent().w as f32, target.extent().h as f32) ),
                scissor  : Rect2D::from_raw( Offset2D::new(0, 0), target.extent().clone() ),
                depth    : 0.0..1.0,
            })
            .rasterization(RasterizerState {
                cull_mode  : CullMode::Back,
                front_face : FrontFace::Clockwise,

                line_width : 1.0,
                draw_mode  : DrawMode::Fill,

                discard_result : false,

                depth_clamp : false,
                clamp_value : 0.0,

                depth_bias   : false,
                depth_factor : 0.0,
                depth_slope  : 0.0,
            })
            .build(device.clone(), layout, render_pass.clone())
        {
            Ok(pipeline) => pipeline,
            Err(err)     => { return Err(Box::new(Error::VkPipelineCreateError{ err })); }
        };

        // Create the framebuffers for this target
        let views: &[Rc<image::View>] = target.views();
        let mut framebuffers: Vec<Rc<Framebuffer>> = Vec::with_capacity(views.len());
        for view in views {
            // Add the newly created buffer (if successful)
            framebuffers.push(match Framebuffer::new(device.clone(), render_pass.clone(), vec![ view.clone() ], target.extent().clone()) {
                Ok(framebuffer) => framebuffer,
                Err(err)        => { return Err(Box::new(Error::FramebufferCreateError{ err })); }
            });
        }

        // Record one command buffer per framebuffer
        let mut command_buffers: Vec<Rc<CommandBuffer>> = Vec::with_capacity(framebuffers.len());
        for framebuffer in &framebuffers {
            // Start recording the command buffer
            let cmd: Rc<CommandBuffer> = match Rc::get_mut(&mut command_pool).expect("Could not get muteable command pool").allocate(device.families().graphics, CommandBufferFlags::EMPTY, CommandBufferLevel::Primary) {
                Ok(cmd)  => cmd,
                Err(err) => { return Err(Box::new(Error::CommandBufferAllocateError{ err })); }
            };
            if let Err(err) = cmd.begin(CommandBufferUsageFlags::SIMULTANEOUS_USE) {
                return Err(Box::new(Error::CommandBufferRecordError{ err }));
            };

            // Record the render pass with a single draw
            cmd.begin_render_pass(&render_pass, framebuffer, Rect2D::new(0, 0, target.extent().w, target.extent().h), &[[0.0, 0.0, 0.0, 1.0]]);
            cmd.bind_pipeline(BindPoint::Graphics, &pipeline);
            cmd.draw(3, 1, 0, 0);
            cmd.end_render_pass();

            // Finish recording
            if let Err(err) = cmd.end() {
                return Err(Box::new(Error::CommandBufferRecordError{ err }));
            }

            // Add the buffer
            command_buffers.push(cmd);
        }

        // Done, store the pipeline
        Ok(Self {
            device,
            command_pool,

            pipeline,
            framebuffers,
            command_buffers,
        })
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
        

        // Done
        Ok(())
    }
}
