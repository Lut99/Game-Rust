/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:30:45
 * Last edited:
 *   03 Apr 2022, 12:47:14
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects errors from all across the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that occur when setting up a Surface.
#[derive(Debug)]
pub enum SurfaceError {
    /// Could not create a new Windows surface
    WindowsSurfaceKHRCreateError{ err: ash::vk::Result },
    /// Could not create a new macOS surface
    MacOSSurfaceKHRCreateError{ err: ash::vk::Result },
    /// This linux installation does not use X11 or Wayland
    UnsupportedWindowSystem,
    /// Could not create a new X11 surface
    X11SurfaceKHRCreateError{ err: ash::vk::Result },
    /// Could not create a new Wayland surface
    WaylandSurfaceCreateError{ err: ash::vk::Result },
}

impl Display for SurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            SurfaceError::WindowsSurfaceKHRCreateError{ err } => write!(f, "Could not create new Windows SurfaceKHR: {}", err),
            SurfaceError::MacOSSurfaceKHRCreateError{ err }   => write!(f, "Could not create new macOS SurfaceKHR: {}", err),
            SurfaceError::UnsupportedWindowSystem             => write!(f, "Target window is not an X11 or Wayland window; other window systems are not supported"),
            SurfaceError::X11SurfaceKHRCreateError{ err }     => write!(f, "Could not create new X11 SurfaceKHR: {}", err),
            SurfaceError::WaylandSurfaceCreateError{ err }    => write!(f, "Could not create new Wayland SurfaceKHR: {}", err),
        }
    }
}

impl Error for SurfaceError {}



/// Defines errors that occur when setting up a Window.
#[derive(Debug)]
pub enum WindowError {
    /// Could not build a winit window.
    WinitBuildError{ err: winit::error::OsError },
    /// Could not build a surface around the new winit window.
    SurfaceBuildError{ err: game_vk::surface::Error },
}

impl Display for WindowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            WindowError::WinitBuildError{ err }   => write!(f, "Could not build a new winit window: {}", err),
            WindowError::SurfaceBuildError{ err } => write!(f, "Could not build Surface: {}", err),
        }
    }
}

impl Error for WindowError {}
