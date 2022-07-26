/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jul 2022, 23:21:16
 * Last edited:
 *   26 Jul 2022, 15:38:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the ECS components used by the RenderSystem.
**/

use std::rc::Rc;

use game_ecs::spec::Component;
use game_vk::device::Device;
use game_vk::render_pass::RenderPass;
use game_vk::framebuffer::Framebuffer;

use crate::spec::RenderPipelineId;


/***** LIBRARY *****/
/// Defines a Pipeline, which is the common part of every RenderSystem Pipeline object. This defines the programmable part of each render pipeline.
pub struct Pipeline {
    /// The Device where the Pipeline lives.
    device : Rc<Device>,

    /// The RenderPass for this Pipeline.
    render_pass : Rc<RenderPass>,
}

impl Component for Pipeline {}



/// Defines a Renderable, which defines a single pipeline/target mapping. This represents something that may be rendered.
pub struct Renderable {
    /// The pipeline to render with.
    pipeline : RenderPipelineId,
    /// The framebuffers to render to.
    views    : Vec<Rc<Framebuffer>>,
}

impl Component for Renderable {}
