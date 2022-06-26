/* BLOCK.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:18:26
 * Last edited:
 *   26 Jun 2022, 13:21:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines a single MemoryBlock, i.e., a wrapper around a vk::DeviceMemory
 *   block.
**/

use std::ptr;
use std::rc::Rc;
use std::slice;

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{DeviceMemoryType, MemoryPropertyFlags, MemoryRequirements};
use crate::device::Device;


/***** POPULATE FUNCTIONS *****/
/// Populates the alloc info for a new Buffer memory (VkMemoryAllocateInfo).
/// 
/// # Arguments
/// - `size`: The VkDeviceSize number of bytes to allocate.
/// - `types`: The index of the device memory type that we will allocate on.
#[inline]
fn populate_alloc_info(size: vk::DeviceSize, types: u32) -> vk::MemoryAllocateInfo {
    vk::MemoryAllocateInfo {
        // Set the standard stuff
        s_type : vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next : ptr::null(),

        // Set the size & memory type
        allocation_size   : size,
        memory_type_index : types,
    }
}





/***** LIBRARY *****/
/// Defines a single, continious block of memory that lives on a single type of memory on the Device.
pub struct MemoryBlock {
    /// The Device where the block lives.
    device : Rc<Device>,

    /// The VkDeviceMemory that is actually represented by this block.
    mem       : vk::DeviceMemory,
    /// The memory type for this block.
    mem_type  : DeviceMemoryType,
    /// The properties supported by this block.
    mem_props : MemoryPropertyFlags,
    /// The size (in bytes) of this block.
    mem_size  : usize,
}

impl MemoryBlock {
    /// Factory method for the MemoryBlock, which allocates a new vk::DeviceMemory with the given requirements and properties.
    /// 
    /// # Arguments
    /// 
    /// # Returns
    /// A new MemoryBlock.
    /// 
    /// # Errors
    /// This function may error if we could not find a suitable memory type or there was no memory left.
    pub fn allocate(device: Rc<Device>, reqs: &MemoryRequirements, props: MemoryPropertyFlags) -> Result<Self, Error> {
        // Attempt to find a suitable memory type for the given requirements & properties
        let mut found_candidate = false;
        let device_props : vk::PhysicalDeviceMemoryProperties = unsafe { device.instance().get_physical_device_memory_properties(device.physical_device()) };
        let device_types : &[vk::MemoryType] = unsafe { slice::from_raw_parts(device_props.memory_types.as_ptr(), device_props.memory_type_count as usize) };
        for i in 0..device_types.len() {
            // Check if this type is in the required ones
            if !reqs.types.check(i as u32) { continue; }
            // Check if this type satisfies the properties
            let mem_props = MemoryPropertyFlags::from(device_types[i].property_flags);
            if !mem_props.check(props) { continue; }
            found_candidate = true;

            // Populate the memory info
            let alloc_info: vk::MemoryAllocateInfo = populate_alloc_info(
                reqs.size as vk::DeviceSize,
                i as u32,
            );

            // Now attempt to allocate a suitably large enough block
            let memory: vk::DeviceMemory = unsafe {
                match device.allocate_memory(&alloc_info, None) {
                    Ok(memory) => memory,

                    // Try the next block if out of memory
                    Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY)   |
                    Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => { continue; },

                    // Otherwise, throw error
                    Err(err)   => { return Err(Error::MemoryAllocateError{ name: device.name().into(), size: reqs.size, mem_type: (i as u32).into(), err }); }
                }
            };

            // Wrap it in a block and we're done
            return Ok(MemoryBlock {
                device,

                mem       : memory,
                mem_type  : (i as u32).into(),
                mem_props,
                mem_size  : reqs.size,
            });
        }

        // Failed to find a suitable block; determine the exact error to throw
        match found_candidate {
            true  => Err(Error::OutOfMemoryError{ req_size: reqs.size }),
            false => Err(Error::UnsupportedMemoryRequirements{ name: device.name().into(), types: reqs.types, props }),
        }
    }





    /// Returns the physical vk::DeviceMemory which this block wraps.
    #[inline]
    pub fn vk(&self) -> vk::DeviceMemory { self.mem }

    /// Returns the DeviceMemoryType where this block lives.
    #[inline]
    pub fn mem_type(&self) -> DeviceMemoryType { self.mem_type }

    /// Returns the MemoryPropertyFlags that describe the properties supported by this block.
    #[inline]
    pub fn mem_props(&self) -> MemoryPropertyFlags{ self.mem_props }

    /// Returns the size of the allocated block (in bytes).
    #[inline]
    pub fn mem_size(&self) -> usize{ self.mem_size }
}

impl Drop for MemoryBlock {
    #[inline]
    fn drop(&mut self) {
        // Deallocate the device memory
        unsafe { self.device.free_memory(self.mem, None); }
    }
}
