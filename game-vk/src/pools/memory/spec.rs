/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   25 Jun 2022, 16:31:04
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and public definitions for the MemoryPools.
**/

use std::error;


/***** LIBRARY *****/
/// The private part of the generic MemoryPool trait.
pub(crate) trait _MemoryPool {
    /// Function that determines a new pointer for new allocations.
    /// 
    /// # Arguments
    /// - `align`: The alignment required for the new block. Should be a power of 2.
    /// - `size`: The size of the new memory block.
    /// 
    /// # Returns
    /// The new pointer for this block.
    /// 
    /// # Errors
    /// This function may error if there is not enough (continious) space available in the pool. It may also panic if the alignment is not a power of 2.
    fn allocate_memory(&mut self, align: usize, size: usize) -> Result<usize, Box<dyn error::Error>>;
}



/// The generic MemoryPool trait, which is used to interact with the generic parts between memory pools with common allocators.
pub trait MemoryPool: _MemoryPool {
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
        let (memory, pointer) = self.allocate_memory(
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
}
