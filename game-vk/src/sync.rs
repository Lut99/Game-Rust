/* SYNC.rs
 *   by Lut99
 *
 * Created:
 *   01 May 2022, 17:26:00
 * Last edited:
 *   01 May 2022, 17:38:11
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains synchronization primitive wrappers.
**/

use std::ptr;
use std::sync::Arc;

use ash::vk;

pub use crate::errors::SyncError as Error;
use crate::device::Device;


/***** POPULATE FUNCTIONS *****/
/// Creates a new VkSemaphoreCreateInfo struct.
#[inline]
fn populate_semaphore_info() -> vk::SemaphoreCreateInfo {
    vk::SemaphoreCreateInfo {
        // Only set the default stuff
        s_type : vk::StructureType::SEMAPHORE_CREATE_INFO,
        p_next : ptr::null(),
        flags  : vk::SemaphoreCreateFlags::empty(),
    }
}

/// Creates a new VkFenceCreateInfo struct.
/// 
/// # Arguments
/// - `flags`: The VkFenceCreateFlags to initialize this Fence with.
#[inline]
fn populate_fence_info(flags: vk::FenceCreateFlags) -> vk::FenceCreateInfo {
    vk::FenceCreateInfo {
        // Only set the default stuff
        s_type : vk::StructureType::FENCE_CREATE_INFO,
        p_next : ptr::null(),
        flags,
    }
}





/***** LIBRARY *****/
/// Implements a Semaphore, i.e., something that gets signalled when something else is ready.
pub struct Semaphore {
    /// The device where the Semaphore lives
    device    : Arc<Device>,
    /// The Semaphore itself
    semaphore : vk::Semaphore,
}

impl Semaphore {
    /// Constructor for the Semaphore.
    /// 
    /// # Arguments
    /// - `device`: The Device where the semaphore will live.
    /// 
    /// # Returns
    /// A new Semaphore instance on success.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend could not create the Semaphore.
    pub fn new(device: Arc<Device>) -> Result<Arc<Self>, Error> {
        // Create the create info
        let semaphore_info = populate_semaphore_info();

        // Create the semaphore on the device
        let semaphore = unsafe {
            match device.create_semaphore(&semaphore_info, None) {
                Ok(semaphore) => semaphore,
                Err(err)      => { return Err(Error::SemaphoreCreateError{ err }); }
            }
        };

        // Done, wrap in an instance and return
        Ok(Arc::new(Self {
            device,
            semaphore,
        }))
    }



    /// Returns the device where this Semaphore lives.
    #[inline]
    pub fn device(&self) -> &Arc<Device> { &self.device }

    /// Returns the internal VkSemaphore.
    #[inline]
    pub fn vk(&self) -> vk::Semaphore { self.semaphore }
}



/// Implements a Fence, i.e., something that the CPU manually has to set to continue.
pub struct Fence {
    /// The device where the Fence lives
    device : Arc<Device>,
    /// The Fence itself
    fence  : vk::Fence,
}

impl Fence {
    /// Constructor for the Fence.
    /// 
    /// # Arguments
    /// - `device`: The Device where the semaphore will live.
    /// - `signalled`: Whether the Fence should already begin in a signalled state or not.
    /// 
    /// # Returns
    /// A new Fence instance on success.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend could not create the Fence.
    pub fn new(device: Arc<Device>, signalled: bool) -> Result<Arc<Self>, Error> {
        // Create the create info with the proper signalled state
        let fence_info = populate_fence_info(if signalled { vk::FenceCreateFlags::SIGNALED } else { vk::FenceCreateFlags::empty() });

        // Create the fence on the device
        let fence = unsafe {
            match device.create_fence(&fence_info, None) {
                Ok(fence) => fence,
                Err(err)  => { return Err(Error::FenceCreateError{ err }); }
            }
        };

        // Done, wrap in an instance and return
        Ok(Arc::new(Self {
            device,
            fence,
        }))
    }



    /// Returns the device where this Semaphore lives.
    #[inline]
    pub fn device(&self) -> &Arc<Device> { &self.device }

    /// Returns the internal VkFence.
    #[inline]
    pub fn vk(&self) -> vk::Fence { self.fence }
}
