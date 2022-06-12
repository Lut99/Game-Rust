/* POOL.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   12 Jun 2022, 13:18:53
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the MemoryPool implementation, which manages general GPU
 *   memory structures.
**/

use std::ptr;
use std::rc::Rc;

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::vec_as_ptr;
use crate::auxillary::{BufferAllocateInfo, DeviceMemoryType, DeviceMemoryTypeFlags, MemoryAllocatorKind, MemoryPropertyFlags};
use crate::device::Device;
use crate::pools::memory::allocators::{DenseAllocator, LinearAllocator, MemoryAllocator};
use crate::pools::memory::buffers::Buffer;


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




/***** AUXILLARY FUNCTIONS *****/
/// Find a suitable VkDeviceMemory and returns it, along with reserved pointer and size for the given requirements.
/// 
/// The memory allocation algorithm used is as follows (Taken from the VMA:
/// https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/general_considerations.html):
///  1. Try to find free range of memory in existing blocks.
///  2. If failed, try to create a new block of VkDeviceMemory, with preferred 
///     block size.
///  3. If failed, try to create such block with size / 2, size / 4, size / 8.
///  // 4. If failed, try to allocate separate VkDeviceMemory for this
///  //   allocation.
///  5. If failed, choose other memory type that meets the requirements
///     specified in VmaAllocationCreateInfo and go to point 1.
///  6. If failed, return out-of-memory error.
/// 
/// # Arguments
/// - `device`: The Device we use to allocate new blocks on.
/// - `pref_block_size`: The preferred block size (in bytes) of new blocks. However, this is just a hint; larger or smaller blocks may be allocated.
/// - `types`: A list of MemoryTypes with already allocated MemoryBlocks in them.
/// - `req_size`: The desired size (in bytes) of the new memory area.
/// - `req_align`: The required memory alignment for the new memory area (within some allocated DeviceMemory).
/// - `req_types`: The desired DeviceMemoryType that we use to filter eventual memory types.
/// - `req_props`: The desired MemoryPropertyFlags that we need supported by the eventual memory type.
/// - `req_kind`: The desired MemoryAllocatorKind to use for the new block. In case of linear allocators, this will commit you to a specific block.
/// 
/// # Returns
/// A tuple with the VkDeviceMemory on `.0` and the start address within that block on `.1`. The reserved memory will be guaranteed to have space for (at least) `req_size` bytes.
/// 
/// # Errors
/// This function will error if there is no suitable memory type (at all), no memory type with enough space or the limit on memory allocations has been exceeded.
fn allocate_memory(
    device: &Rc<Device>,
    pref_block_size: usize,
    types: &mut Vec<MemoryType>,
    req_size: usize,
    req_align: usize,
    req_types: DeviceMemoryTypeFlags,
    req_props: MemoryPropertyFlags,
    req_kind: MemoryAllocatorKind,
) -> Result<(vk::DeviceMemory, usize), Error> {
    // 1: Try to find a free range of memory in existing blocks
    for mtype in types {
        // Skip if the memory type of this block is not any of the required ones
        if !req_types.check(mtype.mtype()) { continue; }
        // Skip if the memory property flags are not present
        if !mtype.mprops().check(req_props) { continue; }

        // Check if any of the nested blocks can allocate the required size
        for block in mtype.blocks_mut() {
            // Skip if not enough total space
            if block.capacity() - block.size() < req_size { continue; }
            // Skip if not the desired allocator
            if block.kind() != req_kind { continue; }

            // Now try to allocate the block with the appropriate allocator
            match block.allocate(req_align, req_size) {
                Ok(pointer)                        => { return Ok((block.mem(), pointer)); },
                Err(Error::OutOfMemoryError{ .. }) => { continue; },
                Err(err)                           => { return Err(err); },
            };
        }

        // 2. & 3. If there are no existing blocks with enough space, attempt to allocate a new one in this area
        for size in [ pref_block_size, pref_block_size / 2, pref_block_size / 4, pref_block_size / 8, req_size ] {
            // Skip if less than the required size
            if size < req_size { continue; }

            // Prepare the allocation info & try to allocate the VkDeviceMemory
            let alloc_info = populate_alloc_info(size as vk::DeviceSize, mtype.mtype().into());
            unsafe {
                match device.allocate_memory(&alloc_info, None) {
                    Ok(device_memory) => {
                        // Wrap it in a new memory block
                        let mut block = match req_kind {
                            MemoryAllocatorKind::Dense      => MemoryBlock::new(device_memory, DenseAllocator::new()),
                            MemoryAllocatorKind::Linear(id) => MemoryBlock::new(device_memory, LinearAllocator::new(id, size))
                        };

                        // Use the block to allocate the requested area, done
                        match block.allocate(req_align, req_size) {
                            Ok(pointer)                        => { return Ok((block.mem(), pointer)); },
                            Err(err)                           => { return Err(err); },
                        };
                    },
                    Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY)   |
                    Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => { continue },
                    Err(err) => { return Err(Error::MemoryAllocateError{ name: device.name().into(), size: req_size, mem_type: mtype.mtype(), err }); }
                }
            };
        }

        // 5. If we have not been able to allocate a new block of the given size, try again for another memory type which may support the requested area
        continue;
    }

    // 6. Unable to find any memory :(
    Err(Error::OutOfMemoryError{ req_size })
}





/***** AUXILLARY STRUCTS *****/
/// Defines a collection of MemoryBlocks belonging to a certain type.
struct MemoryType {
    /// The DeviceMemoryType identifier of this type of memory.
    mtype  : DeviceMemoryType,
    /// The supported properties of this type of memory.
    mprops : MemoryPropertyFlags,

    /// The list of memory blocks allocated in this type.
    blocks : Vec<MemoryBlock>,
}

impl MemoryType {
    /// Constructor for the MemoryType.
    /// 
    /// # Arguments
    /// - `mem_type`: The DeviceMemoryType that this MemoryType represents.
    /// - `mem_props`: The supported MemoryPropertyFlags by this type of memory.
    #[inline]
    fn new(mem_type: DeviceMemoryType, mem_props: MemoryPropertyFlags) -> Self {
        Self {
            mtype  : mem_type,
            mprops : mem_props,

            blocks : Vec::with_capacity(16),
        }
    }



    /// Adds a new MemoryBlock to this type.
    /// 
    /// # Argument
    /// - `block`: The MemoryBlock to register belonging to this memory type.
    #[inline]
    fn add(&mut self, block: MemoryBlock) { self.blocks.push(block) }

    /// Returns a(n immuteable) list of the Blocks in this type.
    #[inline]
    fn blocks(&self) -> &[MemoryBlock] { &self.blocks }

    /// Returns a (muteable) list of the Blocks in this type.
    #[inline]
    fn blocks_mut(&mut self) -> &mut Vec<MemoryBlock> { &mut self.blocks }



    /// Returns the DeviceMemoryType for this MemoryBlock.
    #[inline]
    fn mtype(&self) -> DeviceMemoryType { self.mtype }

    /// Returns the MemoryPropertyFlags for this MemoryBlock.
    #[inline]
    fn mprops(&self) -> MemoryPropertyFlags { self.mprops }
}





/// Defines a single block of continious memory, which may be sub-allocated to provide buffers.
struct MemoryBlock {
    /// The VkDeviceMemory that we wrap.
    memory    : vk::DeviceMemory,
    /// The allocator strategy of this block.
    allocator : Box<dyn MemoryAllocator>,
}

impl MemoryBlock {
    /// Constructor for the MemoryBlock.
    /// 
    /// # Generic types
    /// - `A`: The type of the MemoryAllocator to build this MemoryBlock around.
    /// 
    /// # Arguments
    /// - `memory`: The VkDeviceMemory around which to wrap this block.
    /// - `allocator`: Instance of the Allocator to allocate new memory in this block with.
    #[inline]
    fn new<A: 'static + MemoryAllocator>(memory: vk::DeviceMemory, allocator: A) -> Self {
        Self {
            memory,
            allocator : Box::new(allocator),
        }
    }



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
    fn allocate(&mut self, align: usize, size: usize) -> Result<usize, Error> {
        self.allocator.allocate(align, size)
    }



    /// Returns the memory wrapped by this block.
    #[inline]
    fn mem(&self) -> vk::DeviceMemory { self.memory }

    /// Returns the allocation strategy (and possibly the specific instance of it) for this memory block.
    #[inline]
    fn kind(&self) -> MemoryAllocatorKind {
        self.allocator.kind()
    }

    /// Returns the total space used in this MemoryBlock.
    #[inline]
    fn size(&self) -> usize {
        self.allocator.size()
    }

    /// Returns the total capacity of this MemoryBlock.
    #[inline]
    fn capacity(&self) -> usize {
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
    types           : Vec<MemoryType>,
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
            types           : Vec::with_capacity(n_types),
        })
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

        // Get a piece of memory to allocate
        let (memory, pointer) = allocate_memory(
            &self.device,
            self.pref_block_size,
            &mut self.types,
            buffer_size, buffer_align, buffer_types, info.memory_props, info.allocator
        )?;

        // Bind the buffer to it
        unsafe {
            if let Err(err) = self.device.bind_buffer_memory(buffer, memory, pointer as vk::DeviceSize) {
                return Err(Error::BufferBindError{ err });
            }
        };

        // Nice! Return
        Ok(Rc::new(Buffer {
            buffer,

            usage_flags : info.usage_flags,
            mem_props   : info.memory_props,
            size        : buffer_size,
        }))
    }



    /// Return the parent device of the MemoryPool.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    // /// Returns the block size of each allocated pool of memory.
    // #[inline]
    // pub fn block_size(&self) -> usize { self.block_size }
}
