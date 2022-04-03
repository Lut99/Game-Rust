/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 13:16:42
 * Last edited:
 *   03 Apr 2022, 13:22:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the actual pipeline for this crate.
**/

use log::warn;
use winit::event_loop::EventLoop;

use game_gfx::spec::{RenderPipeline, RenderPipelineBuilder};
use game_vk::instance::Instance;

use crate::spec::CreateInfo;


/***** PIPELINE *****/
/// Renders a triangle to the screen as in a hello-triangle case.
pub struct TrianglePipeline {
    
}

impl RenderPipeline for TrianglePipeline {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderTarget to perform a render pass.
    /// 
    /// You can assume that the synchronization with e.g. swapchains is already been done.
    /// 
    /// # Errors
    /// 
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        warn!("TrianglePipeline::render() is not yet implemented.");
        Ok(())
    }
}

impl RenderPipelineBuilder for TrianglePipeline {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo = CreateInfo;


    /// Constructor for the RenderTarget.
    /// 
    /// This initializes a new RenderTarget. Apart from the custom arguments per-target, there is also a large number of arguments given that are owned by the RenderSystem.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn new(event_loop: &EventLoop<()>, instance: &Instance, create_info: Self::CreateInfo) -> Result<Self, Box<dyn std::error::Error>> {
        // Do nothing, so far
        warn!("TrianglePipeline::new() is not yet implemented.");
        Ok(Self {})
    }
}
