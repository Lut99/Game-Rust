/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 16:56:20
 * Last edited:
 *   03 Jul 2022, 14:59:03
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements a very simple pipeline that only renders a triangle to the
 *   screen.
**/

use std::error;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use log::debug;

use game_vk::auxillary::{AttachmentDescription, AttachmentLoadOp, AttachmentRef, AttachmentStoreOp, BindPoint, BufferUsageFlags, CommandBufferFlags, CommandBufferLevel, CommandBufferUsageFlags, CullMode, DrawMode, Extent2D, FrontFace, ImageFormat, ImageLayout, MemoryPropertyFlags, Offset2D, RasterizerState, Rect2D, SampleCount, ShaderStage, SharingMode, SubpassDescription, VertexBinding, VertexInputRate, VertexInputState, ViewportState};
use game_vk::device::Device;
use game_vk::shader::Shader;
use game_vk::layout::PipelineLayout;
use game_vk::render_pass::{RenderPass, RenderPassBuilder};
use game_vk::pipeline::{Pipeline as VkPipeline, PipelineBuilder as VkPipelineBuilder};
use game_vk::pools::memory::{Buffer, MemoryPool};
use game_vk::pools::command::{Buffer as CommandBuffer, Pool as CommandPool};
use game_vk::image;
use game_vk::framebuffer::Framebuffer;
use game_vk::sync::{Fence, Semaphore};

pub use crate::pipelines::errors::TriangleError as Error;
use crate::pipelines::triangle::{Shaders, Vertex};
use crate::spec::{RenderPipeline, RenderTarget};


/***** CONSTANTS *****/
/// The raw vertex data we'd like to send to the GPU.
const VERTICES: [Vertex; 3] = [
    Vertex {
        pos    : [0.0, -0.5],
        colour : [1.0, 0.0, 0.0],
    },
    Vertex {
        pos    : [0.5, 0.5],
        colour : [0.0, 1.0, 0.0],
    },
    Vertex {
        pos    : [-0.5, 0.5],
        colour : [0.0, 0.0, 1.0],
    },
];





/***** HELPER FUNCTIONS *****/
/// Creates a new RenderPass for the Pipeline.
/// 
/// # Arguments
/// - `device`: The Device where the RenderPass will be created.
/// - `format`: The format of the new RenderTarget.
fn create_render_pass(device: &Rc<Device>, format: ImageFormat) -> Result<Rc<RenderPass>, Error> {
    // Build the render pass
    match RenderPassBuilder::new()
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
        Ok(render_pass) => Ok(render_pass),
        Err(err)        => Err(Error::RenderPassCreateError{ err }),
    }
}

/// Creates a new VkPipeline for the TrianglePipeline.
/// 
/// # Arguments
/// - `device`: The Device where the new Pipeline will be created.
/// - `layout`: The PipelineLayout to define the Pipeline resource layout.
/// - `render_pass`: The RenderPass that describes the actual rendering part.
/// - `extent`: The Extent2D describing the size of the output frames.
fn create_pipeline(device: &Rc<Device>, layout: &Rc<PipelineLayout>, render_pass: &Rc<RenderPass>, extent: &Extent2D<u32>) -> Result<Rc<VkPipeline>, Error> {
    // Now, prepare the static part of the Pipeline
    match VkPipelineBuilder::new()
        .try_shader(ShaderStage::VERTEX, Shader::try_embedded(device.clone(), Shaders::get("vertex.spv")))
        .try_shader(ShaderStage::FRAGMENT, Shader::try_embedded(device.clone(), Shaders::get("fragment.spv")))
        .vertex_input(VertexInputState {
            attributes : Vertex::vk_attributes(),
            bindings   : vec![
                VertexBinding {
                    binding : 0,
                    stride  : Vertex::vk_size(),
                    rate    : VertexInputRate::Vertex,
                }
            ],
        })
        .viewport(ViewportState {
            viewport : Rect2D::from_raw( Offset2D::new(0.0, 0.0), Extent2D::new(extent.w as f32, extent.h as f32) ),
            scissor  : Rect2D::from_raw( Offset2D::new(0, 0), extent.clone() ),
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
        .build(device.clone(), layout.clone(), render_pass.clone())
    {
        Ok(pipeline) => Ok(pipeline),
        Err(err)     => Err(Error::VkPipelineCreateError{ err }),
    }
}

/// Creates new Framebuffers for the TrianglePipeline.
/// 
/// # Arguments
/// - `device`: The Device where the Framebuffers will live.
/// - `render_pass`: The RenderPass to attach the Framebuffers to.
/// - `views`: The ImageViews to wrap around.
/// - `extent`: The Extent2D that determines the Framebuffer's size.
fn create_framebuffers(device: &Rc<Device>, render_pass: &Rc<RenderPass>, views: &[Rc<image::View>], extent: &Extent2D<u32>) -> Result<Vec<Rc<Framebuffer>>, Error> {
    // Create the framebuffers for this target
    let mut framebuffers: Vec<Rc<Framebuffer>> = Vec::with_capacity(views.len());
    for view in views {
        // Add the newly created buffer (if successful)
        framebuffers.push(match Framebuffer::new(device.clone(), render_pass.clone(), vec![ view.clone() ], extent.clone()) {
            Ok(framebuffer) => framebuffer,
            Err(err)        => { return Err(Error::FramebufferCreateError{ err }); }
        });
    }

    // Done
    Ok(framebuffers)
}

/// Creates, allocates and populates the vertex buffer.
/// 
/// # Arguments
/// - `device`: The Device where the new Buffer will be allocated. Note that the Buffer's memory will be allocated on the device of the given `memory_pool`.
/// - `memory_pool`: The MemoryPool where to allocate the memory for the vertex buffer (and a temporary staging buffer).
/// - `command_pool`: The CommandPool where we will get a command buffer to do the copy on.
fn create_vertex_buffer(device: &Rc<Device>, memory_pool: &Rc<dyn MemoryPool>, command_pool: &Arc<RwLock<CommandPool>>) -> Result<Buffer, Error> {
    // Create the Vertex buffer object
    let vertices: Rc<Buffer> = match Buffer::new(
        device.clone(),
        BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::TRANSFER_DST, SharingMode::Exclusive, MemoryPropertyFlags::DEVICE_LOCAL,
        std::mem::size_of_val(&VERTICES),
    ) {
        Ok(vertices) => vertices,
        Err(err)     => { return Err(Error::BufferCreateError{ what: "vertex", err }); }
    };

    // Create the staging buffer
    let staging: Rc<Buffer> = match Buffer::new(
        device.clone(),
        BufferUsageFlags::TRANSFER_SRC, SharingMode::Exclusive, MemoryPropertyFlags::HOST_VISIBLE,
        std::mem::size_of_val(&VERTICES),
    ) {
        Ok(staging) => staging,
        Err(err)    => { return Err(Error::BufferCreateError{ what: "staging", err }); }
    };

    // Allocate memory for both
    if let Err(err) = vertices.bind(memory_pool.clone()) {
        return Err(Error::BufferAllocateError{ what: "vertex", err });
    }
    if let Err(err) = staging.bind(memory_pool.clone()) {
        return Err(Error::BufferAllocateError{ what: "staging", err });
    }

    // Populate the staging buffer
    

    // Copy the staging to the normal buffer
}

/// Records the commands buffers for the TrianglePipeline.
/// 
/// # Arguments
/// - `device`: The Device where we will get queue families from.
/// - `command_pool`: The Pool to allocate new buffers from.
/// - `render_pass`: The RenderPass that we want to run in this buffer.
/// - `pipeline`: The Pipeline that we want to run in this buffer.
/// - `framebuffers`: The Framebuffers for which to record CommandBuffers.
/// - `extent`: The portion of the Framebuffer to render to.
fn record_command_buffers(device: &Rc<Device>, pool: &Arc<RwLock<CommandPool>>, render_pass: &Rc<RenderPass>, pipeline: &Rc<VkPipeline>, framebuffers: &[Rc<Framebuffer>], extent: &Extent2D<u32>) -> Result<Vec<Rc<CommandBuffer>>, Error> {
    // Record one command buffer per framebuffer
    let mut command_buffers: Vec<Rc<CommandBuffer>> = Vec::with_capacity(framebuffers.len());
    for framebuffer in framebuffers {
        // Start recording the command buffer
        let cmd: Rc<CommandBuffer> = match CommandPool::allocate(pool, device.families().graphics, CommandBufferFlags::EMPTY, CommandBufferLevel::Primary) {
            Ok(cmd)  => cmd,
            Err(err) => { return Err(Error::CommandBufferAllocateError{ err }); }
        };
        if let Err(err) = cmd.begin(CommandBufferUsageFlags::SIMULTANEOUS_USE) {
            return Err(Error::CommandBufferRecordError{ err });
        };

        // Record the render pass with a single draw
        cmd.begin_render_pass(&render_pass, framebuffer, Rect2D::from_raw(Offset2D::new(0, 0), extent.clone()), &[[0.0, 0.0, 0.0, 1.0]]);
        cmd.bind_pipeline(BindPoint::Graphics, &pipeline);
        cmd.draw(3, 1, 0, 0);
        cmd.end_render_pass();

        // Finish recording
        if let Err(err) = cmd.end() {
            return Err(Error::CommandBufferRecordError{ err });
        }

        // Add the buffer
        command_buffers.push(cmd);
    }

    // Done
    Ok(command_buffers)
}





/***** LIBRARY *****/
/// The Triangle Pipeline, which implements a simple pipeline that only renders a hardcoded triangle to the screen.
pub struct Pipeline {
    /// The Device where the pipeline runs.
    device       : Rc<Device>,
    /// The PipelineLayout that defines the resource layout of the pipeline.
    layout       : Rc<PipelineLayout>,
    /// The MemoryPool from which we may draw memory.
    memory_pool  : Rc<dyn MemoryPool>,
    /// The CommandPool from which we may allocate buffers.
    command_pool : Arc<RwLock<CommandPool>>,

    /// The VkPipeline we wrap.
    pipeline        : Rc<VkPipeline>,
    /// The framebuffers for this pipeline.
    framebuffers    : Vec<Rc<Framebuffer>>,
    /// The command buffers for this pipeline.
    command_buffers : Vec<Rc<CommandBuffer>>,
}

impl Pipeline {
    /// Constructor for the RenderPipeline.
    /// 
    /// This initializes a new RenderPipeline. Apart from the custom arguments per-target, there is also a large number of arguments given that are owned by the RenderSystem.
    /// 
    /// # Arguments
    /// - `device`: The Device that may be used to initialize parts of the RenderPipeline.
    /// - `target`: The RenderTarget where this pipeline will render to.
    /// - `command_pool`: The RenderSystem's CommandPool struct that may be used to allocate command buffers (also later during rendering).
    /// 
    /// # Returns
    /// A new instance of the backend RenderPipeline.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    pub fn new(device: Rc<Device>, target: &dyn RenderTarget, memory_pool: Rc<dyn MemoryPool>, command_pool: Arc<RwLock<CommandPool>>) -> Result<Self, Error> {
        // Build the pipeline layout
        let layout = match PipelineLayout::new(device.clone(), &[]) {
            Ok(layout) => layout,
            Err(err)   => { return Err(Error::PipelineLayoutCreateError{ err }); }
        };

        // Build the render pass
        let render_pass: Rc<RenderPass> = create_render_pass(&device, target.format())?;

        // Build the pipeline
        let extent = target.extent();
        let pipeline: Rc<VkPipeline> = create_pipeline(&device, &layout, &render_pass, &extent)?;

        // Create the framebuffers for this target
        let framebuffers: Vec<Rc<Framebuffer>> = create_framebuffers(&device, &render_pass, &target.views(), &extent)?;

        // Prepare the triangle buffer
        let vertex_buffer: Buffer = create_vertex_buffer(&device, &mut memory_pool)?;

        // Record one command buffer per framebuffer
        let command_buffers: Vec<Rc<CommandBuffer>> = record_command_buffers(&device, &command_pool, &render_pass, &pipeline, &framebuffers, &extent)?;

        // Done, store the pipeline
        Ok(Self {
            device,
            layout,
            memory_pool,
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
    /// # Arguments
    /// - `index`: The index of the target image to render to.
    /// - `wait_semaphores`: One or more Semaphores to wait for before we can start rendering.
    /// - `done_semaphores`: One or more Semaphores to signal when we're done rendering.
    /// - `done_fence`: Fence to signal when rendering is done.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self, index: usize, wait_semaphores: &[&Rc<Semaphore>], done_semaphores: &[&Rc<Semaphore>], done_fence: &Rc<Fence>) -> Result<(), Box<dyn error::Error>> {
        // We only need to submit our already-recorded command buffer
        match self.device.queues().present.submit(&self.command_buffers[index], wait_semaphores, done_semaphores, done_fence) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Box::new(Error::SubmitError{ err })),
        }
    }



    /// Rebuild the RenderPipeline's resources to a new/rebuilt RenderTarget.
    /// 
    /// # Arguments
    /// - `target`: The new RenderTarget who's size and format etc we will rebuild around.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self, target: &dyn RenderTarget) -> Result<(), Box<dyn error::Error>> {
        debug!("Rebuiling TrianglePipeline...");

        // Build the render pass
        let render_pass: Rc<RenderPass> = create_render_pass(&self.device, target.format())?;

        // Build the pipeline
        let extent = target.extent();
        let pipeline: Rc<VkPipeline> = create_pipeline(&self.device, &self.layout, &render_pass, &extent)?;

        // Create the framebuffers for this target
        let framebuffers: Vec<Rc<Framebuffer>> = create_framebuffers(&self.device, &render_pass, &target.views(), &extent)?;

        // Record one command buffer per framebuffer
        let command_buffers: Vec<Rc<CommandBuffer>> = record_command_buffers(&self.device, &self.command_pool, &render_pass, &pipeline, &framebuffers, &extent)?;

        // Overwrite some internal shit
        self.pipeline        = pipeline;
        self.framebuffers    = framebuffers;
        self.command_buffers = command_buffers;

        // Done
        Ok(())
    }
}
