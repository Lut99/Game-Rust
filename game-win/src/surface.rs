/* SURFACE.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:26:26
 * Last edited:
 *   01 Apr 2022, 17:57:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Vulkan Surface wrapper.
**/

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use ash::{Entry, Instance};
use ash::extensions::khr::Surface as SurfaceKHR;
use ash::vk;
use winit::window::Window as WWindow;

pub use crate::errors::SurfaceError as Error;


/***** HELPER FUNCTIONS *****/
/// Returns a new surface from the given window.
/// 
/// There are three overloads for this function, each for the target platform. This overload is for Windows.
/// 
/// # Examples
/// 
/// ```
/// // TBD
/// ```
/// 
/// # Errors
/// 
/// This function errors whenever the underlying APIs error.
#[cfg(all(windows))]
unsafe fn create_surface(entry: &Entry, instance: &Instance, wwindow: &WWindow) -> Result<SurfaceKHR, Error> {
    use ash::extensions::khr::Win32Surface;
    use winapi::shared::windef::HWND;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;

    
    // Get a Windows Window Handle
    let hwnd = window.hwnd() as HWND;
    // Get the instance handle for this process, which is Window's container of this process' windows
    let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;

    // Now create the appropriate create info
    let surface_info = vk::Win32SurfaceCreateInfoKHR {
        // Set the standard fields
        s_type : vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        p_next : ptr::null(),
        flags  : Default::default(),

        // Pass the instance and the window handle
        hinstance,
        hwnd : hwnd as *const c_void,
    };

    // Build the loader for the surface
    let loader = Win32Surface::new(entry, instance);
    // Create the new surface
    match loader.create_win32_surface(&surface_info, None) {
        Ok(surface) => Ok(surface),
        Err(err)    => { return Err(Error::WindowsSurfaceKHRCreateError{ err }); }
    }
}

/// Returns a new surface from the given window.
/// 
/// There are three overloads for this function, each for the target platform. This overload is for macOS.
/// 
/// # Examples
/// 
/// ```
/// // TBD
/// ```
/// 
/// # Errors
/// 
/// This function errors whenever the underlying APIs error.
#[cfg(target_os = "macos")]
unsafe fn create_surface(entry: &Entry, instance: &Instance, wwindow: &WWindow) -> Result<SurfaceKHR, Error> {
    use ash::extensions::mvk::MacOSSurface;
    use cocoa::base::id as cocoa_id;
    use winit::platform::macos::WindowExtMacOS;

    
    // Get the ID of the window
    let window: cocoa_id
}





/***** LIBRARY *****/
/// Implements a Surface, which can be build from a given Window object.
pub struct Surface {
    /// The SurfaceKHR which we wrap.
    surface : SurfaceKHR,
}

impl Surface {
    /// Constructor for the Surface.
    /// 
    /// This function tries to build a surface from the given winit::Window object.
    /// 
    /// # Examples 
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors whenever the backend Vulkan errors.
    pub fn new(entry: &Entry, instance: &Instance, wwindow: &WWindow) -> Result<Self, Error> {
        
    }
}
