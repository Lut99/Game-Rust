/* POOL.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   29 May 2022, 17:44:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the MemoryPool implementation, which manages general GPU
 *   memory structures.
 * 
 *   The memory allocation algorithm used is as follows (Taken from the VMA:
 *   https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/general_considerations.html):
 *    1. Try to find free range of memory in existing blocks.
 *    2. If failed, try to create a new block of VkDeviceMemory, with preferred 
 *       block size.
 *    3. If failed, try to create such block with size / 2, size / 4, size / 8.
 *    4. If failed, try to allocate separate VkDeviceMemory for this
 *       allocation.
 *    5. If failed, choose other memory type that meets the requirements
 *       specified in VmaAllocationCreateInfo and go to point 1.
 *    6. If failed, return out-of-memory error.
**/

use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocationCreateDesc, AllocatorCreateDesc};

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::vec_as_ptr;
use crate::auxillary::{BufferUsageFlags, DeviceMemoryTypeFlags, MemoryPropertyFlags};
use crate::device::Device;
use crate::pools::memory::buffers::Buffer;


/***** POPULATE FUNCTIONS *****/
/// Populates the create info for a new Buffer (VkBufferCreateInfo).
/// 
/// # Arguments
/// - `usage_flags`: The VkBufferUsageFlags that determine how to use this buffer.
/// - `sharing_mode`: The VkSharingMode value that determines who can access this buffer.
/// - `queue_families`: If `sharing_mode` is `VkSharingMode::CONCURRENT`, then this list specifies the queue families who may access the buffer.
/// - `size`: The requested size (in bytes) of the Buffer. This may not be the actual size.
#[inline]
fn populate_buffer_info(usage_flags: vk::BufferUsageFlags, sharing_mode: vk::SharingMode, queue_families: &[u32], size: vk::DeviceSize) -> vk::BufferCreateInfo {
    vk::BufferCreateInfo {
        // Set the standard stuff
        s_type : vk::StructureType::BUFFER_CREATE_INFO,
        p_next : ptr::null(),
        flags  : vk::BufferCreateFlags::empty(),

        // Set the usage flags
        usage : usage_flags,

        // Set the sharing mode (and eventual queue families)
        sharing_mode,
        queue_family_index_count : queue_families.len() as u32,
        p_queue_family_indices   : vec_as_ptr!(queue_families),

        // Finally, set the size
        size,
    }
}





/***** LIBRARY *****/
/// The CommandPool defines a Pool for command buffers.
pub struct MemoryPool {
    /// The parent GPU where the pools will be allocated.
    device    : Rc<Device>,
    /// The memory-type mapped 
    allocator : Rc<Allocator>,
}

impl MemoryPool {
    /// Constructor for the MemoryPool.
    /// 
    /// # Arguments
    /// - `device`: The parent Device where all the memory will be allocated.
    /// - `block_size`: The size of each continious block of memory.
    pub fn new(device: Rc<Device>, block_size: usize) -> Result<Rc<Self>, Error> {
        // Create a new allocator
        let allocator = match Allocator::new(&AllocatorCreateDesc {
            // Pass the instance and device to allocate stuff on
            instance        : device.instance().vk().clone(),
            device          : device.ash().clone(),
            physical_device : device.physical_device(),

            // Set the debug settings
            debug_settings        : Default::default(),
            buffer_device_address : true,
        }) {
            Ok(allocator) => Rc::new(allocator),
            Err(err)      => { return Err(Error::AllocatorCreateError{ err }); }
        };

        // Wrap it in the struct and return
        Ok(Rc::new(Self {
            device,
            allocator,
        }))
    }



    /// Given a set of memory requirements & desired properties, returns the associated DeviceMemoryTypeFlags.
    /// 
    /// # Arguments
    /// - `requirements`: A filter of DeviceMemoryTypeFlags that contain the possible types which we allow according to the specific Buffer.
    /// - `properties`: The MemoryPropertyFlags that list desired properties from our resulting memory type.
    /// 
    /// # Returns
    /// The most suited DeviceMemoryTypeFlags according to the MemoryPool. If it returns None, then no memory type was found with all of the requirements.
    pub fn select_device_type(&self, requirements: DeviceMemoryTypeFlags, properties: MemoryPropertyFlags) -> Option<DeviceMemoryTypeFlags> {
        // Get the properties of the parent GPU
        let device_props: vk::PhysicalDeviceMemoryProperties = unsafe { self.device.instance().vk().get_physical_device_memory_properties(self.device.physical_device()) };

        // Iterate through the available memory types
        for (i, memory_type) in device_props.memory_types.iter().enumerate() {
            // Skip this type if not in the requirements
            let memory_props: DeviceMemoryTypeFlags = (i as u32).into();
            if !requirements.check(memory_props) { continue; }

            // Make sure that the property flags are present
            if !MemoryPropertyFlags::from(memory_type.property_flags).check(properties) { continue; }

            // Alright, we'll take it
            return Some(memory_props);
        }

        // No supported queue found
        None
    }



    // /// Allocates a new buffer in the MemoryPool.
    // /// 
    // /// The memory type will automatically be deduced based on the given buffer usage flags and memory property flags. Note that the actual size of the buffer may be padded if needed according to the memory type.
    // /// 
    // /// If no memory of that type has been allocated yet, the MemoryPool will attempt to do so.
    // /// 
    // /// # Arguments
    // /// - `name`: A debug name that may be used to distinguish allocation calls.
    // /// 
    // /// # Returns
    // /// A new Buffer object that represents some allocated chunk of memory.
    // /// 
    // /// # Errosr
    // /// This function may error if we fail to allocate a new piece of pool memory or if not enough space is left.
    // pub fn allocate_buf(&mut self, name: &str, usage_flags: BufferUsageFlags, memory_properties: MemoryPropertyFlags, size: usize) -> Result<Rc<Buffer>, Error> {
    //     // First, create a new Buffer object from the usage flags
    //     let buffer_info = populate_buffer_info(
    //         usage_flags.into(),
    //         vk::SharingMode::EXCLUSIVE, vec![],
    //         size as vk::DeviceSize,
    //     );

    //     // Create the Buffer
    //     let buffer: vk::Buffer = unsafe {
    //         match self.device.create_buffer(&buffer_info, None) {
    //             Ok(buffer) => buffer,
    //             Err(err)   => { return Err(Error::BufferCreateError{ err }); }
    //         }
    //     };

    //     // Now prepare the allocation info for this Buffer type
    //     let buffer_requirements: vk::MemoryRequirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };
    //     let alloc_info = self.allocator.allocate(&AllocationCreateDesc {
    //         name,
    //         requirements: buffer_requirements,
    //         location: location.into(),
    //         linear: true,
    //     });

    //     // Now choose the memory type based on this buffer's requirements
    //     let memory_type: DeviceMemoryTypeFlags = match self.select_device_type(buffer_requirements.memory_type_bits.into(), memory_properties) {
    //         Some(memory_type) => memory_type,
    //         None              => { return Err(Error::UnsupportedMemoryRequirements{ name: self.device.name().to_string(), types: buffer_requirements.memory_type_bits.into(), props: memory_properties }); }
    //     };

    //     // With the memory type chosen, see if we have a pool memory available
    //     let pool: &mut DeviceMemory = match self.pools.get_mut(&memory_type) {
    //         Some(pool) => pool,
    //         None       => {
    //             // Try to allocate a new one
    //             self.pools.insert(memory_type, DeviceMemory::new(memory_type, self.block_size)?);

    //             // Return it
    //             self.pools.get_mut(&memory_type).unwrap()
    //         }
    //     };
    // }



    /// Return the parent device of the MemoryPool.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    // /// Returns the block size of each allocated pool of memory.
    // #[inline]
    // pub fn block_size(&self) -> usize { self.block_size }
}
