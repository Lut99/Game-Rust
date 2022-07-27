/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:25
 * Last edited:
 *   27 Jul 2022, 14:17:19
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors for the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use crate::spec::RenderPipelineId;


/***** ERRORS *****/
/// Defines the errors that happen at the base system itself.
#[derive(Debug)]
pub enum RenderSystemError {
    /// Could not instantiate the Vulkan instance
    InstanceCreateError{ err: game_vk::errors::InstanceError },
    /// Could not instantiate the Gpu
    DeviceCreateError{ err: game_vk::errors::DeviceError },
    /// Could not create the CommandPool
    CommandPoolCreateError{ err: game_vk::pools::errors::CommandPoolError },
    /// Could not initialize a new window component
    WindowCreateError{ title: String, err: WindowError },
    /// Could not initialize a new render pipeline.
    RenderPipelineCreateError{ name: &'static str, err: Box<dyn Error> },
    /// Failed to create a Semaphore
    SemaphoreCreateError{ err: game_vk::sync::Error },
    /// Failed to create a Fence
    FenceCreateError{ err: game_vk::sync::Error },

    /// Could not poll if a fence is ready
    FencePollError{ err: game_vk::sync::Error },
    /// Could not rebuild RenderPipeline
    PipelineRebuildError{ id: RenderPipelineId, err: PipelineError },
    /// Could not render one of the Pipelines
    RenderError{ err: PipelineError },
    /// COuld not present to one of the render targets
    PresentError{ err: PipelineError },

    /// Could not wait for the Device to become idle
    IdleError{ err: game_vk::device::Error },

    /// Could not auto-select a GPU
    DeviceAutoSelectError{ err: game_vk::errors::DeviceError },
    /// Could not list the GPUs
    DeviceListError{ err: game_vk::errors::DeviceError },
}

impl Display for RenderSystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use RenderSystemError::*;
        match self {
            InstanceCreateError{ err }             => write!(f, "Could not initialize graphics Instance: {}", err),
            DeviceCreateError{ err }               => write!(f, "Could not initialize Device: {}", err),
            CommandPoolCreateError{ err }          => write!(f, "Could not initialize CommandPool: {}", err),
            WindowCreateError{ title, err }        => write!(f, "Could not initialize new window '{}': {}", title, err),
            RenderPipelineCreateError{ name, err } => write!(f, "Could not initialize render pipeline '{}': {}", name, err),
            SemaphoreCreateError{ err }            => write!(f, "Failed to create Semaphore: {}", err),
            FenceCreateError{ err }                => write!(f, "Failed to create Fence: {}", err),

            FencePollError{ err }           => write!(f, "Could not poll Fence: {}", err),
            PipelineRebuildError{ id, err } => write!(f, "Could not rebuild Pipeline {}: {}", id, err),
            RenderError{ err }              => write!(f, "Could not render to RenderTarget: {}", err),
            PresentError{ err }             => write!(f, "Could not present to RenderTarget: {}", err),

            IdleError{ err } => write!(f, "{}", err),

            DeviceAutoSelectError{ err } => write!(f, "Could not auto-select a GPU: {}", err),
            DeviceListError{ err }       => write!(f, "Could not list GPUs: {}", err),
        }
    }
}

impl Error for RenderSystemError {}



/// Defines errors that occur when setting up a Window.
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

            SwapchainNextImageError{ err } => write!(f, "Could not get next Window frame: {}", err),
            SwapchainPresentError{ err }   => write!(f, "Could not present Swapchain image: {}", err),

            IdleError{ err }             => write!(f, "{}", err),
            SwapchainRebuildError{ err } => write!(f, "Could not rebuild Swapchain: {}", err),
            ViewRebuildError{ err }      => write!(f, "Could not rebuild ImageView: {}", err),
        }
    }
}

impl Error for WindowError {}



/// Defines errors that relate to RenderPipelines.
#[derive(Debug)]
pub enum PipelineError {
    /// Failed to create the PipelineLayout
    PipelineLayoutCreateError{ err: game_vk::layout::Error },
    /// Failed to create the RenderPass
    RenderPassCreateError{ err: game_vk::render_pass::Error },
    /// Failed to create a Vulkan pipeline
    VkPipelineCreateError{ err: game_vk::pipeline::Error },
    /// Failed to create a Framebuffer
    FramebufferCreateError{ err: game_vk::framebuffer::Error },
    /// Could not allocate a buffer
    BufferCreateError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not map the memory of a staging buffer
    BufferMapError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not flush a Buffer
    BufferFlushError{ what: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Failed to copy from one buffer to another.
    BufferCopyError{ src: &'static str, dst: &'static str, err: game_vk::pools::errors::MemoryPoolError },
    /// Could not allocate a new CommandBuffer
    CommandBufferAllocateError{ err: game_vk::pools::command::Error },
    /// Could not end a command buffer (because something else went wrong).
    CommandBufferRecordError{ err: game_vk::pools::command::Error },
    /// Could not create a semaphore
    SemaphoreCreateError{ err: game_vk::sync::Error },

    /// The swapchain of the pipeline's target needs to be rebuilt.
    SwapchainRebuildNeeded,
    /// Could not get the next swapchain image.
    SwapchainNextImageError{ err: WindowError },
    /// Could not submit the command buffer for rendering
    SubmitError{ err: game_vk::queue::Error },

    /// Could not present the rendered image.
    PresentError{ title: String, err: WindowError },

    /// Could not rebuild the Window.
    WindowRebuildError{ title: String, err: WindowError },
}

impl Display for PipelineError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use PipelineError::*;
        match self {
            PipelineLayoutCreateError{ err }  => write!(f, "Failed to create empty PipelineLayout: {}", err),
            RenderPassCreateError{ err }      => write!(f, "Failed to create RenderPass: {}", err),
            VkPipelineCreateError{ err }      => write!(f, "Failed to create Vulkan Pipeline: {}", err),
            FramebufferCreateError{ err }     => write!(f, "Failed to create Framebuffer: {}", err),
            BufferCreateError{ what, err }    => write!(f, "Failed to create {} buffer: {}", what, err),
            BufferMapError{ what, err }       => write!(f, "Could not map memory for {} buffer: {}", what, err),
            BufferFlushError{ what, err }     => write!(f, "Could not flush host memory for {} buffer: {}", what, err),
            BufferCopyError{ src, dst, err }  => write!(f, "Could not copy {} buffer to {} buffer: {}", src, dst, err),
            CommandBufferAllocateError{ err } => write!(f, "Could not allocate a new CommandBuffer for the Triangle pipeline: {}", err),
            CommandBufferRecordError{ err }   => write!(f, "Could not record a new CommandBuffer for the Triangle pipeline: {}", err),
            SemaphoreCreateError{ err }       => write!(f, "Could not create a new semaphore: {}", err),
            
            SwapchainRebuildNeeded         => write!(f, "The pipeline's target's swapchain needs to be rebuilt"),
            SwapchainNextImageError{ err } => write!(f, "Could not get next swapchain image of pipeline's target: {}", err),
            SubmitError{ err }             => write!(f, "Could not submit command buffer: {}", err),

            PresentError{ title, err } => write!(f, "Could not present rendered image to Window '{}': {}", title, err),

            WindowRebuildError{ title, err } => write!(f, "Could not rebuild the target '{}' Window: {}", title, err),
        }
    }
}

impl Error for PipelineError {}
