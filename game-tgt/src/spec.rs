//  SPEC.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 18:04:05
//  Last edited:
//    07 Aug 2022, 18:59:28
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs for the `game-tgt` crate.
// 

use std::rc::Rc;

use rust_vk::auxillary::enums::ImageFormat;
use rust_vk::auxillary::structs::Extent2D;
use rust_vk::image;
use rust_vk::sync::Semaphore;

use game_utl::traits::AsAny;

pub use crate::errors::RenderTargetError as Error;


/***** LIBRARY *****/
/// Defines a target that the RenderSystem may render to (like a Window or an Image).
pub trait RenderTarget: 'static + AsAny {
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
    fn get_index(&self, done_semaphore: Option<&Rc<Semaphore>>) -> Result<Option<usize>, Error>;

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
    fn present(&self, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<bool, Error>;



    /// Resize the RenderTarget to the new size.
    /// 
    /// # Errors
    /// This function may error if we could not recreate / resize the required resources
    fn rebuild(&mut self) -> Result<(), Error>;



    /// Returns a list of all image views in the RenderTarget.
    fn views(&self) -> &[Rc<image::View>];

    /// Returns the ImageFormat of this RenderTarget.
    fn format(&self) -> ImageFormat;

    /// Returns a cached extent of this RenderTarget. Faster than quering the window, but might be inaccurate after resizes.
    fn cached_extent(&self) -> &Extent2D<u32>;

    /// Returns the extent of this RenderTarget.
    fn extent(&self) -> Extent2D<u32>;
}
