/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   17 Apr 2022, 18:01:23
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
    GpuCreateError{ err: game_vk::errors::GpuError },

    /// The given target already exists
    DuplicateTarget{ type_name: &'static str, id: usize },
    /// Could not initialize a new render system.
    RenderTargetCreateError{ type_name: &'static str, err: String },
    
    /// Could not auto-select a GPU
    GpuAutoSelectError{ err: game_vk::errors::GpuError },
    /// Could not list the GPUs
    GpuListError{ err: game_vk::errors::GpuError },

    /// Could not render to one of the RenderTargets
    RenderError{ err: Box<dyn Error> },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            RenderSystemError::InstanceCreateError{ err } => write!(f, "Could not initialize graphics Instance: {}", err),
            RenderSystemError::GpuCreateError{ err }      => write!(f, "Could not initialize GPU: {}", err),

            RenderSystemError::DuplicateTarget{ type_name, id }          => write!(f, "Could not register a RenderTarget of type '{}': a target with id {} already exists", type_name, id),
            RenderSystemError::RenderTargetCreateError{ type_name, err } => write!(f, "Could not initialize render target of type '{}': {}", type_name, err),

            RenderSystemError::GpuAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            RenderSystemError::GpuListError{ err }       => write!(f, "Could not list GPUs: {}", err),

            RenderSystemError::RenderError{ err } => write!(f, "Could not render to RenderTarget: {}", err),
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
        }
    }
}

impl Error for WindowError {}
