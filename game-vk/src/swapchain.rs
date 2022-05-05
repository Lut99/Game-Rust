/* SWAPCHAIN.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 15:33:26
 * Last edited:
 *   05 May 2022, 21:19:14
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Wraps around the SwapchainKHR to provide the Swapchain to the Game.
**/

use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

use ash::vk;
use ash::extensions::khr;
use log::{debug, warn};

pub use crate::errors::SwapchainError as Error;
use crate::auxillary::{Extent2D, ImageFormat, SwapchainSupport};
use crate::device::Device;
use crate::surface::Surface;
use crate::image::Image;
use crate::sync::{Fence, Semaphore};


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
fn choose_sharing_mode(_device: &Rc<Device>) -> Result<(vk::SharingMode, u32, Vec<u32>), Error> {
    // Because we present with the same queue as we render, we only need exclusive
    Ok((vk::SharingMode::EXCLUSIVE, 0, vec![]))
}





/***** LIBRARY *****/
/// The Swapchain struct is used to render to and provide the RenderTarget's images.
pub struct Swapchain {
    /// The device where the Swapchain lives.
    device  : Rc<Device>,
    /// The surface around which the Swapchain wraps.
    surface : Rc<Surface>,

    /// The loader for the swapchain
    loader    : khr::Swapchain,
    /// The Swapchain itself
    swapchain : vk::SwapchainKHR,
    /// The images of the swapchain
    images    : Vec<Rc<Image>>,
    
    /// The chosen format of the swapchain
    format : ImageFormat,
    /// The chosen extent of the swapchain
    extent : Extent2D<u32>,
}

impl Swapchain {
    /// Constructor for the Swapchain.
    /// 
    /// Wraps a SwapchainKHR around the given GPU (Device) and surface (SurfaceKHR).
    /// 
    /// # Arguments
    /// - `device`: The Device to create the swapchain on.
    /// - `surface`: The Surface to create the swapchain around.
    /// - `width`: The initial width of the swapchain surface. Might be bounded to min/max width supported by this device/surface.
    /// - `height`: The initial height of the swapchain surface. Might be bounded to min/max height supported by this device/surface.
    /// - `image_count`: The number of images to put in the swapchain. Might be bounded by the min/max amount supported by this device/surface.
    /// 
    /// # Returns
    /// A new Swapchain instance on success, or else an Error explaining what went wrong.
    pub fn new(device: Rc<Device>, surface: Rc<Surface>, width: u32, height: u32, image_count: u32) -> Result<Rc<Self>, Error> {
        // First, query the Gpu's support for this surface
        let swapchain_support = match device.get_swapchain_support(&surface) {
            Ok(support) => support,
            Err(err)    => { return Err(Error::DeviceSurfaceSupportError{ index: device.index(), name: device.name().to_string(), err }); }
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
        let (sharing_mode, n_queue_families, queue_families) = choose_sharing_mode(&device)?;

        // Use the collect info for the CreateInfo
        let swapchain_info = vk::SwapchainCreateInfoKHR {
            // Do the standard fields
            s_type : vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next : ptr::null(),
            flags  : vk::SwapchainCreateFlagsKHR::empty(),

            // Define the surface to use
            surface : surface.vk(),

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
        let loader = khr::Swapchain::new(device.instance().vk(), device.ash());
        let swapchain = unsafe {
            match loader.create_swapchain(&swapchain_info, None) {
                Ok(swapchain) => swapchain,
                Err(err)      => { return Err(Error::SwapchainCreateError{ err }); }
            }
        };

        // Get the images of the chain
        let vk_images: Vec<vk::Image> = unsafe {
            match loader.get_swapchain_images(swapchain) {
                Ok(images) => images,
                Err(err)   => { return Err(Error::SwapchainImagesError{ err }); }
            }
        };

        // Wrap them in our own struct
        let mut images: Vec<Rc<Image>> = Vec::with_capacity(vk_images.len());
        for image in vk_images {
            // Wrap the image
            let image = match Image::from_vk(image) {
                Ok(image) => image,
                Err(err)  => { return Err(Error::ImageError{ err }); }
            };

            // Add it to the list
            images.push(image);
        }

        // Store everything in a new Swapchain instance and return
        Ok(Rc::new(Self {
            device,
            surface,

            loader,
            swapchain,
            images,
            
            format : format.into(),
            extent : extent.into(),
        }))
    }



    /// Tries to acquire the next image.
    /// 
    /// # Arguments
    /// - `semaphore`: An optional Semaphore to call when done.
    /// - `fence`: An optional Fence to call when done.
    /// - `timeout`: An optional timeout for waiting for a new image.
    /// 
    /// # Returns
    /// If the swapchain is still valid, returns the index of the image that is ready. If it's not valid but needs a resize, then 'None' is returned.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend failed to get the next image (for any other reason than a Swapchain that needs resizing).
    pub fn next_image(&self, semaphore: Option<&Rc<Semaphore>>, fence: Option<&Rc<Fence>>, timeout: Option<u64>) -> Result<Option<usize>, Error> {
        // Resolve the semaphores, fences and timeouts
        let vk_semaphore: vk::Semaphore = match semaphore {
            Some(semaphore) => semaphore.vk(),
            None            => vk::Semaphore::null(),
        };
        let vk_fence: vk::Fence = match fence {
            Some(fence) => fence.vk(),
            None        => vk::Fence::null(),
        };
        let vk_timeout: u64 = timeout.unwrap_or(u64::MAX);

        // Call the function on the internal loader
        let index = match unsafe { self.loader.acquire_next_image(self.swapchain, vk_timeout, vk_semaphore, vk_fence) } {
            Ok((index, not_optimal))                    => { if !not_optimal { index } else { return Ok(None); } },
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => { return Ok(None); }
            Err(err)                                    => { return Err(Error::SwapchainNextImageError{ err }); }
        };

        // Success; return it
        Ok(Some(index as usize))
    }



    /// Returns the device on which this swapchain is built.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the surface around which this swapchain is built.
    #[inline]
    pub fn surface(&self) -> &Rc<Surface> { &self.surface }



    /// Returns the loader for the swapchain.
    #[inline]
    pub fn ash(&self) -> &khr::Swapchain { &self.loader }

    /// Returns the Vulkan swapchain.
    #[inline]
    pub fn vk(&self) -> vk::SwapchainKHR { self.swapchain }

    /// Returns the images for the swapchain.
    #[inline]
    pub fn images(&self) -> &Vec<Rc<Image>> { &self.images }
    


    /// Returns the chosen format for this Swapchain.
    #[inline]
    pub fn format(&self) -> ImageFormat { self.format }

    /// Returns the chosen extent for this Swapchain.
    #[inline]
    pub fn extent(&self) -> &Extent2D<u32> { &self.extent }
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
