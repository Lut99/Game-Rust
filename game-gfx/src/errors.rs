/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   01 May 2022, 17:59:50
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors for the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines the errors that happen at the base system itself.
#[derive(Debug)]
pub enum RenderSystemError {
    /// Could not instantiate the Vulkan instance
    InstanceCreateError{ err: game_vk::errors::InstanceError },
    /// Could not instantiate the Gpu
    DeviceCreateError{ err: game_vk::errors::DeviceError },

    /// Could not initialize a new render system.
    RenderTargetCreateError{ type_name: &'static str, err: String },

    /// Could not initialize a new render pipeline.
    RenderPipelineCreateError{ type_name: &'static str, err: String },
    
    /// Could not auto-select a GPU
    DeviceAutoSelectError{ err: game_vk::errors::DeviceError },
    /// Could not list the GPUs
    DeviceListError{ err: game_vk::errors::DeviceError },

    /// Could not render to one of the RenderTargets
    RenderError{ err: Box<dyn Error> },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderSystemError::*;
        match self {
            InstanceCreateError{ err } => write!(f, "Could not initialize graphics Instance: {}", err),
            DeviceCreateError{ err }   => write!(f, "Could not initialize Device: {}", err),

            RenderTargetCreateError{ type_name, err } => write!(f, "Could not initialize render target of type '{}': {}", type_name, err),
            
            RenderPipelineCreateError{ type_name, err } => write!(f, "Could not initialize render pipeline of type '{}': {}", type_name, err),

            DeviceAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            DeviceListError{ err }       => write!(f, "Could not list GPUs: {}", err),

            RenderError{ err } => write!(f, "Could not render to RenderTarget: {}", err),
        }
    }
}

impl Error for RenderSystemError {}



/// Defines errors that occur when setting up a Window.
#[derive(Debug)]
pub enum WindowError {
    /// Could not build a winit window.
    WinitCreateError{ err: winit::error::OsError },
    /// Could not build a surface around the new winit window.
    SurfaceCreateError{ err: game_vk::surface::Error },
    /// Could not build a swapchain around the new surface
    SwapchainCreateError{ err: game_vk::swapchain::Error },
    /// Could not collect the swapchain's images
    ImagesCreateError{ err: game_vk::image::ViewError },
    /// Could not build the child pipeline
    PipelineCreateError{ type_name: &'static str, err: Box<dyn Error> },

    /// Could not get the new swapchain image
    SwapchainNextImageError{ err: game_vk::swapchain::Error },
}

impl Display for WindowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowError::*;
        match self {
            WinitCreateError{ err }               => write!(f, "Could not build a new winit window: {}", err),
            SurfaceCreateError{ err }             => write!(f, "Could not build Surface: {}", err),
            SwapchainCreateError{ err }           => write!(f, "Could not build Swapchain: {}", err),
            ImagesCreateError{ err }              => write!(f, "Could not build Views around Swapchain images: {}", err),
            PipelineCreateError{ type_name, err } => write!(f, "Could not initialize RenderPipeline of type '{}': {}", type_name, err),

            SwapchainNextImageError{ err } => write!(f, "Could not get next Window frame: {}", err),
        }
    }
}

impl Error for WindowError {}
