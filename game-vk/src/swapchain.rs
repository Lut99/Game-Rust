/* SWAPCHAIN.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 15:33:26
 * Last edited:
 *   03 Apr 2022, 16:40:24
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Wraps around the SwapchainKHR to provide the Swapchain to the Game.
**/

use std::ops::Deref;
use std::ptr;

use ash::vk;
use ash::extensions::khr;
use log::{debug, warn};

pub use crate::errors::SwapchainError as Error;
use crate::instance::Instance;
use crate::gpu::{Gpu, SwapchainSupport};
use crate::surface::Surface;


/***** HELPER FUNCTIONS *****/
/// Chooses an appropriate swapchain format from the available ones.
fn choose_format(swapchain_support: &SwapchainSupport) -> Result<(vk::Format, vk::ColorSpaceKHR), Error> {
    // Try to choose B8G8R8A8
    for avail_format in &swapchain_support.formats {
        if avail_format.format == vk::Format::B8G8R8A8_SRGB && avail_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
            return Ok((avail_format.format, avail_format.color_space));
        }
    }

    // Otherwise, choose the first one or something idc
    warn!("Preferred Format not found; using first one");
    match swapchain_support.formats.first() {
        Some(format) => {
            debug!("Using unpreferred format: {:?}", format);
            Ok((format.format, format.color_space))
        },
        None => Err(Error::NoFormatFound),
    }
}

/// Chooses an appropriate swapchain prsent mode from the available ones.
fn choose_present_mode(_swapchain_support: &SwapchainSupport) -> Result<vk::PresentModeKHR, Error> {
    // The FIFO is always guaranteed to be present, so hit it
    Ok(vk::PresentModeKHR::FIFO)
}

/// Chooses an appropriate swapchain extent.
fn choose_extent(swapchain_support: &SwapchainSupport, width: u32, height: u32) -> Result<vk::Extent2D, Error> {
    // Get the supported width & height boundries by the swapchain
    let wmin = swapchain_support.capabilities.min_image_extent.width;
    let hmin = swapchain_support.capabilities.min_image_extent.height;
    let wmax = swapchain_support.capabilities.max_image_extent.width;
    let hmax = swapchain_support.capabilities.max_image_extent.height;

    // Clap the width & height in between them
    let width = if width < wmin { warn!("Increasing width to {}", wmin); wmin }
    else if width > wmax { warn!("Decreasing width to {}", wmax); wmax }
    else { width };
    let height = if height < hmin { warn!("Increasing height to {}", hmin); hmin }
    else if height > hmax { warn!("Decreasing height to {}", hmax); hmax }
    else { height };

    // Return that as an extent
    Ok(vk::Extent2D{
        width,
        height,
    })
}

/// Chooses an appropriate image count for the swapchain.
fn choose_image_count(swapchain_support: &SwapchainSupport, image_count: u32) -> Result<u32, Error> {
    // Get the supported boundries by the swapchain
    let min = swapchain_support.capabilities.min_image_count;
    let max = swapchain_support.capabilities.max_image_count;

    // Clamp the image count in between that
    let image_count = if image_count < min { warn!("Increasing image_count to {}", min); min }
    else if image_count > max { warn!("Decreasing image_count to {}", max); max }
    else { image_count };

    // Return that as the count
    Ok(image_count)
}

/// Chooses an appropriate sharing mode for the swapchain.
fn choose_sharing_mode(_gpu: &Gpu) -> Result<(vk::SharingMode, u32, Vec<u32>), Error> {
    // Because we present with the same queue as we render, we only need exclusive
    Ok((vk::SharingMode::EXCLUSIVE, 0, vec![]))
}





/***** LIBRARY *****/
/// The Swapchain struct is used to render to and provide the RenderTarget's images.
pub struct Swapchain {
    /// The loader for the swapchain
    loader    : khr::Swapchain,
    /// The Swapchain itself
    swapchain : vk::SwapchainKHR,
    /// The images of the swapchain
    images    : Vec<vk::Image>,
}

impl Swapchain {
    /// Constructor for the Swapchain.
    /// 
    /// Wraps a SwapchainKHR around the given GPU (Device) and surface (SurfaceKHR).
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors if the Vulkan API backend does.
    pub fn new(instance: &Instance, gpu: &Gpu, surface: &Surface, width: u32, height: u32, image_count: u32) -> Result<Self, Error> {
        // First, query the Gpu's support for this surface
        let swapchain_support = match gpu.get_swapchain_support(surface) {
            Ok(support) => support,
            Err(err)    => { return Err(Error::GpuSurfaceSupportError{ index: gpu.index(), name: gpu.name().to_string(), err }); }
        };

        // Next, choose an appropriate swapchain format
        let (format, colour_space) = choose_format(&swapchain_support)?;
        // Next, choose an appropriate swapchain present mode
        let present_mode = choose_present_mode(&swapchain_support)?;
        // Then, choose the swapchain extent
        let extent = choose_extent(&swapchain_support, width, height)?;
        // Then, choose the image count
        let image_count = choose_image_count(&swapchain_support, image_count)?;
        // Finally, choose the charing mode
        let (sharing_mode, n_queue_families, queue_families) = choose_sharing_mode(gpu)?;

        // Use the collect info for the CreateInfo
        let swapchain_info = vk::SwapchainCreateInfoKHR {
            // Do the standard fields
            s_type : vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next : ptr::null(),
            flags  : vk::SwapchainCreateFlagsKHR::empty(),

            // Define the surface to use
            surface : surface.surface(),

            // Define the found properties
            image_format      : format,
            image_color_space : colour_space,
            present_mode,
            image_extent      : extent,
            min_image_count   : image_count,

            // Set the sharing mode, with potential queue families to share between if concurrent
            image_sharing_mode       : sharing_mode,
            queue_family_index_count : n_queue_families,
            p_queue_family_indices   : queue_families.as_ptr(),

            // Set some additional image properties
            // The image use, which we only use to render to with shaders
            image_usage        : vk::ImageUsageFlags::COLOR_ATTACHMENT,
            // The pre-transform to apply to the images before rendering (unchanged)
            pre_transform      : swapchain_support.capabilities.current_transform,
            // How to deal with the alpha channel
            composite_alpha    : vk::CompositeAlphaFlagsKHR::OPAQUE,
            // We clip the image at the edges
            clipped            : vk::TRUE,
            // The number of layers in the images (only used for stuff like stereophonic 3D etc)
            image_array_layers : 1,

            // If we re-create the swapchain, we can use this to speed the process up
            old_swapchain : vk::SwapchainKHR::null(),
        };

        // Create the swapchain with it
        debug!("Initializing swapchain...");
        let loader = khr::Swapchain::new(instance.instance(), gpu.device());
        let swapchain = unsafe {
            match loader.create_swapchain(&swapchain_info, None) {
                Ok(swapchain) => swapchain,
                Err(err)      => { return Err(Error::SwapchainCreateError{ err }); }
            }
        };

        // Get the images of the chain
        let images = unsafe {
            match loader.get_swapchain_images(swapchain) {
                Ok(images) => images,
                Err(err)   => { return Err(Error::SwapchainImagesError{ err }); }
            }
        };

        // Store everything in a new Swapchain instance and return
        Ok(Self {
            loader,
            swapchain,
            images,
        })
    }



    /// Returns the loader for the swapchain.
    #[inline]
    pub fn loader(&self) -> &khr::Swapchain { &self.loader }

    /// Returns the Vulkan swapchain.
    #[inline]
    pub fn swapchain(&self) -> vk::SwapchainKHR { self.swapchain }

    /// Returns the images for the swapchain.
    #[inline]
    pub fn images(&self) -> &Vec<vk::Image> { &self.images }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        debug!("Destroying Swapchain...");
        unsafe { self.loader.destroy_swapchain(self.swapchain, None); }
    }
}

impl Deref for Swapchain {
    type Target = khr::Swapchain;

    fn deref(&self) -> &Self::Target { &self.loader }
}
