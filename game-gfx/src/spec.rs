/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   05 May 2022, 21:42:15
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains interfaces and other structs for the GFX crate.
**/

use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FResult};
use std::rc::Rc;

use game_utl::traits::AsAny;
use game_vk::auxillary::{Extent2D, ImageFormat};
use game_vk::image;
use game_vk::sync::{Fence, Semaphore};


/***** AUXILLARY NEWTYPES *****/
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





/***** RENDER TARGET TRAIT *****/
/// Defines a target that the RenderSystem may render to (like a Window or an Image).
pub trait RenderTarget: 'static + AsAny {
    /// Returns a list of all image views in the RenderTarget.
    fn views(&self) -> &Vec<Rc<image::View>>;

    /// Returns the index of a renderable target, i.e., an image::View to render to.
    /// 
    /// For non-Swapchain targets, this function will be very simple.
    /// 
    /// # Arguments
    /// - `done_semaphore`: Optional Semaphore that should be signalled when the image is available.
    /// 
    /// # Returns
    /// A new ImageView on success.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<usize, Box<dyn Error>>;

    /// Presents this RenderTarget in the way it likes.
    /// 
    /// # Arguments
    /// - `index`: The index of the internal image to present.
    /// - `wait_semaphores`: Zero or more Semaphores that we should wait for before we can present the image.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn present(&self, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<(), Box<dyn Error>>;



    /// Returns the ImageFormat of this RenderTarget.
    fn format(&self) -> ImageFormat;

    /// Returns the extent of this RenderTarget.
    fn extent(&self) -> &Extent2D<u32>;
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
}
