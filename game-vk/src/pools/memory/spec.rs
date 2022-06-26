/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   26 Jun 2022, 14:15:04
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and definitions for the MemoryPools.
**/

use std::rc::Rc;

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{MemoryPropertyFlags, MemoryRequirements};
use crate::device::Device;


/***** LIBRARY *****/
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
    fn allocate(&mut self, reqs: &MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, usize), Error>;

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



    /// Returns the device of the pool.
    fn device(&self) -> &Rc<Device>;

    /// Returns the used space in the pool.
    fn size(&self) -> usize;

    /// Returns the total space in the pool.
    fn capacity(&self) -> usize;
}
