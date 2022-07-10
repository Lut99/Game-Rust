/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   10 Jul 2022, 15:07:50
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use std::error;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder, WindowId};

use game_vk::auxillary::enums::{ImageAspect, ImageFormat, ImageViewKind};
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;
use game_vk::sync::Semaphore;

pub use crate::errors::WindowError as Error;
use crate::spec::RenderTarget;


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(device: &Rc<Device>, swapchain: &Arc<RwLock<Swapchain>>) -> Result<Vec<Rc<image::View>>, Error> {
    // Get a read lock for the rest
    let sc = swapchain.read().expect("Could not get read lock on Swapchain");

    // Rebuild all of the image views
    debug!("Generating image views...");
    let mut views: Vec<Rc<image::View>> = Vec::with_capacity(sc.images().len());
    for swapchain_image in sc.images() {
        // Create the view around it
        let view = match image::View::new(device.clone(), swapchain_image.clone(), image::ViewInfo {
            kind    : ImageViewKind::TwoD,
            format  : sc.format().into(),
            swizzle : Default::default(),

            aspect     : ImageAspect::Colour,
            base_level : 0,
            mip_levels : 1,
        }) {
            Ok(view) => view,
            Err(err) => { return Err(Error::ImagesCreateError{ err }); }
        };

        // Store it in the list
        views.push(view);
    }

    // Done, return
    Ok(views)
}





/***** WINDOW *****/
/// Manages a single Window and associated resources.
/// 
/// Note that this Window is modular, as in, the pipeline backend may be defined customly.
pub struct Window {
    /// The device that we used to build this Window in.
    device : Rc<Device>,

    /// The WinitWindow that we wrap.
    window    : WWindow,
    /// The Vulkan Surface that we create from this Window.
    _surface  : Rc<Surface>,
    /// The Vulkan swapchain that we create from this Window.
    swapchain : Arc<RwLock<Swapchain>>,
    /// The list of Vulkan swapchain images that we create from this Window.
    views     : Vec<Rc<image::View>>,
    
    /// The title of this Window.
    title  : String,
    /// The size of the window (as an extent)
    extent : Extent2D<u32>,
}

impl Window {
    /// Constructor for the Window.
    /// 
    /// This initializes a new Window as a RenderTarget.
    /// 
    /// # Arguments
    /// - `device`: The Device to bind the Swapchain etc to.
    /// - `event_loop`: The EventLoop to register Window events on.
    /// - `title`: The title of the Window.
    /// - `width`: The width of the Window, in pixels.
    /// - `height`: The height of the Window, in pixels.
    /// - `image_count`: The suggested number of images in the swapchain. May be bound by hardware limitations.
    /// 
    /// # Returns
    /// A new Window on success.
    /// 
    /// # Errors
    /// This function errors if the Window or any subsequent resource (like Surfaces or Swapchains) failed to be created. Will always be in the form of an Error.
    pub fn new(device: Rc<Device>, event_loop: &EventLoop<()>, title: &str, width: u32, height: u32, image_count: u32) -> Result<Self, Error> {
        // Build the new Winit window
        let wwindow = match WindowBuilder::new()
            .with_title(title)
            .with_inner_size(Size::Physical(PhysicalSize{ width, height }))
            .build(event_loop)
        {
            Ok(wwindow) => wwindow,
            Err(err)    => { return Err(Error::WinitCreateError{ err }); }
        };

        // Build the surface around the window
        let surface = match Surface::new(device.instance().clone(), &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Error::SurfaceCreateError{ err }); }
        };

        // Build the swapchain around the GPU and surface
        let swapchain = match Swapchain::new(device.clone(), surface.clone(), width, height, image_count) {
            Ok(swapchain) => swapchain,
            Err(err)      => { return Err(Error::SwapchainCreateError{ err }); }
        };

        // Generate the views
        let views: Vec<Rc<image::View>> = create_views(&device, &swapchain)?;

        // Done! Return the window
        debug!("Initialized new window '{}'", title);
        Ok(Self {
            device : device,

            window   : wwindow,
            _surface : surface,
            swapchain,
            views,

            title  : title.to_string(),
            extent : Extent2D::new(width, height),
        })
    }



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
    // pub fn device(&self) -> &Rc<Device> { &self.device }



    // /// Returns the internal window object.
    // #[inline]
    // pub fn winit(&self) -> &WWindow { &self.window }

    // /// Returns the internal Vulkan Surface object.
    // #[inline]
    // pub fn surface(&self) -> &Rc<Surface> { &self.surface }

    // /// Returns the internal Vulkan Swapchain object.
    // #[inline]
    // pub fn swapchain(&self) -> &Rc<Swapchain> { &self.swapchain }

    // /// Returns the image views to the internal swapchain images.
    // #[inline]
    // pub fn views(&self) -> &Vec<Rc<image::View>> { &self.views }



    /// Returns the parent device of this Window.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }



    /// Returns the identifier of this window if it is a Window, or None otherwise.
    #[inline]
    pub fn id(&self) -> WindowId { self.window.id() }
    
    /// Requests a redraw on this window if this is a window. Does nothing otherwise.
    #[inline]
    pub fn request_redraw(&self) { self.window.request_redraw() }



    /// Returns the title of the window.
    #[inline]
    pub fn title(&self) -> &str { &self.title }
}

impl RenderTarget for Window {
    /// Returns the index of a renderable target, i.e., an image::View to render to.
    /// 
    /// For non-Swapchain targets, this function will be very simple.
    /// 
    /// # Arguments
    /// - `done_semaphore`: Optional Semaphore that should be signalled when the image is available.
    /// 
    /// # Returns
    /// A new ImageView on success. It could be that stuff like Swapchains are outdated or invalid, in which case 'None' is returned.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<Option<usize>, Box<dyn error::Error>> {
        // Get a lock around the swapchain
        let sc = self.swapchain.read().expect("Could not get freshly created Swapchain lock");

        // Try to get an image from the swapchain
        match sc.next_image(done_semaphore, None, None) {
            Ok(Some(index)) => Ok(Some(index)),
            Ok(None)        => Ok(None),
            Err(err)        => Err(Box::new(Error::SwapchainNextImageError{ err })),
        }
    }

    /// Presents this RenderTarget in the way it likes.
    /// 
    /// # Arguments
    /// - `index`: The index of the internal image to present.
    /// - `wait_semaphores`: Zero or more Semaphores that we should wait for before we can present the image.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn present(&self, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<bool, Box<dyn error::Error>> {
        // Get a lock around the swapchain
        let sc = self.swapchain.read().expect("Could not get freshly created Swapchain lock");

        // Call with the swapchain's function
        match sc.present(index as u32, wait_semaphores) {
            Ok(redo) => Ok(redo),
            Err(err) => Err(Box::new(Error::SwapchainPresentError{ err })),
        }
    }



    /// Resizes the RenderTarget to the new size.
    /// 
    /// # Arguments
    /// - `new_size`: The new Extent2D of the RenderTarget.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self, new_size: &Extent2D<u32>) -> Result<(), Box<dyn error::Error>> {
        debug!("Rebuilding Window...");

        // Get a write lock on the swapchain
        {
            let mut sc = self.swapchain.write().expect("Could not get write lock on Swapchain");

            // Rebuild the swapchain (this will also make sure the device is idle, but with some nice busy time overlap)
            if let Err(err) = sc.rebuild(new_size.w, new_size.h) { return Err(Box::new(Error::SwapchainRebuildError{ err })); }
        }

        // Generate the views & store them
        self.views = create_views(&self.device, &self.swapchain)?;

        // Done; the Window has been resized
        Ok(())
    }



    /// Returns a list of all image views in the RenderTarget.
    #[inline]
    fn views(&self) -> &Vec<Rc<image::View>> { &self.views }

    /// Returns the ImageFormat of this RenderTarget.
    #[inline]
    fn format(&self) -> ImageFormat { self.swapchain.read().expect("Could not get read lock on Swapchain").format() }

    /// Returns the extent of this RenderTarget (cached but cheap).
    #[inline]
    fn extent(&self) -> &Extent2D<u32> { &self.extent }

    /// Returns the _actual_ extent of this RenderTarget (more expensive but accurate).
    fn real_extent(&self) -> Extent2D<u32> { Extent2D::new(self.window.inner_size().width, self.window.inner_size().height) }
}
