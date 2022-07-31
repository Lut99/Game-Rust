//  COMPONENTS.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:16:27
//  Last edited:
//    31 Jul 2022, 15:53:27
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines ECS components that are specific to the triangle pipeline.
// 

use std::rc::Rc;

use game_ecs::Component;
use game_vk::pools::command::Buffer as CommandBuffer;
use game_vk::pools::memory::VertexBuffer;
use game_vk::layout::PipelineLayout;
use game_vk::pipeline::Pipeline as VkPipeline;
use game_vk::framebuffer::Framebuffer;


/***** LIBRARY *****/
/// The component that stores the additional structures of the TrianglePipeline not exposed in the DrawCallback entity.
pub struct TrianglePipeline {
    /// The layout (in terms of memory management) of the pipeline.
    pub layout        : Rc<PipelineLayout>,
    /// The pipeline that describes how to render to the target.
    pub pipeline      : Rc<VkPipeline>,
    /// The framebuffers which we actually render to.
    pub framebuffers  : Vec<Rc<Framebuffer>>,
    /// The vertex buffer which we'll send to the GPU.
    pub vertex_buffer : Rc<VertexBuffer>,
    /// The command buffer that contains the GPU code to execute.
    pub cmds          : Vec<Rc<CommandBuffer>>,
}

impl Component for TrianglePipeline {}
