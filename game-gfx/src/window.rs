/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   18 Apr 2022, 12:07:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use std::fmt::Debug;
use std::sync::Arc;

use ash::vk;
use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder, WindowId};

use game_vk::image;
use game_vk::instance::Instance;
use game_vk::gpu::Gpu;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;

pub use crate::errors::WindowError as Error;
use crate::spec::{RenderPipeline, RenderPipelineBuilder, RenderTarget, RenderTargetBuilder, RenderTargetKind};


/***** WINDOW *****/
/// The CreateInfo for this Window.
#[derive(Debug, Default, Clone)]
pub struct CreateInfo<T: Debug + Default + Clone> {
    /// The title of the new window.
    pub title : String,

    /// The desired width of the window.
    pub width  : u32,
    /// The desired height of the window.
    pub height : u32,

    /// The number of images we would like as minimum for the swapchain.
    pub image_count : u32,

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
    /// The title of this Window.
    title : String,
    /// The size of the window (as width, height)
    size  : (u32, u32),

    // NOTE: The order of the next fields matters, due to Rust's in-order drop policy.
    /// The backend, as a RenderPipeline.
    pipeline : P,

    /// The list of Vulkan swapchain images that we create from this Window.
    images    : Vec<image::View>,
    /// The Vulkan swapchain that we create from this Window.
    swapchain : Swapchain,
    /// The Vulkan Surface that we create from this Window.
    surface   : Surface,
    /// The WinitWindow that we wrap.
    window    : WWindow,
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
    fn new(event_loop: &EventLoop<()>, instance: &Instance, gpu: Arc<Gpu>, create_info: Self::CreateInfo) -> Result<Self, Box<dyn std::error::Error>> {
        // Build the new Winit window
        let wwindow = match WindowBuilder::new()
            .with_title(&create_info.title)
            .with_inner_size(Size::Physical(PhysicalSize{ width: create_info.width, height: create_info.height }))
            .build(event_loop)
        {
            Ok(wwindow) => wwindow,
            Err(err)    => { return Err(Box::new(Error::WinitCreateError{ err })); }
        };

        // Build the surface around the window
        let surface = match Surface::new(instance, &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Box::new(Error::SurfaceCreateError{ err })); }
        };

        // Build the swapchain around the GPU and surface
        let swapchain = match Swapchain::new(instance, gpu, &surface, create_info.width, create_info.height, create_info.image_count) {
            Ok(swapchain) => swapchain,
            Err(err)      => { return Err(Box::new(Error::SwapchainCreateError{ err })); }
        };

        // Build the image views around the swapchain images
        let mut images: Vec<image::View> = Vec::with_capacity(swapchain.images().len());
        for swapchain_image in swapchain.images() {
            // Create the view around it
            let view = match image::View::from_vk(gpu, *swapchain_image, image::ViewInfo {
                kind    : vk::ImageViewType::TYPE_2D,
                format  : *swapchain.format(),
                swizzle : Default::default(),

                aspect     : vk::ImageAspectFlags::COLOR,
                base_level : 0,
                mip_levels : 1,
            }) {
                Ok(view) => view,
                Err(err) => { return Err(Box::new(Error::ImagesCreateError{ err })); }
            };

            // Store it in the list
            images.push(view);
        }



        // Build the render pipeline
        let pipeline = match P::new(event_loop, instance, create_info.pipeline_info) {
            Ok(pipeline) => pipeline,
            Err(err)     => { return Err(Box::new(Error::PipelineCreateError{ type_name: std::any::type_name::<P>(), err })); }
        };



        // Done! Return the window
        debug!("Initialized new window '{}'", &create_info.title);
        Ok(Self {
            title : create_info.title,
            size  : (create_info.width, create_info.height),

            window : wwindow,
            surface,
            swapchain,
            images,

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
