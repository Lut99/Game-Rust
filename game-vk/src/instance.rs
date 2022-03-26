/* INSTANCE.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 14:10:40
 * Last edited:
 *   26 Mar 2022, 18:25:34
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the wrapper around the Vulkan instance.
**/

use std::ffi::CString;
use std::ptr;
use std::str::FromStr;

use ash::vk;
#[cfg(all(windows))]
use ash::extensions::khr::Win32Surface;
#[cfg(target_is = "macos")]
use ash::extensions::khr::MacOSSurface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
use semver::Version;

pub use crate::errors::InstanceError as Error;


/***** HELPER FUNCTIONS *****/
/// Returns the proper extensions for the target OS' window system.  
/// This overload is for Windows.
/// 
/// **Returns**  
/// The list of required extensions, as a list of CStrings.
#[cfg(all(windows))]
fn os_surface_extensions() -> Vec<*const i8> {
    vec![
        Win32Surface::name().as_ptr()
    ]
}

/// Returns the proper extensions for the target OS' window system.  
/// This overload is for macOS.
/// 
/// **Returns**  
/// The list of required extensions, as a list of CStrings.
#[cfg(target_os = "macos")]
fn os_surface_extensions() -> Vec<*const i8> {
    vec![
        MacOSSurface::name().as_ptr()
    ]
}

/// Returns the proper extensions for the target OS' window system.  
/// This overload is for Linux (X11).
/// 
/// **Returns**  
/// The list of required extensions, as a list of CStrings.
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
fn os_surface_extensions() -> Vec<*const i8> {
    vec![
        XlibSurface::name().as_ptr()
    ]
}





/***** LIBRARY *****/
/// Represents the Instance in the wrapper, which is the application-global instantiation of Vulkan and other libraries.
pub struct Instance {
    /// The ash entry, that determines how we link to the underlying Vulkan library
    _entry : ash::Entry,

    /// The instance object that this struct wraps.
    instance : ash::Instance,
}

impl Instance {
    /// Constructor for the Instance.
    /// 
    /// **Generic types**
    ///  * `S1`: The String-like type of the name.
    ///  * `S2`: The String-like type of the engine name.
    ///  * `I1`: The Iterator-type for the extension names.
    ///  * `I2`: The Iterator-type for the layer names.
    /// 
    /// **Arguments**
    ///  * `name`: The name of the calling application.
    ///  * `version`: The version of the calling application.
    ///  * `engine`: The name of the engine of the calling application.
    ///  * `engine_version`: The version of the engine of the calling application.
    ///  * `extensions`: Extra extensions to enable on top of the required ones for the current platform.
    ///  * `layers`: Vulkan validation layers to enable.
    /// 
    /// **Returns**  
    /// The new Instance on success, or else an Error.
    pub fn new<'a, 'b, S1: AsRef<str>, S2: AsRef<str>, I1: IntoIterator<Item=&'a str>, I2: IntoIterator<Item=&'b str>>(name: S1, engine: S2, engine_version: Version, extensions: I1, layers: I2) -> Result<Self, Error> {
        // Convert the str-like into String
        let name: &str   = name.as_ref();
        let engine: &str = engine.as_ref();
        // Convert the iterators into actual iterators
        let extensions = extensions.into_iter();
        let layers     = layers.into_iter();

        // Create the entry
        let entry = unsafe {
            match ash::Entry::load() {
                Ok(entry) => entry,
                Err(err)  => { return Err(Error::LoadError{ err }); }
            }
        };

        // Get the version from cargo
        let version: Version = Version::from_str(env!("CARGO_PKG_VERSION")).expect("Could not parse CARGO_PKG_VERSION as Version");

        // Get a CString from the String
        let cname = CString::new(name.as_bytes()).expect("Given string contains a NULL-byte; this should never happen!");
        let cengine = CString::new(engine.as_bytes()).expect("Given string contains a NULL-byte; this should never happen!");

        // Construct the ApplicationInfo
        let app_info = vk::ApplicationInfo {
            s_type              : vk::StructureType::APPLICATION_INFO,
            p_next              : ptr::null(),
            p_application_name  : cname.as_ptr(),
            application_version : vk::make_api_version(0, version.major as u32, version.minor as u32, version.patch as u32),
            p_engine_name       : cengine.as_ptr(),
            engine_version      : vk::make_api_version(0, engine_version.major as u32, engine_version.minor as u32, engine_version.patch as u32),
            api_version         : vk::API_VERSION_1_1,
        };

        // Convert the extensions and layers into vectors of the appropriate type
        let cextensions: Vec<CString> = extensions.map(|s| CString::new(s.as_bytes()).expect("Given string contains a NULL-byte; this should never happen!")).collect();
        let clayers: Vec<CString>     = layers.map(|s| CString::new(s.as_bytes()).expect("Given string contains a NULL-byte; this should never happen!")).collect();
        let mut p_extensions: Vec<*const i8> = cextensions.iter().map(|s| s.as_ptr()).collect();
        let p_layers: Vec<*const i8>         = clayers.iter().map(|s| s.as_ptr()).collect();

        // Possibly extend the extensions based on the OS
        let mut required_extensions: Vec<*const i8> = os_surface_extensions();
        p_extensions.append(&mut required_extensions);

        // Prepare the create info for the Instance
        let create_info = vk::InstanceCreateInfo {
            s_type                     : vk::StructureType::INSTANCE_CREATE_INFO,
            p_next                     : ptr::null(),
            flags                      : vk::InstanceCreateFlags::empty(),
            p_application_info         : &app_info,
            pp_enabled_extension_names : p_extensions.as_ptr(),
            enabled_extension_count    : p_extensions.len() as u32,
            pp_enabled_layer_names     : p_layers.as_ptr(),
            enabled_layer_count        : p_layers.len() as u32,
        };

        // Use that to create the instance
        let instance: ash::Instance = unsafe {
            match entry.create_instance(&create_info, None) {
                Ok(instance) => instance,
                Err(err)     => { return Err(Error::CreateError{ err }); }
            }
        };

        // Finally, create the struct!
        Ok(Self {
            _entry : entry,
            instance,
        })
    }



    /// Returns (an immuteable reference to) the internal Vulkan instance.
    #[inline]
    pub fn instance(&self) -> &ash::Instance { &self.instance }
    
    // /// Returns (a muteable reference to) the internal Vulkan instance.
    // #[inline]
    // pub fn instance_mut(&mut self) -> &mut ash::Instance { &mut self.instance }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
