/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   05 May 2022, 12:07:07
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
use game_vk::device::Device;
use game_vk::pools::command::Pool as CommandPool;
use game_vk::image;


/***** AUXILLARY NEWTYPES *****/
/// Defines an ID to reference specific render targets with.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct RenderTargetId(usize);

impl RenderTargetId {
    /// Spawns the ID with a '0' value
    #[inline]
    pub(crate) fn new() -> Self { Self(0) }

    /// Increments the ID in case it's a counter to generate new ones.
    #[inline]
    pub(crate) fn increment(&mut self) -> Self { self.0 += 1; Self(self.0) }
}

impl Display for RenderTargetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.0)
    }
}



/// Defines an ID to reference specific render pipelines with.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct RenderPipelineId(usize);

impl RenderPipelineId {
    /// Spawns the ID with a '0' value
    #[inline]
    pub(crate) fn new() -> Self { Self(0) }

    /// Increments the ID in case it's a counter to generate new ones.
    #[inline]
    pub(crate) fn increment(&mut self) -> Self { self.0 += 1; Self(self.0) }
}

impl Display for RenderPipelineId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.0)
    }
}





/***** RENDER TARGET TRAIT *****/
/// Defines a target that the RenderSystem may render to (like a Window or an Image).
pub trait RenderTarget: 'static + AsAny {
    /// Returns a renderable target, i.e., an image::View to render to.
    /// 
    /// # Returns
    /// A new ImageView on success.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn get_view(&mut self) -> Result<Rc<image::View>, Box<dyn Error>>;



    /// Returns a list of all image views in the RenderTarget.
    fn views(&self) -> &Vec<Rc<image::View>>;

    /// Returns the ImageFormat of this RenderTarget.
    fn format(&self) -> ImageFormat;

    /// Returns the extent of this RenderTarget.
    fn extent(&self) -> &Extent2D<u32>;
}



/// Defines the interface to build a new RenderTarget.
pub trait RenderTargetBuilder<'a>: RenderTarget {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo: 'a + Sized + Debug + Clone;


    /// Constructor for the RenderTarget.
    /// 
    /// This initializes a new RenderTarget. Apart from the custom arguments per-target, there is also a large number of arguments given that are owned by the RenderSystem.
    /// 
    /// # Arguments
    /// - `device`: The Device that may be used to initialize parts of the RenderTarget.
    /// - `create_info`: The CreateInfo struct specific to the backend RenderTarget, which we use to pass target-specific arguments.
    /// 
    /// # Returns
    /// A new instance of the backend RenderTarget.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn new(device: Rc<Device>, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
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
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Defines a Render-capable pipeline.
pub trait RenderPipelineBuilder<'a>: RenderPipeline {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo: 'a + Sized + Debug + Clone;


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
    fn new(device: Rc<Device>, target: &dyn RenderTarget, command_pool: Rc<CommandPool>, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
}
