/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   24 Jul 2022, 16:09:07
 * Last edited:
 *   24 Jul 2022, 16:11:04
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the errors for this crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Lists the errors for the WindowSystem.
#[derive(Debug)]
pub enum WindowError {
    /// Could not resolve the given monitor index.
    UnknownMonitor{ got: usize, expected: usize },
    /// No monitors at all found
    NoMonitors,
    /// The video mode with the given properties was not supported on the given monitor
    UnknownVideoMode{ monitor: usize, resolution: (u32, u32), refresh_rate: u16, bit_depth: u16 },
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
    /// Could not present the given swapchain image
    SwapchainPresentError{ err: game_vk::swapchain::Error },

    /// Could not wait for the Device to become idle
    IdleError{ err: game_vk::device::Error },
    /// Could not rebuild the swapchain
    SwapchainRebuildError{ err: game_vk::swapchain::Error },
    /// Could not rebuild some swapchain ImageView
    ViewRebuildError{ err: game_vk::image::view::Error },
}

impl Display for WindowError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowError::*;
        match self {
            UnknownMonitor{ got, expected }                                  => write!(f, "Unknown monitor index '{}' (found {} monitors)", got, expected),
            NoMonitors                                                       => write!(f, "No monitors found"),
            UnknownVideoMode{ monitor, resolution, refresh_rate, bit_depth } => write!(f, "Monitor {} does not support {}x{}@{} ({} bpp)", monitor, resolution.0, resolution.1, refresh_rate, bit_depth),
            WinitCreateError{ err }                                          => write!(f, "Could not build a new winit window: {}", err),
            SurfaceCreateError{ err }                                        => write!(f, "Could not build Surface: {}", err),
            SwapchainCreateError{ err }                                      => write!(f, "Could not build Swapchain: {}", err),
            ImagesCreateError{ err }                                         => write!(f, "Could not build Views around Swapchain images: {}", err),
            PipelineCreateError{ type_name, err }                            => write!(f, "Could not initialize RenderPipeline of type '{}': {}", type_name, err),

            SwapchainNextImageError{ err } => write!(f, "Could not get next Window frame: {}", err),
            SwapchainPresentError{ err }   => write!(f, "Could not present Swapchain image: {}", err),

            IdleError{ err }             => write!(f, "{}", err),
            SwapchainRebuildError{ err } => write!(f, "Could not rebuild Swapchain: {}", err),
            ViewRebuildError{ err }      => write!(f, "Could not rebuild ImageView: {}", err),
        }
    }
}

impl Error for WindowError {}
