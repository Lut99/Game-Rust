//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 18:08:27
//  Last edited:
//    31 Jul 2022, 15:44:57
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the possible errors that may arise within the `game-gfx`
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Defines errors that originate in the main code of the RenderSystem.
#[derive(Debug)]
pub enum RenderError {
    /// Failed to create a new Vulkan Instance
    InstanceCreateError{ err: game_vk::instance::Error },
    /// Failed to create a new Device
    DeviceCreateError{ err: game_vk::device::Error },
    /// Failed to create a new CommandPool
    CommandPoolCreateError{ err: game_vk::pools::command::Error },
    /// Failed to create a new Window
    WindowCreateError{ title: String, err: WindowError },
    /// Failed to create a new Pipeline
    PipelineCreateError{ name: &'static str, err: PipelineError },
}

impl Display for RenderError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderError::*;
        match self {
            InstanceCreateError{ err }       => write!(f, "Could not create a new Instance: {}", err),
            DeviceCreateError{ err }         => write!(f, "Could not create a new Device: {}", err),
            CommandPoolCreateError{ err }    => write!(f, "Could not create a new CommandPool: {}", err),
            WindowCreateError{ title, err }  => write!(f, "Could not create Window '{}': {}", title, err),
            PipelineCreateError{ name, err } => write!(f, "Could not create Pipeline '{}': {}", name, err),
        }
    }
}

impl Error for RenderError {}



/// Defines errors that relate to winit windows and the Window component.
#[derive(Debug)]
pub enum WindowError {
    /// The given monitor was unknown to us.
    UnknownMonitor{ got: usize, expected: usize },
    /// No monitors at all were found.
    NoMonitors,
    /// The given video mode is not supported by the given monitor.
    UnsupportedVideoMode{ monitor: usize, resolution: (u32, u32), refresh_rate: u16, bit_depth: u16 },
    /// Could not create the winit window.
    WinitCreateError{ title: String, err: winit::error::OsError },
    /// Could not create the surface for the new window.
    SurfaceCreateError{ title: String, err: game_vk::surface::Error },
    /// Could not create the swapchain for the new window.
    SwapchainCreateError{ title: String, err: game_vk::swapchain::Error },
    /// Could not create the ImageViews from the Swapchain.
    ViewsCreateError{ title: String, err: game_vk::image::ViewError },
}

impl Display for WindowError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowError::*;
        match self {
            UnknownMonitor{ got, expected }                                      => write!(f, "Given monitor with index {} is unknown to a system with {} monitors ({} >= {})", got, expected, got, expected),
            NoMonitors                                                           => write!(f, "No monitors to place a window found"),
            UnsupportedVideoMode{ monitor, resolution, refresh_rate, bit_depth } => write!(f, "Monitor {} does not support a video mode of {}x{}@{} ({} bpp)", monitor, resolution.0, resolution.1, refresh_rate, bit_depth),
            WinitCreateError{ title, err }                                       => write!(f, "Could not create new window '{}': {}", title, err),
            SurfaceCreateError{ title, err }                                     => write!(f, "Could not create new surface '{}': {}", title, err),
            SwapchainCreateError{ title, err }                                   => write!(f, "Could not create new swapchain '{}': {}", title, err),
            ViewsCreateError{ title, err }                                       => write!(f, "Could not create the image views for the swapchain images of window '{}': {}", title, err),
        }
    }
}

impl Error for WindowError {}



/// Defines errors that may occur in any type of pipeline.
#[derive(Debug)]
pub enum PipelineError {
    /// Failed to create a new RenderPass.
    RenderPassCreateError{ name: &'static str, err: game_vk::render_pass::Error },

    /// Failed to create a new Vulkan-backend Pipeline.
    VkPipelineCreateError{ name: &'static str, err: game_vk::pipeline::Error },

    /// Failed to create a new framebuffer.
    FramebufferCreateError{ name: &'static str, err: game_vk::framebuffer::Error },

    /// Failed to create a new Buffer object.
    BufferCreateError{ name: &'static str, what: &'static str, err: game_vk::pools::memory::Error },
    /// Failed to map a Buffer to host memory.
    BufferMapError{ name: &'static str, what: &'static str, err: game_vk::pools::memory::Error },
    /// Failed to flush a mapped memory range.
    BufferFlushError{ name: &'static str, what: &'static str, err: game_vk::pools::memory::Error },
    /// Failed to copy one buffer to another.
    BufferCopyError{ name: &'static str, src: &'static str, dst: &'static str, err: game_vk::pools::memory::Error },

    /// Failed to allocate a new CommandBuffer.
    CommandBufferAllocateError{ name: &'static str, err: game_vk::pools::command::Error },
    /// Failed to record a new CommandBuffer.
    CommandBufferRecordError{ name: &'static str, err: game_vk::pools::command::Error },

    /// Failed to create a new PipelineLayout
    PipelineLayoutCreateError{ name: &'static str, err: game_vk::layout::Error },
}

impl Display for PipelineError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use PipelineError::*;
        match self {
            RenderPassCreateError{ name, err }     => write!(f, "Could not create a new RenderPass for the {} pipeline: {}", name, err),

            VkPipelineCreateError{ name, err }     => write!(f, "Could not create a new Vulkan Pipeline for the {} pipeline: {}", name, err),

            FramebufferCreateError{ name, err }    => write!(f, "Could not create a new Framebuffer for the {} pipeline: {}", name, err),

            BufferCreateError{ name, what, err }   => write!(f, "Could not create a new {} buffer for the {} pipeline: {}", what, name, err),
            BufferMapError{ name, what, err }      => write!(f, "Could not map a {} buffer to host memory for the {} pipeline: {}", what, name, err),
            BufferFlushError{ name, what, err }    => write!(f, "Could not flush mapped memory range of a {} buffer for the {} pipeline: {}", what, name, err),
            BufferCopyError{ name, src, dst, err } => write!(f, "Could not copy a {} buffer to a {} buffer for the {} pipeline: {}", src, dst, name, err),

            CommandBufferAllocateError{ name, err } => write!(f, "Could not create a new CommandBuffer for the {} pipeline: {}", name, err),
            CommandBufferRecordError{ name, err }   => write!(f, "Could not record a CommandBuffer for the {} pipeline: {}", name, err),

            PipelineLayoutCreateError{ name, err } => write!(f, "Could not create a new PipelineLayout for the {} pipeline: {}", name, err),
        }
    }
}

impl Error for PipelineError {}
