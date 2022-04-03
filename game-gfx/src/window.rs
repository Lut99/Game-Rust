/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   03 Apr 2022, 15:28:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use std::fmt::Debug;

use log::{debug, warn};
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder, WindowId};

use game_utl::traits::AsAny;
use game_vk::instance::Instance;
use game_vk::surface::Surface;

pub use crate::errors::WindowError as Error;
use crate::spec::{RenderPipeline, RenderPipelineBuilder, RenderTarget, RenderTargetBuilder, RenderTargetKind};


/***** WINDOW *****/
/// The CreateInfo for this Window.
#[derive(Debug, Default, Clone)]
pub struct CreateInfo<T: Debug + Default + Clone> {
    /// The title of the new window.
    pub title : String,

    /// The CreateInfo of the RenderPipeline.
    pub pipeline_info : T,
}



/// Manages a single Window and associated resources.
/// 
/// Note that this Window is modular, as in, the pipeline backend may be defined customly.
pub struct Window<P>
where
    P: RenderPipeline,
{
    // NOTE: The order of the next three matters, due to Rust's in-order drop policy.

    /// The title of this Window.
    title : String,

    /// The backend, as a RenderPipeline.
    pipeline : P,

    // /// The Vulkan swapchain that we create from this Window.
    /// The Vulkan Surface that we create from this Window.
    surface : Surface,
    /// The WinitWindow that we wrap.
    window  : WWindow,
}

impl<P> Window<P>
where
    P: 'static + RenderPipeline,
{
    /// Returns the title of the window.
    #[inline]
    fn title(&self) -> &str { &self.title }

    /// Updates the title in the internal window.
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// ```
    fn set_title(&mut self, new_title: &str) {
        // Set the title
        self.window.set_title(new_title);

        // Update the title internally too
        self.title = new_title.to_string();
    }



    // /// Returns the internal window object.
    // #[inline]
    // pub fn window(&self) -> &WWindow { &self.window }

    // /// Returns the internal Vulkan surface object.
    // #[inline]
    // pub fn surface(&self) -> &Surface { &self.surface }
}

impl<P> RenderTargetBuilder for Window<P>
where
    P: 'static + RenderPipelineBuilder,
{
    type CreateInfo = CreateInfo<P::CreateInfo>;


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
        // Build the new Winit window
        let wwindow = match WindowBuilder::new()
            .with_title(&create_info.title)
            .build(event_loop)
        {
            Ok(wwindow) => wwindow,
            Err(err)    => { return Err(Box::new(Error::WinitCreateError{ err })); }
        };

        // Build the surface around the window
        let surface = match Surface::new(instance.entry(), instance.instance(), &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Box::new(Error::SurfaceCreateError{ err })); }
        };



        // Build the render pipeline
        let pipeline = match P::new(event_loop, instance, create_info.pipeline_info) {
            Ok(pipeline) => pipeline,
            Err(err)     => { return Err(Box::new(Error::PipelineCreateError{ type_name: std::any::type_name::<P>(), err })); }
        };



        // Done! Return the window
        debug!("Initialized new window '{}'", &create_info.title);
        Ok(Self {
            title : create_info.title,

            window : wwindow,
            surface,

            pipeline,
        })
    }
}

impl<P> RenderTarget for Window<P>
where
    P: 'static + RenderPipeline,
{
    /// Returns the type of this target.
    #[inline]
    fn kind(&self) -> RenderTargetKind { RenderTargetKind::Window }



    /// Renders a single frame to the given RenderTarget.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderSystem either during the main render loop or in some other instance.
    /// 
    /// You can assume that the synchronization with e.g. swapchains is already been done.
    /// 
    /// # Errors
    /// 
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }



    /// Returns the identifier of this window if it is a Window, or None otherwise.
    #[inline]
    fn window_id(&self) -> Option<WindowId> { Some(self.window.id()) }
    
    /// Requests a redraw on this window if this is a window. Does nothing otherwise.
    #[inline]
    fn window_request_redraw(&self) { self.window.request_redraw() }
}
