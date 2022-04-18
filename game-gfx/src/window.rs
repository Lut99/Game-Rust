/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   18 Apr 2022, 15:48:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use std::fmt::Debug;
use std::sync::Arc;

use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder, WindowId};

use game_vk::auxillary::{ImageAspect, ImageViewKind};
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;

pub use crate::errors::WindowError as Error;
use crate::spec::{RenderTarget, RenderTargetBuilder, RenderTargetKind};


/***** WINDOW *****/
/// The CreateInfo for this Window.
#[derive(Debug, Default, Clone)]
pub struct CreateInfo {
    /// The title of the new window.
    pub title : String,

    /// The desired width of the window.
    pub width  : u32,
    /// The desired height of the window.
    pub height : u32,

    /// The number of images we would like as minimum for the swapchain.
    pub image_count : u32,
}



/// Manages a single Window and associated resources.
/// 
/// Note that this Window is modular, as in, the pipeline backend may be defined customly.
pub struct Window {
    /// The device that we used to build this Window in.
    device : Arc<Device>,

    /// The WinitWindow that we wrap.
    window    : WWindow,
    /// The Vulkan Surface that we create from this Window.
    surface   : Arc<Surface>,
    /// The Vulkan swapchain that we create from this Window.
    swapchain : Arc<Swapchain>,
    /// The list of Vulkan swapchain images that we create from this Window.
    views     : Vec<Arc<image::View>>,
    
    /// The title of this Window.
    title : String,
    /// The size of the window (as width, height)
    size  : (u32, u32),
}

impl Window {
    /// Updates the title in the internal window.
    /// 
    /// # Arguments
    /// - `new_title`: The new title of the Window.
    pub fn set_title(&mut self, new_title: &str) {
        // Set the title
        self.window.set_title(new_title);

        // Update the title internally too
        self.title = new_title.to_string();
    }



    // /// Returns the Device where the resources of this Window are bound to.
    // #[inline]
    // pub fn device(&self) -> &Arc<Device> { &self.device }



    // /// Returns the internal window object.
    // #[inline]
    // pub fn winit(&self) -> &WWindow { &self.window }

    // /// Returns the internal Vulkan Surface object.
    // #[inline]
    // pub fn surface(&self) -> &Arc<Surface> { &self.surface }

    // /// Returns the internal Vulkan Swapchain object.
    // #[inline]
    // pub fn swapchain(&self) -> &Arc<Swapchain> { &self.swapchain }

    // /// Returns the image views to the internal swapchain images.
    // #[inline]
    // pub fn views(&self) -> &Vec<Arc<image::View>> { &self.views }



    /// Returns the title of the window.
    #[inline]
    pub fn title(&self) -> &str { &self.title }

    // Returns the size of the window, as (width, height).
    #[inline]
    pub fn size(&self) -> &(u32, u32) { &self.size }

}

impl RenderTargetBuilder for Window {
    type CreateInfo = CreateInfo;


    /// Constructor for the Window.
    /// 
    /// This initializes a new Window and the 
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
    fn new(event_loop: &EventLoop<()>, device: Arc<Device>, create_info: Self::CreateInfo) -> Result<Self, Box<dyn std::error::Error>> {
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
        let surface = match Surface::new(device.instance().clone(), &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Box::new(Error::SurfaceCreateError{ err })); }
        };

        // Build the swapchain around the GPU and surface
        let swapchain = match Swapchain::new(device.clone(), surface.clone(), create_info.width, create_info.height, create_info.image_count) {
            Ok(swapchain) => swapchain,
            Err(err)      => { return Err(Box::new(Error::SwapchainCreateError{ err })); }
        };

        // Build the image views around the swapchain images
        let mut views: Vec<Arc<image::View>> = Vec::with_capacity(swapchain.images().len());
        for swapchain_image in swapchain.images() {
            // Create the view around it
            let view = match image::View::new(device.clone(), swapchain_image.clone(), image::ViewInfo {
                kind    : ImageViewKind::TwoD,
                format  : swapchain.format().into(),
                swizzle : Default::default(),

                aspect     : ImageAspect::Colour,
                base_level : 0,
                mip_levels : 1,
            }) {
                Ok(view) => view,
                Err(err) => { return Err(Box::new(Error::ImagesCreateError{ err })); }
            };

            // Store it in the list
            views.push(view);
        }



        // Done! Return the window
        debug!("Initialized new window '{}'", &create_info.title);
        Ok(Self {
            device : device,

            window : wwindow,
            surface,
            swapchain,
            views,

            title : create_info.title,
            size  : (create_info.width, create_info.height),
        })
    }
}

impl RenderTarget for Window {
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
