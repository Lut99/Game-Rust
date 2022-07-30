/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   28 Jul 2022, 17:42:19
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains interfaces and other structs for the GFX crate.
**/

use std::fmt::{Display, Debug, Formatter, Result as FResult};
use std::rc::Rc;

use game_ecs::Entity;
use game_utl::traits::AsAny;
use game_vk::sync::{Fence, Semaphore};


/***** AUXILLARY NEWTYPES *****/
/// Defines an ID to reference specific render pipelines with.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RenderPipelineId {
    /// The Triangle pipeline, which just draws a hardcoded triangle.
    Triangle,
}

impl Display for RenderPipelineId {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderPipelineId::*;
        match self {
            Triangle => write!(f, "Triangle"),
        }
    }
}





/***** RENDER PIPELINE TRAIT *****/
/// Defines a Render-capable pipeline.
pub trait RenderPipeline: 'static + AsAny {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function doesn't perform the actual rendering, but rather schedules it.
    /// 
    /// # Arguments
    /// - `current_frame`: The current frame in flight, since there will likely be multiple.
    /// - `wait_semaphores`: One or more Semaphores to wait for before we can start rendering.
    /// - `done_semaphores`: One or more Semaphores to signal when we're done rendering.
    /// - `done_fence`: Fence to signal when rendering is done.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self, current_frame: usize, wait_semaphores: &[&Rc<Semaphore>], done_semaphores: &[&Rc<Semaphore>], done_fence: &Rc<Fence>) -> Result<(), crate::errors::PipelineError>;

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
    fn present(&mut self, current_frame: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<(), crate::errors::PipelineError>;



    /// Rebuild the RenderPipeline's resources to a new/rebuilt RenderTarget.
    /// 
    /// This is only useful if the target's dimensions have changed (e.g., the window has been resized).
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self) -> Result<(), crate::errors::PipelineError>;



    /// Returns the internal Target's Entity ID.
    fn target(&self) -> Entity;
}
