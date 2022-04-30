/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   30 Apr 2022, 17:15:42
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains interfaces and other structs for the GFX crate.
**/

use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FResult};
use std::sync::Arc;

use game_utl::traits::AsAny;
use game_vk::auxillary::ImageFormat;
use game_vk::device::Device;
use game_vk::image::Image;


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
    /// Returns a renderable target, i.e., an Image to render to.
    /// 
    /// # Returns
    /// A new Image on success.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn get_target(&mut self) -> Result<Arc<Image>, Box<dyn Error>>;
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
    fn new(device: Arc<Device>, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
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
    /// - `format`: The ImageFormat of the target frame where this pipeline will render to.
    /// - `create_info`: The CreateInfo struct specific to the backend RenderPipeline, which we use to pass target-specific arguments.
    /// 
    /// # Returns
    /// A new instance of the backend RenderPipeline.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn new(device: Arc<Device>, format: ImageFormat, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
}
