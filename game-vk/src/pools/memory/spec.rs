/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   25 Jun 2022, 18:11:58
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and definitions for the MemoryPools.
**/

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{DeviceMemoryType, MemoryPropertyFlags};


/***** LIBRARY *****/
/// Defines a single, continious block of memory that lives on a single type of memory on the Device.
pub(crate) struct MemoryBlock {
    /// The VkDeviceMemory that is actually represented by this block.
    pub(crate) mem       : vk::DeviceMemory,
    /// The memory type for this block.
    pub(crate) mem_type  : DeviceMemoryType,
    /// The properties supported by this block.
    pub(crate) mem_props : MemoryPropertyFlags,
    /// The size (in bytes) of this block.
    pub(crate) mem_size  : usize,
}





/// The MemoryPool trait which we use to define common access to a MemoryPool.
pub trait MemoryPool {
    /// Returns a newly allocated area of (at least) the requested size.
    /// 
    /// # Arguments
    /// - `reqs`: The memory requirements of the new memory block.
    /// - `props`: Any desired memory properties for this memory block.
    /// 
    /// # Returns
    /// A tuple with the VkDeviceMemory where the new block of memory is allocated on `.0`, and the index in this memory block on `.1`.
    /// 
    /// # Errors
    /// This function errors if the MemoryPool failed to allocate new memory.
    fn allocate(&mut self, reqs: vk::MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, usize), Error>;

    /// Frees an allocated bit of memory.
    /// 
    /// Note that not all types of pools may actually do anything with this. A LinearPool, for example, might deallocate but will never re-use that memory until reset anyway.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer to the block that was allocated.
    /// 
    /// # Panics
    /// This function may panic if the given pointer was never allocated with this pool.
    fn free(&mut self, pointer: usize);

    /// Resets the memory pool back to its initial, empty state.
    fn reset(&mut self);
}
