//  PIPELINE.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:17:24
//  Last edited:
//    31 Jul 2022, 12:54:31
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the code of the triangle pipeline. This is implemented like
//!   an ECS-system, because it is meant to put its resources in the ECS
//!   as well.
// 

use std::cell::RefCell;
use std::rc::Rc;

use game_ecs::{Ecs, Entity};
use game_vk::auxillary::enums::{AttachmentLoadOp, AttachmentStoreOp, BindPoint, CullMode, DrawMode, FrontFace, ImageFormat, ImageLayout, SampleCount, SharingMode, VertexInputRate};
use game_vk::auxillary::flags::{CommandBufferFlags, CommandBufferUsageFlags, ShaderStage};
use game_vk::auxillary::structs::{AttachmentDescription, AttachmentRef, Extent2D, Offset2D, RasterizerState, Rect2D, SubpassDescription, VertexBinding, VertexInputState, ViewportState};
use game_vk::device::Device;
use game_vk::pools::command::{Buffer as CommandBuffer, Pool as CommandPool};
use game_vk::pools::memory::{HostBuffer, MappedMemory, MemoryPool, StagingBuffer, TransferBuffer, VertexBuffer};
use game_vk::image;
use game_vk::render_pass::{RenderPass, RenderPassBuilder};
use game_vk::layout::PipelineLayout;
use game_vk::shader::Shader;
use game_vk::pipeline::{Pipeline as VkPipeline, PipelineBuilder as VkPipelineBuilder};
use game_vk::framebuffer::Framebuffer;

pub use crate::errors::PipelineError as Error;
use crate::pipelines::triangle::{NAME, Shaders};
use crate::pipelines::triangle::spec::Vertex;


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
        Err(err)        => Err(Error::RenderPassCreateError{ name: NAME, err }),
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
        Err(err)     => Err(Error::VkPipelineCreateError{ name: NAME, err }),
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
            Err(err)        => { return Err(Error::FramebufferCreateError{ name: NAME, err }); }
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
fn create_vertex_buffer(device: &Rc<Device>, memory_pool: &Rc<RefCell<dyn MemoryPool>>, command_pool: &Rc<RefCell<CommandPool>>) -> Result<Rc<VertexBuffer>, Error> {
    // Create the Vertex buffer object
    let vertices: Rc<VertexBuffer> = match VertexBuffer::new(
        device.clone(),
        memory_pool.clone(),
        std::mem::size_of_val(&VERTICES),
        SharingMode::Exclusive,
    ) {
        Ok(vertices) => vertices,
        Err(err)     => { return Err(Error::BufferCreateError{ name: NAME, what: "vertex", err }); }
    };

    // Create the staging buffer
    let staging: Rc<StagingBuffer> = match StagingBuffer::new(
        device.clone(),
        memory_pool.clone(),
        std::mem::size_of_val(&VERTICES),
        SharingMode::Exclusive,
    ) {
        Ok(staging) => staging,
        Err(err)    => { return Err(Error::BufferCreateError{ name: NAME, what: "vertex staging", err }); }
    };

    // Populate the staging buffer
    {
        let mapped: MappedMemory = match staging.map() {
            Ok(mapped) => mapped,
            Err(err)   => { return Err(Error::BufferMapError{ name: NAME, what: "vertex staging", err }); }
        };
        mapped.as_slice_mut::<Vertex>(3).clone_from_slice(&VERTICES);
        if let Err(err) = mapped.flush() { return Err(Error::BufferFlushError{ name: NAME, what: "vertex staging", err }); }
    }

    // Copy the staging to the normal buffer
    let tvertices: Rc<dyn TransferBuffer> = vertices.clone();
    if let Err(err) = staging.copyto(command_pool, &tvertices) { return Err(Error::BufferCopyError{ name: NAME, src: "vertex staging", dst: "vertex", err }); }

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
fn record_command_buffers(device: &Rc<Device>, pool: &Rc<RefCell<CommandPool>>, render_pass: &Rc<RenderPass>, pipeline: &Rc<VkPipeline>, framebuffers: &[Rc<Framebuffer>], vertex_buffer: &Rc<VertexBuffer>, extent: &Extent2D<u32>) -> Result<Vec<Rc<CommandBuffer>>, Error> {
    // Record one command buffer per framebuffer
    let mut command_buffers: Vec<Rc<CommandBuffer>> = Vec::with_capacity(framebuffers.len());
    for framebuffer in framebuffers {
        // Allocate the command buffer
        let cmd: Rc<CommandBuffer> = match CommandBuffer::new(device.clone(), pool.clone(), device.families().graphics, CommandBufferFlags::empty()) {
            Ok(cmd)  => cmd,
            Err(err) => { return Err(Error::CommandBufferAllocateError{ name: NAME, err }); }
        };

        // Start recording the command buffer
        if let Err(err) = cmd.begin(CommandBufferUsageFlags::SIMULTANEOUS_USE) {
            return Err(Error::CommandBufferRecordError{ name: NAME, err });
        };

        // Record the render pass with a single draw
        cmd.begin_render_pass(&render_pass, framebuffer, Rect2D::from_raw(Offset2D::new(0, 0), extent.clone()), &[[0.0, 0.0, 0.0, 1.0]]);
        cmd.bind_pipeline(BindPoint::Graphics, &pipeline);
        cmd.bind_vertex_buffer(0, vertex_buffer);
        cmd.draw(3, 1, 0, 0);
        cmd.end_render_pass();

        // Finish recording
        if let Err(err) = cmd.end() {
            return Err(Error::CommandBufferRecordError{ name: NAME, err });
        }

        // Add the buffer
        command_buffers.push(cmd);
    }

    // Done
    Ok(command_buffers)
}





/***** LIBRARY *****/
/// Creates a new Triangle Pipeline that renders to the given entity.
/// 
/// # Arguments
/// - `target`: The entity to use as a render target. Must implement the RenderTarget component.
/// 
/// # Returns
/// Nothing, but does create new components for the target so that it may function as a renderable pipeline.
/// 
/// # Errors
/// This function errors if we fail to create the appropriate Vulkan structs.
/// 
/// # Panics
/// This function may panic if the given entity does not have a RenderTarget component.
pub fn create(ecs: &Rc<RefCell<Ecs>>, target: Entity) -> Result<(), Error> {
    
}
