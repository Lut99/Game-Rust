/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   05 May 2022, 21:52:56
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use std::error;
use std::ptr;
use std::rc::Rc;

use ash::vk;
use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder, WindowId};

use game_vk::vec_as_ptr;
use game_vk::auxillary::{Extent2D, ImageAspect, ImageFormat, ImageViewKind};
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;
use game_vk::sync::Semaphore;

pub use crate::errors::WindowError as Error;
use crate::spec::RenderTarget;


/***** POPULATE FUNCTIONS *****/
/// Populates a VkPresentInfoKHR struct.
/// 
/// # Arguments
/// - `swapchains`: The list of Swapchains to present to.
/// - `indices`: The list of image indices in each Swapchain to present to.
/// - `wait_semaphores`: The list of Semaphores to wait to before presentation.
fn populate_present_info(swapchains: &[vk::SwapchainKHR], indices: &[u32], wait_semaphores: &[vk::Semaphore]) -> vk::PresentInfoKHR {
    // Do a few sanity checks
    if swapchains.len() != indices.len() { panic!("Given list of Swapchains (swapchains) is not the same length as the given list of indices (indices)"); }

    // Populate
    vk::PresentInfoKHR {
        // Set the standard stuff
        s_type : vk::StructureType::PRESENT_INFO_KHR,
        p_next : ptr::null(),

        // Set the swapchains and associated images to present to
        swapchain_count : swapchains.len() as u32,
        p_swapchains    : vec_as_ptr!(swapchains),
        p_image_indices : vec_as_ptr!(indices),

        // Set the semaphores to wait for
        wait_semaphore_count : wait_semaphores.len() as u32,
        p_wait_semaphores    : vec_as_ptr!(wait_semaphores),

        // We don't want per-swapchain results
        p_results : ptr::null::<vk::Result>() as *mut vk::Result,
    }
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
    swapchain : Rc<Swapchain>,
    /// The list of Vulkan swapchain images that we create from this Window.
    views     : Vec<Rc<image::View>>,
    
    /// The title of this Window.
    title : String,
    /// The size of the window (as width, height)
    size  : (u32, u32),
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

        // Build the image views around the swapchain images
        debug!("Initializing image views...");
        let mut views: Vec<Rc<image::View>> = Vec::with_capacity(swapchain.images().len());
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
                Err(err) => { return Err(Error::ImagesCreateError{ err }); }
            };

            // Store it in the list
            views.push(view);
        }



        // Done! Return the window
        debug!("Initialized new window '{}'", title);
        Ok(Self {
            device : device,

            window   : wwindow,
            _surface : surface,
            swapchain,
            views,

            title : title.to_string(),
            size  : (width, height),
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

    // Returns the size of the window, as (width, height).
    #[inline]
    pub fn size(&self) -> &(u32, u32) { &self.size }

}

impl RenderTarget for Window {
    /// Returns a list of all image views in the RenderTarget.
    #[inline]
    fn views(&self) -> &Vec<Rc<image::View>> { &self.views }

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
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<usize, Box<dyn error::Error>> {
        // Try to get an image from the swapchain
        let index = match self.swapchain.next_image(done_semaphore, None, None) {
            Ok(Some(index)) => index,
            Ok(None)        => { panic!("Swapchain resize is not yet implemented"); }
            Err(err)        => { return Err(Box::new(Error::SwapchainNextImageError{ err })); }
        };

        // Return that index
        Ok(index)
    }

    /// Presents this RenderTarget in the way it likes.
    /// 
    /// # Arguments
    /// - `index`: The index of the internal image to present.
    /// - `wait_semaphores`: Zero or more Semaphores that we should wait for before we can present the image.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn present(&self, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<(), Box<dyn error::Error>> {
        // Cast the semaphores
        let vk_wait_semaphores: Vec<vk::Semaphore> = wait_semaphores.iter().map(|sem| sem.vk()).collect();

        // Populate the present info struct.
        let vk_swapchains: [vk::SwapchainKHR; 1] = [self.swapchain.vk()];
        let vk_indices: [u32; 1] = [index as u32];
        let present_info = populate_present_info(&vk_swapchains, &vk_indices, &vk_wait_semaphores);

        // Present
        
    }



    /// Returns the ImageFormat of this RenderTarget.
    #[inline]
    fn format(&self) -> ImageFormat { self.swapchain.format() }

    /// Returns the extent of this RenderTarget.
    #[inline]
    fn extent(&self) -> &Extent2D<u32> { self.swapchain.extent() }
}
