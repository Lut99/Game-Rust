/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 14:09:56
 * Last edited:
 *   05 Apr 2022, 17:47:00
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
        use InstanceError::*;
        match self {
            LoadError{ err }                      => write!(f, "Could not load the Vulkan library: {}", err),
            ExtensionEnumerateError{ layer, err } => write!(f, "Could not enumerate extensions properties{}: {}", if let Some(layer) = layer { format!(" for layer '{:?}'", layer) } else { String::new() }, err),
            LayerEnumerateError{ err }            => write!(f, "Could not enumerate layer properties: {}", err),
            UnknownExtension{ extension }         => write!(f, "Extension '{:?}' is not found in local Vulkan installation", extension),
            UnknownLayer{ layer }                 => write!(f, "Layer '{:?}' is not found in local Vulkan installation", layer),

            CreateError{ err }      => write!(f, "Could not create Vulkan instance: {}", err),
            DebugCreateError{ err } => write!(f, "Could not create Vulkan debug messenger: {}", err),
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

    /// Could not get whether or not the given surface is supported
    SurfaceSupportError{ err: ash::vk::Result },
    /// Could not get the capabilities of the given surface
    SurfaceCapabilitiesError{ err: ash::vk::Result },
    /// Could not get the formats of the given surface
    SurfaceFormatsError{ err: ash::vk::Result },
    /// Could not get the present modes of the given surface
    SurfacePresentModesError{ err: ash::vk::Result },
    /// The given surface is not supported at all
    UnsupportedSurface,
}

impl Display for GpuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use GpuError::*;
        match self {
            DeviceExtensionEnumerateError{ err }                 => write!(f, "Could not enumerate device extension properties: {}", err),
            UnsupportedDeviceExtension{ index, name, extension } => write!(f, "Physical device {} ({}) does not support extension '{:?}'; choose another device", index, name, extension),
            DeviceLayerEnumerateError{ err }                     => write!(f, "Could not enumerate device layer properties: {}", err),
            UnsupportedDeviceLayer{ index, name, layer }         => write!(f, "Physical device {} ({}) does not support layer '{:?}'; choose another device", index, name, layer),
            UnsupportedFeature{ index, name, feature }           => write!(f, "Physical device {} ({}) does not support feature '{}'; choose another device", index, name, feature),

            OperationUnsupported{ index, name, operation } => write!(f, "Physical device {} ({}) does not have queues that support '{:?}'; choose another device", index, name, operation),

            PhysicalDeviceEnumerateError{ err }   => write!(f, "Could not enumerate physical devices: {}", err),
            PhysicalDeviceNotFound{ index }       => write!(f, "Could not find physical device '{}'; see the list of available devices by running 'list'", index),
            PhysicalDeviceNameError{ index, err } => write!(f, "Could not parse name of device {} as UTF-8: {}", index, err),
            DeviceCreateError{ err }              => write!(f, "Could not create logical device: {}", err),

            NoSupportedPhysicalDevices => write!(f, "No GPU found that supports this application"),

            SurfaceSupportError{ err }      => write!(f, "Could not query swapchain support for surface: {}", err),
            SurfaceCapabilitiesError{ err } => write!(f, "Could not query supported swapchain capabilities for surface: {}", err),
            SurfaceFormatsError{ err }      => write!(f, "Could not query supported swapchain formats for surface: {}", err),
            SurfacePresentModesError{ err } => write!(f, "Could not query supported swapchain present modes for surface: {}", err),
            UnsupportedSurface              => write!(f, "The given surface is not supported by the chosen GPU"),
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
        use SurfaceError::*;
        match self {
            WindowsSurfaceKHRCreateError{ err } => write!(f, "Could not create new Windows SurfaceKHR: {}", err),
            MacOSSurfaceKHRCreateError{ err }   => write!(f, "Could not create new macOS SurfaceKHR: {}", err),
            UnsupportedWindowSystem             => write!(f, "Target window is not an X11 or Wayland window; other window systems are not supported"),
            X11SurfaceKHRCreateError{ err }     => write!(f, "Could not create new X11 SurfaceKHR: {}", err),
            WaylandSurfaceCreateError{ err }    => write!(f, "Could not create new Wayland SurfaceKHR: {}", err),
        }
    }
}

impl Error for SurfaceError {}



/// Defines errors that occur when setting up a Surface.
#[derive(Debug)]
pub enum SwapchainError {
    /// The given surface was not supported at all by the given GPU.
    GpuSurfaceSupportError{ index: usize, name: String, err: GpuError },
    /// Could not find an appropriate format for this GPU / surface combo.
    NoFormatFound,
    /// Could not create a new swapchain
    SwapchainCreateError{ err: ash::vk::Result },
    /// Could not get the images from the swapchain
    SwapchainImagesError{ err: ash::vk::Result },
}

impl Display for SwapchainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SwapchainError::*;
        match self {
            GpuSurfaceSupportError{ index, name, err } => write!(f, "Gpu {} ('{}') does not support given Surface: {}", index, name, err),
            NoFormatFound                              => write!(f, "No suitable formats found for swapchain; try choosing another device."),
            SwapchainCreateError{ err }                => write!(f, "Could not create Swapchain: {}", err),
            SwapchainImagesError{ err }                => write!(f, "Could not get Swapchain images: {}", err),
        }
    }
}

impl Error for SwapchainError {}



/// Defines errors that relate to an ImageView.
#[derive(Debug)]
pub enum ImageViewError {
    /// The constructor that builds on an Image is not yet implemented.
    NotImplemented,
}

impl Display for ImageViewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ImageViewError::*;
        match self {
            NotImplemented => write!(f, "View::new() is not yet implemented"),
        }
    }
}

impl Error for ImageViewError {}
