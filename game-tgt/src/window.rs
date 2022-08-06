//  WINDOW.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 18:04:36
//  Last edited:
//    06 Aug 2022, 18:19:36
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements a RenderTarget trait for the `rust-win`'s Window.
// 

use std::rc::Rc;

use rust_vk::auxillary::enums::ImageFormat;
use rust_vk::auxillary::structs::Extent2D;
use rust_vk::image;
use rust_vk::sync::Semaphore;
use rust_win::Window;

pub use crate::errors::{RenderTargetError, WindowError as Error};
use crate::spec::RenderTarget;


/***** LIBRARY *****/
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
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<Option<usize>, RenderTargetError> {
        // Get a lock around the swapchain
        let swapchain = self.swapchain().borrow();

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
        let swapchain = self.swapchain().borrow();

        // Call with the swapchain's function
        match swapchain.present(index as u32, wait_semaphores) {
            Ok(redo) => Ok(redo),
            Err(err) => Err(RenderTargetError::Custom{ err: Box::new(Error::SwapchainPresentError{ index, err }) }),
        }
    }



    /// Resize the RenderTarget to the new size.
    /// 
    /// # Arguments
    /// - `new_size`: The new Extent2D of the RenderTarget.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    #[inline]
    fn rebuild(&mut self, _new_size: &Extent2D<u32>) -> Result<(), RenderTargetError> {
        // Simply recursively call the window
        match self.rebuild() {
            Ok(_)    => Ok(()),
            Err(err) => Err(RenderTargetError::Custom{ err: Box::new(Error::WindowRebuildError{ err }) }),
        }
    }



    /// Returns a list of all image views in the RenderTarget.
    #[inline]
    fn views(&self) -> &[Rc<image::View>] {
        self.views()
    }

    /// Returns the ImageFormat of this RenderTarget.
    #[inline]
    fn format(&self) -> ImageFormat {
        self.format()
    }

    /// Returns the extent of this RenderTarget.
    #[inline]
    fn extent(&self) -> Extent2D<u32> {
        self.extent()
    }
}
