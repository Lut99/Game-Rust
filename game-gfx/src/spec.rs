//  SPEC.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 13:01:17
//  Last edited:
//    06 Aug 2022, 18:32:09
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains interfaces and other structs for the GFX crate.
// 

use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FResult};
use std::rc::Rc;

use rust_vk::sync::{Fence, Semaphore};
use winit::window::WindowId as WinitWindowId;

use game_utl::traits::AsAny;


/***** AUXILLARY NEWTYPES *****/
/// Defines an ID to reference specific windows.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum WindowId {
    /// The main Window to which the RenderSystem renders.
    Main(WinitWindowId),
}

impl Display for WindowId {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowId::*;
        match self {
            Main(_) => write!(f, "WindowId"),
        }
    }
}



/// Defines an ID to reference specific render targets with.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RenderTargetId {
    /// The Window to which the TrianglePipeline renders.
    TriangleWindow,
}

impl Display for RenderTargetId {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderTargetId::*;
        match self {
            TriangleWindow => write!(f, "TriangleWindow"),
        }
    }
}



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
    fn render(&mut self, index: usize, wait_semaphores: &[&Rc<Semaphore>], done_semaphores: &[&Rc<Semaphore>], done_fence: &Rc<Fence>) -> Result<(), Box<dyn Error>>;



    /// Rebuild the RenderPipeline's resources to a new/rebuilt RenderTarget.
    /// 
    /// # Arguments
    /// - `target`: The new RenderTarget who's size and format etc we will rebuild around.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self) -> Result<(), Box<dyn Error>>;
}
