//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 18:03:29
//  Last edited:
//    07 Aug 2022, 13:33:30
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the errors for the `game-tgt` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Defines common errors that may occur when working with the RenderTargets.
#[derive(Debug)]
pub enum RenderTargetError {
    /// Failed to create a new image view.
    ViewCreateError{ name: String, err: rust_vk::image::ViewError },
    /// Failed to re-create a new image view.
    ViewRecreateError{ name: String, err: rust_vk::image::ViewError },

    /// Something non-common happened.
    Custom{ err: Box<dyn Error> },
}

impl Display for RenderTargetError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderTargetError::*;
        match self {
            ViewCreateError{ name, err }   => write!(f, "Failed to create image view for RenderTarget '{}': {}", name, err),
            ViewRecreateError{ name, err } => write!(f, "Failed to re-create image view for RenderTarget '{}': {}", name, err),

            Custom{ err } => write!(f, "{}", err),
        }
    }
}

impl Error for RenderTargetError {}



/// Defines errors that occur for RenderTargets that are Windows.
#[derive(Debug)]
pub enum WindowError {
    /// Could not create a new Window object.
    WindowCreateError{ err: rust_win::Error },

    /// Could not get the next swapchain image index.
    SwapchainNextImageError{ err: rust_vk::swapchain::Error },
    /// Could not present the image with the given index to the swapchain.
    SwapchainPresentError{ index: usize, err: rust_vk::swapchain::Error },
    /// Could not rebuild the Window.
    WindowRebuildError{ err: rust_win::Error },
}

impl Display for WindowError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowError::*;
        match self {
            WindowCreateError{ err } => write!(f, "Could not create new Window: {}", err),

            SwapchainNextImageError{ err }      => write!(f, "Could not get next swapchain image index: {}", err),
            SwapchainPresentError{ index, err } => write!(f, "Could not present swapchain image {}: {}", index, err),
            WindowRebuildError{ err }           => write!(f, "Could not rebuild window: {}", err),
        }
    }
}

impl Error for WindowError {}
