//  WINDOW.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 18:04:36
//  Last edited:
//    09 Aug 2022, 20:07:02
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements a RenderTarget trait for the `rust-win`'s Window.
// 

use std::cell::Ref;
use std::rc::Rc;

use winit::event_loop::EventLoop;

use log::debug;
use rust_vk::auxillary::enums::{ImageAspect, ImageFormat, ImageViewKind};
use rust_vk::auxillary::structs::Extent2D;
use rust_vk::device::Device;
use rust_vk::swapchain::Swapchain;
use rust_vk::image;
use rust_vk::sync::Semaphore;
use rust_win::Window;
use rust_win::spec::WindowInfo;

pub use crate::errors::{RenderTargetError, WindowError as Error};
use crate::spec::RenderTarget;


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `title`: The title of the Window for which we create images (only used for debugging).
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(title: &str, device: &Rc<Device>, swapchain: Ref<Swapchain>) -> Result<Vec<Rc<image::View>>, RenderTargetError> {
    // Rebuild all of the image views
    debug!("Generating image views...");
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
            Err(err) => { return Err(RenderTargetError::ViewCreateError{ name: format!("Window({})", title), err }); }
        };

        // Store it in the list
        views.push(view);
    }

    // Done, return
    Ok(views)
}





/***** LIBRARY *****/
/// Wraps around a Window to implement a RenderTarget.
pub struct WindowTarget {
    /// The Window that we wrap.
    window : Window,

    /// The image views that we wrap.
    views : Vec<Rc<image::View>>,

    /// A cached extent of the Window.
    extent : Extent2D<u32>,
}

impl WindowTarget {
    /// Constructor for the WindowTarget.
    /// 
    /// # Generic types:
    /// - `T`: The custom Event type for the EventLoop. If the EventLoop has no custom events, use `()`.
    /// 
    /// # Arguments
    /// - `event_loop`: The winit EventLoop where the new Window will be attached to.
    /// - `device`: The Device where the Window will be created.
    /// - `info`: The WindowInfo that contains the config for the new winit Window.
    /// 
    /// # Returns
    /// A new WindowTarget instance.
    /// 
    /// # Errors
    /// This function errors if we could not create a new Window or image views.
    pub fn new<T>(device: Rc<Device>, event_loop: &EventLoop<T>, info: WindowInfo) -> Result<Self, RenderTargetError> {
        // Create the Window
        let window: Window = match Window::new(device, event_loop, info, 3) {
            Ok(window) => window,
            Err(err)   => { return Err(RenderTargetError::Custom{ err: Box::new(Error::WindowCreateError{ err }) }); }
        };

        // Create the image views
        let views: Vec<Rc<image::View>> = create_views(window.title(), window.device(), window.swapchain().borrow())?;

        // Done
        let extent = window.extent();
        Ok(Self {
            window,

            views,

            extent,
        })
    }



    /// Returns the internal Window.
    #[inline]
    pub fn window(&self) -> &Window { &self.window }
}

impl RenderTarget for WindowTarget {
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
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<Option<usize>, RenderTargetError> {
        // Get a lock around the swapchain
        let swapchain = self.window.swapchain().borrow();

        // Try to get an image from the swapchain
        match swapchain.next_image(done_semaphore, None, None) {
            Ok(Some(index)) => Ok(Some(index)),
            Ok(None)        => Ok(None),
            Err(err)        => Err(RenderTargetError::Custom{ err: Box::new(Error::SwapchainNextImageError{ err }) }),
        }
    }

    /// Presents this RenderTarget in the way it likes.
    /// 
    /// # Arguments
    /// - `index`: The index of the internal image to present.
    /// - `wait_semaphores`: Zero or more Semaphores that we should wait for before we can present the image.
    /// 
    /// # Returns
    /// Whether or not the Target needs to be rebuild.
    /// 
    /// # Errors
    /// This function may error whenever the backend implementation likes. However, if it does, it should return a valid Error.
    fn present(&self, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<bool, RenderTargetError> {
        // Get a lock around the swapchain
        let swapchain = self.window.swapchain().borrow();

        // Call with the swapchain's function
        match swapchain.present(index as u32, wait_semaphores) {
            Ok(redo) => Ok(redo),
            Err(err) => Err(RenderTargetError::Custom{ err: Box::new(Error::SwapchainPresentError{ index, err }) }),
        }
    }



    /// Resize the RenderTarget to the new size.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    #[inline]
    fn rebuild(&mut self) -> Result<(), RenderTargetError> {
        // Simply recursively call the window
        if let Err(err) = self.window.rebuild() {
            return Err(RenderTargetError::Custom{ err: Box::new(Error::WindowRebuildError{ err }) });
        }

        // Next, rebuild the image views
        self.views = match create_views(self.window.title(), self.window.device(), self.window.swapchain().borrow()) {
            Ok(views)                                            => views,
            Err(RenderTargetError::ViewCreateError{ name, err }) => { return Err(RenderTargetError::ViewRecreateError{ name, err }); },
            Err(err)                                             => { return Err(err); },
        };

        // Done
        Ok(())
    }



    /// Returns a list of all image views in the RenderTarget.
    #[inline]
    fn views(&self) -> &[Rc<image::View>] { &self.views }

    /// Returns the ImageFormat of this RenderTarget.
    #[inline]
    fn format(&self) -> ImageFormat { self.window.format() }

    /// Returns a cached extent of this RenderTarget. Faster than quering the window, but might be inaccurate after resizes.
    #[inline]
    fn cached_extent(&self) -> &Extent2D<u32> { &self.extent }

    /// Returns the extent of this RenderTarget.
    #[inline]
    fn extent(&self) -> Extent2D<u32> { self.window.extent() }
}
