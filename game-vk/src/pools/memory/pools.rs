/* POOLS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 18:04:08
 * Last edited:
 *   25 Jun 2022, 18:41:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The pools that we use to allocate new bits of memory.
**/

use std::rc::Rc;
use std::slice;
use std::sync::{Arc, RwLock};

use ash::vk;
use log::warn;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{DeviceMemoryTypeFlags, MemoryPropertyFlags};
use crate::device::Device;
use crate::pools::memory::utils::populate_alloc_info;
use crate::pools::memory::spec::{MemoryBlock, MemoryPool};


/***** LIBRARY *****/
/// A LinearPool uses a very fast memory allocation algorithm, but wastes space because freed blocks cannot be re-used until the pool is reset. Additionally, this type of pool only supports one type of memory.
pub struct LinearPool {
    /// The Device where the LinearPool lives.
    device : Rc<Device>,
    /// The single memory block used in the linear pool.
    block  : Option<MemoryBlock>,

    /// The pointer that determines up to where we already gave to memory blocks.
    pointer  : usize,
    /// The size (in bytes) of the LinearPool.
    capacity : usize,
}

impl LinearPool {
    /// Constructor for the LinearPool.
    /// 
    /// Note that memory will be allocated lazily.
    /// 
    /// # Arguments
    /// - `capacity`: The size (in bytes) of the pool.
    /// 
    /// # Returns
    /// A new LinearPool instance, already wrapped in an Arc and a RwLock.
    #[inline]
    pub fn new(device: Rc<Device>, capacity: usize) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            device,
            block : None,

            pointer : 0,
            capacity,
        }))
    }



    /// Returns the used size in the LinearPool.
    #[inline]
    pub fn size(&self) -> usize { self.pointer }

    /// Returns the total size of the LinearPool.
    #[inline]
    pub fn capacity(&self) -> usize { self.capacity }
}

impl MemoryPool for LinearPool {
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
    fn allocate(&mut self, reqs: vk::MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, usize), Error> {
        // Check whether we have a block of memory already
        let new_req: DeviceMemoryTypeFlags = reqs.memory_type_bits.into();
        match self.block.as_ref() {
            Some(block) => {
                // Make sure the requirements & properties are satisfied
                if !new_req.check(block.mem_type) { panic!("LinearAllocator is allocated for device memory type {}, but new allocation only supports {}", block.mem_type, DeviceMemoryTypeFlags::from(reqs.memory_type_bits)); }
                if !block.mem_props.check(props) { panic!("LinearAllocator is allocated for device memory type {} which supports the properties {}, but new allocation requires {}", block.mem_type, block.mem_props, props); }
            },

            None => {
                // Attempt to find a suitable memory type for the given requirements & properties
                let device_props : vk::PhysicalDeviceMemoryProperties = unsafe { self.device.instance().get_physical_device_memory_properties(self.device.physical_device()) };
                let device_types : &[vk::MemoryType] = unsafe { slice::from_raw_parts(device_props.memory_types.as_ptr(), device_props.memory_type_count as usize) };
                for i in 0..device_types.len() {
                    // Check if this type is in the required ones
                    if !new_req.check(i as u32) { continue; }
                    // Check if this type satisfies the properties
                    if !MemoryPropertyFlags::from(device_types[i].property_flags).check(props) { continue; }

                    // Populate the memory info
                    let alloc_info: vk::MemoryAllocateInfo = populate_alloc_info(
                        reqs.size,
                        i as u32,
                    );

                    // Now attempt to allocate a suitably large enough block
                    let memory: vk::DeviceMemory = unsafe {
                        match self.device.allocate_memory(&alloc_info, None) {
                            Ok(memory) => memory,
                            Err(err)   => { continue; }
                        }
                    };

                    
                }
            },
        }

        // // Compute the alignment requirements based on the current pointer
        // let pointer = if reqs.alignment != 0 {
        //     if (reqs.alignment & (reqs.alignment - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", reqs.alignment); }
        //     (self.pointer + (reqs.alignment - 1)) & ((!reqs.alignment) + 1)
        // } else {
        //     self.pointer
        // };

        // Done
        Ok(())
    }



    /// Frees an allocated bit of memory.
    /// 
    /// Note that not all types of pools may actually do anything with this. A LinearPool, for example, might deallocate but will never re-use that memory until reset anyway.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer to the block that was allocated.
    /// 
    /// # Panics
    /// This function may panic if the given pointer was never allocated with this pool.
    fn free(&mut self, _pointer: usize) { warn!("LinearAllocator::free() called, which has no effect."); }



    /// Resets the memory pool back to its initial, empty state.
    #[inline]
    fn reset(&mut self) { self.pointer = 0; }
}
