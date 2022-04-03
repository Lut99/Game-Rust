/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   03 Apr 2022, 15:30:01
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains interfaces and other structs for the GFX crate.
**/

use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FResult};

use winit::event_loop::EventLoop;
use winit::window::WindowId;

use game_utl::traits::AsAny;
use game_vk::instance::Instance;
use game_vk::gpu::Gpu;


/***** RENDER TARGET STAGE *****/
/// Defines the type of the RenderTarget. This is used to sort them efficiently for type-specific treatments.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RenderTargetKind {
    /// A Window
    Window,
}

impl Display for RenderTargetKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderTargetKind::*;
        match self {
            Window => write!(f, "Window"),
        }
    }
}



/// Defines whenever RenderTargets may be called.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RenderTargetStage {
    /// The target will be called during the main render loop.
    MainLoop,
}

impl Display for RenderTargetStage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderTargetStage::*;
        match self {
            MainLoop => write!(f, "Main Render Loop"),
        }
    }
}





/***** RENDER TARGET TRAIT *****/
/// Defines a target that the RenderSystem may render to (like a Window or an Image).
pub trait RenderTarget: 'static + AsAny {
    /// Returns the type of this target.
    fn kind(&self) -> RenderTargetKind;



    /// Renders a single frame to the given RenderTarget.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderSystem either during the main render loop or in some other instance.
    /// 
    /// You can assume that the synchronization with e.g. swapchains is already been done.
    /// 
    /// # Errors
    /// 
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn Error>>;



    /// Returns the identifier of this window if it is a Window, or None otherwise.
    #[inline]
    fn window_id(&self) -> Option<WindowId> { None }
    
    /// Requests a redraw on this window if this is a window. Does nothing otherwise.
    #[inline]
    fn window_request_redraw(&self) {}
}



/// Defines the interface to build a new RenderTarget.
pub trait RenderTargetBuilder: RenderTarget {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo: Sized + Debug + Default + Clone;
    

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
    fn new(event_loop: &EventLoop<()>, instance: &Instance, gpu: &Gpu, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
}





/***** RENDER PIPELINE TRAIT *****/
/// Defines a customizeable backend for the graphics pipeline(s) for most of the RenderTargets.
pub trait RenderPipeline {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderTarget to perform a render pass.
    /// 
    /// You can assume that the synchronization with e.g. swapchains is already been done.
    /// 
    /// # Errors
    /// 
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Defines a customizeable backend for the graphics pipeline(s) for most of the RenderTargets.
pub trait RenderPipelineBuilder: RenderPipeline {
    /// Defines the arguments that will be passed as a single struct to the constructor.
    type CreateInfo: Sized + Debug + Default + Clone;


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
    fn new(event_loop: &EventLoop<()>, instance: &Instance, create_info: Self::CreateInfo) -> Result<Self, Box<dyn Error>>
        where Self: Sized;
}
