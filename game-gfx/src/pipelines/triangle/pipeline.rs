/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 16:56:20
 * Last edited:
 *   28 Jul 2022, 17:42:38
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements a very simple pipeline that only renders a triangle to the
 *   screen.
**/

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use log::debug;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard};

use game_ecs::{Ecs, Entity};
use game_vk::auxillary::enums::{AttachmentLoadOp, AttachmentStoreOp, BindPoint, CullMode, DrawMode, FrontFace, ImageFormat, ImageLayout, SampleCount, SharingMode, VertexInputRate};
use game_vk::auxillary::flags::{CommandBufferFlags, CommandBufferUsageFlags, ShaderStage};
use game_vk::auxillary::structs::{AttachmentDescription, AttachmentRef, Extent2D, Offset2D, RasterizerState, Rect2D, SubpassDescription, VertexBinding, VertexInputState, ViewportState};
use game_vk::device::Device;
use game_vk::shader::Shader;
use game_vk::layout::PipelineLayout;
use game_vk::render_pass::{RenderPass, RenderPassBuilder};
use game_vk::pipeline::{Pipeline as VkPipeline, PipelineBuilder as VkPipelineBuilder};
use game_vk::pools::memory::prelude::*;
use game_vk::pools::memory::{MappedMemory, StagingBuffer, VertexBuffer};
use game_vk::pools::command::{Buffer as CommandBuffer, Pool as CommandPool};
use game_vk::image;
use game_vk::framebuffer::Framebuffer;
use game_vk::sync::{Fence, Semaphore};

pub use crate::errors::PipelineError as Error;
use crate::pipelines::triangle::{Shaders, Vertex};
use crate::spec::RenderPipeline;
use crate::components;
use crate::window;


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
fn create_vertex_buffer(device: &Rc<Device>, memory_pool: &Arc<RwLock<dyn MemoryPool>>, command_pool: &Arc<RwLock<CommandPool>>) -> Result<Rc<VertexBuffer>, Error> {
    // Create the Vertex buffer object
    let vertices: Rc<VertexBuffer> = match VertexBuffer::new(
        device.clone(),
        memory_pool.clone(),
        std::mem::size_of_val(&VERTICES),
        SharingMode::Exclusive,
    ) {
        Ok(vertices) => vertices,
        Err(err)     => { return Err(Error::BufferCreateError{ what: "vertex", err }); }
    };

    // Create the staging buffer
    let staging: Rc<StagingBuffer> = match StagingBuffer::new(
        device.clone(),
        memory_pool.clone(),
        std::mem::size_of_val(&VERTICES),
        SharingMode::Exclusive,
    ) {
        Ok(staging) => staging,
        Err(err)    => { return Err(Error::BufferCreateError{ what: "vertex staging", err }); }
    };

    // Populate the staging buffer
    {
        let mapped: MappedMemory = match staging.map() {
            Ok(mapped) => mapped,
            Err(err)   => { return Err(Error::BufferMapError{ what: "vertex staging", err }); }
        };
        mapped.as_slice_mut::<Vertex>(3).clone_from_slice(&VERTICES);
        if let Err(err) = mapped.flush() { return Err(Error::BufferFlushError{ what: "vertex staging", err }); }
    }

    // Copy the staging to the normal buffer
    let tvertices: Rc<dyn TransferBuffer> = vertices.clone();
    if let Err(err) = staging.copyto(command_pool, &tvertices) { return Err(Error::BufferCopyError{ src: "vertex staging", dst: "vertex", err }); }

    // Done
    Ok(vertices)
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
fn record_command_buffers(device: &Rc<Device>, pool: &Arc<RwLock<CommandPool>>, render_pass: &Rc<RenderPass>, pipeline: &Rc<VkPipeline>, framebuffers: &[Rc<Framebuffer>], vertex_buffer: &Rc<VertexBuffer>, extent: &Extent2D<u32>) -> Result<Vec<Rc<CommandBuffer>>, Error> {
    // Record one command buffer per framebuffer
    let mut command_buffers: Vec<Rc<CommandBuffer>> = Vec::with_capacity(framebuffers.len());
    for framebuffer in framebuffers {
        // Allocate the command buffer
        let cmd: Rc<CommandBuffer> = match CommandBuffer::new(device.clone(), pool.clone(), device.families().graphics, CommandBufferFlags::empty()) {
            Ok(cmd)  => cmd,
            Err(err) => { return Err(Error::CommandBufferAllocateError{ err }); }
        };

        // Start recording the command buffer
        if let Err(err) = cmd.begin(CommandBufferUsageFlags::SIMULTANEOUS_USE) {
            return Err(Error::CommandBufferRecordError{ err });
        };

        // Record the render pass with a single draw
        cmd.begin_render_pass(&render_pass, framebuffer, Rect2D::from_raw(Offset2D::new(0, 0), extent.clone()), &[[0.0, 0.0, 0.0, 1.0]]);
        cmd.bind_pipeline(BindPoint::Graphics, &pipeline);
        cmd.bind_vertex_buffer(0, vertex_buffer);
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
    /// The ECS we use to interface with entities.
    ecs    : Rc<RefCell<Ecs>>,
    /// The ID of the Target for this Pipeline.
    target : Entity,

    /// The Device where the pipeline runs.
    device       : Rc<Device>,
    /// The PipelineLayout that defines the resource layout of the pipeline.
    layout       : Rc<PipelineLayout>,
    /// The MemoryPool from which we may draw memory.
    _memory_pool : Arc<RwLock<dyn MemoryPool>>,
    /// The CommandPool from which we may allocate buffers.
    command_pool : Arc<RwLock<CommandPool>>,

    /// The vertex buffer for this pipeline.
    vertex_buffer   : Rc<VertexBuffer>,

    /// The VkPipeline we wrap.
    pipeline        : Rc<VkPipeline>,
    /// The framebuffers for this pipeline.
    framebuffers    : Vec<Rc<Framebuffer>>,
    /// The command buffers for this pipeline.
    command_buffers : Vec<Rc<CommandBuffer>>,

    /// A list used to determine the image for the current frame in flight. And index of usize::MAX means no image in flight.
    indices     : Vec<usize>,
    /// The list of semaphores which we use to determine if the next image is ready.
    image_ready : Vec<Rc<Semaphore>>,
}

impl Pipeline {
    /// Constructor for the RenderPipeline.
    /// 
    /// This initializes a new RenderPipeline. Apart from the custom arguments per-target, there is also a large number of arguments given that are owned by the RenderSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The Entity Component System where we will read the render target's data from.
    /// - `device`: The Device that may be used to initialize parts of the RenderPipeline.
    /// - `memory_pool`: The RenderSystem's MemoryPool struct that may be used to allocate generic buffers and images (also later during rendering).
    /// - `command_pool`: The RenderSystem's CommandPool struct that may be used to allocate command buffers (also later during rendering).
    /// - `target`: The entity where this pipeline will render to.
    /// - `frames_in_flight`: The number of frames that will (at most) be in flight.
    /// 
    /// # Returns
    /// A new instance of the backend RenderPipeline.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    pub fn new(ecs: Rc<RefCell<Ecs>>, device: Rc<Device>, memory_pool: Arc<RwLock<dyn MemoryPool>>, command_pool: Arc<RwLock<CommandPool>>, target: Entity, frames_in_flight: usize) -> Result<Self, Error> {
        // Create all the necessary structs
        let layout          : Rc<PipelineLayout>;
        let render_pass     : Rc<RenderPass>;
        let pipeline        : Rc<VkPipeline>;
        let vertex_buffer   : Rc<VertexBuffer>;
        let framebuffers    : Vec<Rc<Framebuffer>>;
        let command_buffers : Vec<Rc<CommandBuffer>>;
        {
            // Get the target information
            let ecs: Ref<Ecs> = ecs.borrow();
            let starget: MappedRwLockReadGuard<components::Target> = ecs.get_component(target).unwrap_or_else(|| panic!("Given entity {:?} does not have a Target component", target));

            // Build the pipeline layout
            layout = match PipelineLayout::new(device.clone(), &[]) {
                Ok(layout) => layout,
                Err(err)   => { return Err(Error::PipelineLayoutCreateError{ err }); }
            };

            // Build the render pass & pipeline
            render_pass = create_render_pass(&device, starget.format)?;
            pipeline = create_pipeline(&device, &layout, &render_pass, &starget.extent)?;

            // Prepare the triangle buffer
            vertex_buffer = create_vertex_buffer(&device, &memory_pool, &command_pool)?;

            // Create the framebuffers for this target
            framebuffers = create_framebuffers(&device, &render_pass, &starget.views, &starget.extent)?;

            // Record one command buffer per framebuffer
            command_buffers = record_command_buffers(&device, &command_pool, &render_pass, &pipeline, &framebuffers, &vertex_buffer, &starget.extent)?;
        }

        // Create the semaphores
        let mut image_ready : Vec<Rc<Semaphore>> = Vec::with_capacity(frames_in_flight);
        for _ in 0..frames_in_flight {
            image_ready.push(match Semaphore::new(device.clone()) {
                Ok(semaphore) => semaphore,
                Err(err)      => { return Err(Error::SemaphoreCreateError{ err }); }  
            });
        }

        // Done, store the pipeline
        Ok(Self {
            ecs,
            target,

            device,
            layout,
            _memory_pool : memory_pool,
            command_pool,

            vertex_buffer,

            pipeline,
            framebuffers,
            command_buffers,

            indices : (0..frames_in_flight).map(|_| usize::MAX).collect(),
            image_ready,
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
    /// - `current_frame`: The current frame in flight, since there will likely be multiple.
    /// - `wait_semaphores`: One or more Semaphores to wait for before we can start rendering.
    /// - `done_semaphores`: One or more Semaphores to signal when we're done rendering.
    /// - `done_fence`: Fence to signal when rendering is done.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    /// 
    /// # Panics
    /// This function panics if the internal target component has become invalid.
    fn render(&mut self, current_frame: usize, wait_semaphores: &[&Rc<Semaphore>], done_semaphores: &[&Rc<Semaphore>], done_fence: &Rc<Fence>) -> Result<(), Error> {
        // Copy the wait semaphores in a slightly larger list
        let mut wait_semaphores: Vec<&Rc<Semaphore>> = wait_semaphores.to_owned();

        // Get the next index, which is dependent on the actual Target type of the pipeline's render target
        let mut index: Option<usize> = None;
        if let Some(win) = self.ecs.borrow().get_component::<components::Window>(self.target) {
            // Attempt to get the next image
            match window::next_image(&win, Some(&self.image_ready[current_frame])) {
                Ok(Some(i)) => { index = Some(i); },
                Ok(None)    => { return Err(Error::SwapchainRebuildNeeded); },
                Err(err)    => { return Err(Error::SwapchainNextImageError{ err }); }
            }

            // If so, then add the semaphore to the list
            wait_semaphores.push(&self.image_ready[current_frame]);
        }
        self.indices[current_frame] = index.unwrap_or_else(|| panic!("Target {:?} has neither a Window nor an Image component", self.target));

        // We only need to submit our already-recorded command buffer
        match self.device.queues().present.submit(&self.command_buffers[self.indices[current_frame]], &wait_semaphores, done_semaphores, Some(done_fence)) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::SubmitError{ err }),
        }
    }

    /// Presents the rendered image to the internal target.
    /// 
    /// Note that this doesn't _actually_ present it, but merely schedule it. Thus, this function may be executed before rendering is done.
    /// 
    /// # Arguments
    /// - `current_frame`: The current frame in flight, since there will likely be multiple.
    /// - `wait_semaphores`: A list of semaphores to wait for before we can start presenting the image.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn present(&mut self, current_frame: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<(), crate::errors::PipelineError> {
        // Switch on the type of entity again
        if let Some(win) = self.ecs.borrow().get_component::<components::Window>(self.target) {
            // Run the present function
            return match window::present(&win, self.indices[current_frame], wait_semaphores) {
                Ok(_)    => Ok(()),
                Err(err) => Err(Error::PresentError{ title: win.title.clone(), err }),
            }
        }

        // If we got here, panic.
        panic!("Target {:?} has neither a Window nor an Image component", self.target);
    }



    /// Rebuild the RenderPipeline's resources to the internal target.
    /// 
    /// This is only useful if the target's dimensions have changed (e.g., the window has been resized).
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self) -> Result<(), Error> {
        debug!("Rebuiling TrianglePipeline...");

        // Get the target component
        let ecs: Ref<Ecs> = self.ecs.borrow();
        let mut target: MappedRwLockWriteGuard<components::Target> = ecs.get_component_mut(self.target).unwrap_or_else(|| panic!("Internal entity {:?} does not have a Target component (anymore)", self.target));

        // Refresh the entity size if needed for this type
        if let Some(mut win) = ecs.get_component_mut::<components::Window>(self.target) {
            // Rebuild the window itself
            if let Err(err) = window::rebuild(&mut target, &mut win) { return Err(Error::WindowRebuildError{ title: win.title.clone(), err }); }
        }

        // Build the render pass and pipeline with the new extent
        let render_pass: Rc<RenderPass> = create_render_pass(&self.device, target.format)?;
        let pipeline: Rc<VkPipeline> = create_pipeline(&self.device, &self.layout, &render_pass, &target.extent)?;

        // Create the framebuffers for this target, with the new extent
        let framebuffers: Vec<Rc<Framebuffer>> = create_framebuffers(&self.device, &render_pass, &target.views, &target.extent)?;

        // Record one command buffer per framebuffer (with the new extent)
        let command_buffers: Vec<Rc<CommandBuffer>> = record_command_buffers(&self.device, &self.command_pool, &render_pass, &pipeline, &framebuffers, &self.vertex_buffer, &target.extent)?;

        // Overwrite some internal shit to store this all
        self.pipeline        = pipeline;
        self.framebuffers    = framebuffers;
        self.command_buffers = command_buffers;

        // Done
        Ok(())
    }



    /// Returns the internal Target's Entity ID.
    #[inline]
    fn target(&self) -> Entity { self.target }
}
