/* POOL.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   04 Jun 2022, 16:06:33
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
use crate::auxillary::{BufferAllocateInfo, DeviceMemoryTypeFlags, MemoryAllocatorKind, MemoryPropertyFlags};
use crate::device::Device;
use crate::pools::memory::allocators::MemoryAllocator;
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




/***** AUXILLARY STRUCTS *****/
/// Defines a single block of continious memory, which may be sub-allocated to provide buffers.
struct MemoryBlock {
    /// The allocator strategy of this block.
    allocator : Box<dyn MemoryAllocator>,
}

impl MemoryBlock {
    /// Allocates a new chunk of continious memory using the internal allocation strategy.
    /// 
    /// # Arguments
    /// - `align`: The alignment (in bytes) of the memory block.
    /// - `size`: The size (in bytes) of the memory block to allocate.
    /// 
    /// # Returns
    /// A pointer in the internal block of memory.
    /// 
    /// # Errors
    /// This function may error if there is no large enough continious block of memory available for the given alignment + size request.
    #[inline]
    pub fn allocate(&mut self, align: usize, size: usize) -> Result<usize, Error> {
        self.allocator.allocate(align, size)
    }



    /// Returns the allocation strategy (and possibly the specific instance of it) for this memory block.
    #[inline]
    pub fn kind(&self) -> MemoryAllocatorKind {
        self.allocator.kind()
    }

    /// Returns the total space used in this MemoryBlock.
    #[inline]
    pub fn size(&self) -> usize {
        self.allocator.size()
    }

    /// Returns the total capacity of this MemoryBlock.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.allocator.capacity()
    }
}





/***** LIBRARY *****/
/// The CommandPool defines a Pool for command buffers.
pub struct MemoryPool {
    /// The parent GPU where the pools will be allocated.
    device       : Rc<Device>,
    /// The memory properties of the parent physical device.
    device_props : vk::PhysicalDeviceMemoryProperties,

    /// The preferred block size
    pref_block_size : usize,
    /// The memory-type mapped blocks of memory.
    blocks          : HashMap<DeviceMemoryTypeFlags, Vec<MemoryBlock>>,
}

impl MemoryPool {
    /// Constructor for the MemoryPool.
    /// 
    /// # Arguments
    /// - `device`: The parent Device where all the memory will be allocated.
    /// - `block_size`: The preferred size of each continious block of memory. Blocks may be larger, if a larger buffer is requested, or smaller, if there is no more space left for such a large block.
    pub fn new(device: Rc<Device>, block_size: usize) -> Rc<Self> {
        // Get the number of types for this device
        let device_props = unsafe { device.instance().vk().get_physical_device_memory_properties(device.physical_device()) };
        let n_types      = device_props.memory_types.len();

        // Done, return as a struct
        Rc::new(Self {
            device,
            device_props,

            pref_block_size : block_size,
            blocks          : HashMap::with_capacity(n_types),
        })
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



    /// Allocates a new buffer in the MemoryPool.
    /// 
    /// The memory type will automatically be deduced based on the given buffer usage flags and memory property flags. Note that the actual size of the buffer may be padded if needed according to the memory type.
    /// 
    /// If no memory of that type has been allocated yet, the MemoryPool will attempt to do so.
    /// 
    /// # Arguments
    /// - `name`: A debug name that may be used to distinguish allocation calls.
    /// 
    /// # Returns
    /// A new Buffer object that represents some allocated chunk of memory.
    /// 
    /// # Errosr
    /// This function may error if we fail to allocate a new piece of pool memory or if not enough space is left.
    pub fn allocate_buf(&mut self, info: BufferAllocateInfo) -> Result<Rc<Buffer>, Error> {
        // Split the sharing mode
        let (vk_sharing_mode, vk_queue_family_indices) = info.sharing_mode.into();

        // First, create a new Buffer object from the usage flags
        let buffer_info = populate_buffer_info(
            info.usage_flags.into(),
            vk_sharing_mode, &vk_queue_family_indices.unwrap_or(Vec::new()),
            info.size as vk::DeviceSize,
        );

        // Create the Buffer
        let buffer: vk::Buffer = unsafe {
            match self.device.create_buffer(&buffer_info, None) {
                Ok(buffer) => buffer,
                Err(err)   => { return Err(Error::BufferCreateError{ err }); }
            }
        };

        // Get the buffer memory type requirements
        let buffer_requirements: vk::MemoryRequirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };
        let buffer_align : usize                 = buffer_requirements.alignment as usize;
        let buffer_size  : usize                 = buffer_requirements.size as usize;
        let buffer_types : DeviceMemoryTypeFlags = buffer_requirements.memory_type_bits.into();

        // Now iterate over all memory types to find a suitable one
        for (i, memory_type) in self.device_props.memory_types.iter().enumerate() {
            // Skip if this type is not one of the required ones
            let memory_props: DeviceMemoryTypeFlags = (i as u32).into();
            if !memory_props.check(buffer_types) { continue; }

            // Skip if the memory property flags are not present
            let type_props: MemoryPropertyFlags = memory_type.property_flags.into();
            if !type_props.check(info.memory_props) { continue; }

            // Now check to see if any blocks have already been allocated
            let pointer = match self.blocks.get(&memory_props) {
                Some(blocks) => {
                    // Try to allocate it in *some* block of this type
                    let mut pointer = usize::MAX;
                    for block in blocks {
                        // Skip if not enough total space
                        if block.capacity() < buffer_size { continue; }
                        // Skip if not the desired allocator
                        if block.kind() != info.allocator { continue; }

                        // Now try to allocate the block with the appropriate allocator
                        pointer = match block.allocate(buffer_align, buffer_size) {
                            Ok(pointer)                        => pointer,
                            Err(Error::OutOfMemoryError{ .. }) => { continue; },
                            Err(err)                           => { return Err(err); },
                        };

                        // Done
                        break;
                    }

                    // Return it
                    pointer
                },
                None => usize::MAX,
            };

            // If no suitable block has been found, attempt to allocate a new one
            if pointer == usize::MAX {
                
            }
        }

        // Now prepare the allocation info for this Buffer type
        let alloc_info = self.allocator.allocate(&AllocationCreateDesc {
            name,
            requirements: buffer_requirements,
            location: location.into(),
            linear: true,
        });

        // Now choose the memory type based on this buffer's requirements
        let memory_type: DeviceMemoryTypeFlags = match self.select_device_type(buffer_requirements.memory_type_bits.into(), memory_properties) {
            Some(memory_type) => memory_type,
            None              => { return Err(Error::UnsupportedMemoryRequirements{ name: self.device.name().to_string(), types: buffer_requirements.memory_type_bits.into(), props: memory_properties }); }
        };

        // With the memory type chosen, see if we have a pool memory available
        let pool: &mut DeviceMemory = match self.pools.get_mut(&memory_type) {
            Some(pool) => pool,
            None       => {
                // Try to allocate a new one
                self.pools.insert(memory_type, DeviceMemory::new(memory_type, self.block_size)?);

                // Return it
                self.pools.get_mut(&memory_type).unwrap()
            }
        };
    }



    /// Return the parent device of the MemoryPool.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    // /// Returns the block size of each allocated pool of memory.
    // #[inline]
    // pub fn block_size(&self) -> usize { self.block_size }
}
