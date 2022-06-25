/* UTILS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 18:36:50
 * Last edited:
 *   25 Jun 2022, 18:37:54
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains common populate functions and other utilities.
**/

use std::ptr;

use ash::vk;


/***** POPULATE FUNCTIONS *****/
/// Populates the alloc info for a new Buffer memory (VkMemoryAllocateInfo).
/// 
/// # Arguments
/// - `size`: The VkDeviceSize number of bytes to allocate.
/// - `types`: The index of the device memory type that we will allocate on.
#[inline]
pub(crate) fn populate_alloc_info(size: vk::DeviceSize, types: u32) -> vk::MemoryAllocateInfo {
    vk::MemoryAllocateInfo {
        // Set the standard stuff
        s_type : vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next : ptr::null(),

        // Set the size & memory type
        allocation_size   : size,
        memory_type_index : types,
    }
}
