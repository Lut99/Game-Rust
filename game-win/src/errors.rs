/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:30:45
 * Last edited:
 *   01 Apr 2022, 17:55:33
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
}

impl Display for SurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
        }
    }
}

impl Error for SurfaceError {}



/// Defines errors that occur when setting up a Window.
#[derive(Debug)]
pub enum WindowError {
    /// Could not create a new Windows surface
    WindowsSurfaceKHRCreateError{ err: ash::vk::Result },

    /// Could not build a winit window.
    WinitBuildError{ err: winit::error::OsError },
    /// Could not build a surface around the new winit window.
    SurfaceBuildError{ err: SurfaceError },
}

impl Display for WindowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            WindowError::WindowsSurfaceKHRCreateError{ err } => write!(f, "Could not create new Windows SurfaceKHR: {}", err),

            WindowError::WinitBuildError{ err }   => write!(f, "Could not build a new winit window: {}", err),
            WindowError::SurfaceBuildError{ err } => write!(f, "Could not build Surface: {}", err),
        }
    }
}

impl Error for WindowError {}
