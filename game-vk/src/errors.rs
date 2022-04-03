/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 14:09:56
 * Last edited:
 *   03 Apr 2022, 12:46:04
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors for the crate.
**/

use std::error::Error;
use std::ffi::CString;
use std::fmt::{Display, Formatter, Result as FResult};


/***** ERRORS *****/
/// Defines errors that occur when setting up an Instance.
#[derive(Debug)]
pub enum InstanceError {
    /// Could not load the Vulkan library at runtime
    LoadError{ err: ash::LoadingError },
    /// Could not enumerate the extension properties (possible the extensions from a certain layer)
    ExtensionEnumerateError{ layer: Option<CString>, err: ash::vk::Result },
    /// Could not enumerate the layer properties
    LayerEnumerateError{ err: ash::vk::Result },
    /// Unknown extension encountered
    UnknownExtension{ extension: CString },
    /// Unknown layer encountered
    UnknownLayer{ layer: CString },

    /// Could not create the Instance
    CreateError{ err: ash::vk::Result },
    /// Could not create the debug messenger
    DebugCreateError{ err: ash::vk::Result },
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            InstanceError::LoadError{ err }                      => write!(f, "Could not load the Vulkan library: {}", err),
            InstanceError::ExtensionEnumerateError{ layer, err } => write!(f, "Could not enumerate extensions properties{}: {}", if let Some(layer) = layer { format!(" for layer '{:?}'", layer) } else { String::new() }, err),
            InstanceError::LayerEnumerateError{ err }            => write!(f, "Could not enumerate layer properties: {}", err),
            InstanceError::UnknownExtension{ extension }         => write!(f, "Extension '{:?}' is not found in local Vulkan installation", extension),
            InstanceError::UnknownLayer{ layer }                 => write!(f, "Layer '{:?}' is not found in local Vulkan installation", layer),

            InstanceError::CreateError{ err }      => write!(f, "Could not create Vulkan instance: {}", err),
            InstanceError::DebugCreateError{ err } => write!(f, "Could not create Vulkan debug messenger: {}", err),
        }
    }
}

impl Error for InstanceError {}



/// Defines errors that occur when setting up an Instance.
#[derive(Debug)]
pub enum GpuError {
    /// Could not enumerate over the available device extensions
    DeviceExtensionEnumerateError{ err: ash::vk::Result },
    /// The given device extension was not supported by the given device
    UnsupportedDeviceExtension{ index: usize, name: String, extension: CString },
    /// Could not enumerate over the available device layers
    DeviceLayerEnumerateError{ err: ash::vk::Result },
    /// The given device layer was not supported by the given device
    UnsupportedDeviceLayer{ index: usize, name: String, layer: CString },
    /// The given device feature was not supported by the given device
    UnsupportedFeature{ index: usize, name: String, feature: &'static str },

    /// One of the operations we want for the queue families is unsupported
    OperationUnsupported{ index: usize, name: String, operation: ash::vk::QueueFlags },

    /// Could not get the iterator over the physical devices
    PhysicalDeviceEnumerateError{ err: ash::vk::Result },
    /// Did not find the given physical device
    PhysicalDeviceNotFound{ index: usize },
    /// Could not convert the raw name of the device to a String
    PhysicalDeviceNameError{ index: usize, err: std::str::Utf8Error },
    /// Could not create the new logical device
    DeviceCreateError{ err: ash::vk::Result },

    /// None of the found devices support this application
    NoSupportedPhysicalDevices,
}

impl Display for GpuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            GpuError::DeviceExtensionEnumerateError{ err }                 => write!(f, "Could not enumerate device extension properties: {}", err),
            GpuError::UnsupportedDeviceExtension{ index, name, extension } => write!(f, "Physical device {} ({}) does not support extension '{:?}'; choose another device", index, name, extension),
            GpuError::DeviceLayerEnumerateError{ err }                     => write!(f, "Could not enumerate device layer properties: {}", err),
            GpuError::UnsupportedDeviceLayer{ index, name, layer }         => write!(f, "Physical device {} ({}) does not support layer '{:?}'; choose another device", index, name, layer),
            GpuError::UnsupportedFeature{ index, name, feature }           => write!(f, "Physical device {} ({}) does not support feature '{}'; choose another device", index, name, feature),

            GpuError::OperationUnsupported{ index, name, operation } => write!(f, "Physical device {} ({}) does not have queues that support '{:?}'; choose another device", index, name, operation),

            GpuError::PhysicalDeviceEnumerateError{ err }   => write!(f, "Could not enumerate physical devices: {}", err),
            GpuError::PhysicalDeviceNotFound{ index }       => write!(f, "Could not find physical device '{}'; see the list of available devices by running 'list'", index),
            GpuError::PhysicalDeviceNameError{ index, err } => write!(f, "Could not parse name of device {} as UTF-8: {}", index, err),
            GpuError::DeviceCreateError{ err }              => write!(f, "Could not create logical device: {}", err),

            GpuError::NoSupportedPhysicalDevices => write!(f, "No GPU found that supports this application"),
        }
    }
}

impl Error for GpuError {}



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
