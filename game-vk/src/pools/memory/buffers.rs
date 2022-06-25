/* BUFFERS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:17:19
 * Last edited:
 *   25 Jun 2022, 18:03:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines buffers that are used in the MemoryPool.
**/

use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::vec_as_ptr;
use crate::auxillary::{BufferUsageFlags, MemoryPropertyFlags, SharingMode};
use crate::device::Device;
use crate::pools::memory::spec::MemoryPool;


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
/// An allocated piece of memory in the MemoryPool.
pub struct Buffer {
    /// The Device where the Buffer lives.
    device : Rc<Device>,

    /// The VkBuffer object we wrap.
    buffer  : vk::Buffer,
    /// The bound memory area for this buffer.
    /// 
    /// # Layout
    /// - `0`: The MemoryPool where this memory area was allocated.
    /// - `1`: The block of device memory itself.
    /// - `2`: The offset of the device memory (used for deallocation).
    memory  : Option<(Arc<RwLock<dyn MemoryPool>>, vk::DeviceMemory, usize)>,

    /// The usage flags for this Buffer.
    usage_flags  : BufferUsageFlags,
    /// The sharing mode that determines which queue families have access to this Buffer.
    sharing_mode : SharingMode,
    /// The memory requirements of this Buffer.
    mem_req      : vk::MemoryRequirements,
    /// The memory properties of the memory backing this Buffer.
    mem_props    : MemoryPropertyFlags,
    /// The size (in bytes) of this Buffer.
    size         : usize,
}

impl Buffer {
    /// Constructor for the Buffer.
    /// 
    /// # Arguments
    /// - `usage_flags`: The BufferUsageFlags that determine how this buffer will/may be used.
    /// - `mem_props`: Any memory properties of this Buffer. Used when deciding how to allocate.
    /// - `size`: The size of the buffer, in bytes. The actually allocated size may be larger due to alignment etc.
    /// 
    /// # Errors
    /// This function may error if the buffer creation in the Vulkan backend failed.
    pub fn new(device: Rc<Device>, usage_flags: BufferUsageFlags, sharing_mode: SharingMode, mem_props: MemoryPropertyFlags, size: usize) -> Result<Rc<Self>, Error> {
        // Split the sharing mode
        let (vk_sharing_mode, vk_queue_family_indices) = sharing_mode.clone().into();

        // First, create a new Buffer object from the usage flags
        let buffer_info = populate_buffer_info(
            usage_flags.into(),
            vk_sharing_mode, &vk_queue_family_indices.unwrap_or(Vec::new()),
            size as vk::DeviceSize,
        );

        // Create the Buffer
        let buffer: vk::Buffer = unsafe {
            match device.create_buffer(&buffer_info, None) {
                Ok(buffer) => buffer,
                Err(err)   => { return Err(Error::BufferCreateError{ err }); }
            }
        };

        // Get the buffer memory type requirements
        let requirements: vk::MemoryRequirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        // For now, we leave it at this; return the buffer
        Ok(Rc::new(Self {
            device,
            pool : None,

            buffer,
            memory : None,

            usage_flags,
            sharing_mode,
            mem_req : requirements,
            mem_props,
            size,
        }))
    }



    /// Allocates a new piece of memory on the given pool and binds it to the internal Buffer.
    /// 
    /// Note that the Vulkan spec does not seem to require that the bound memory lives on the same GPU as this, so for now, this is not a requirement.
    /// 
    /// # Arguments
    /// - `pool`: A MemoryPool that we use to allocate the new memory for this Buffer.
    /// 
    /// # Results
    /// Nothing explicitly, but does set the memory area for this Buffer. Can override an already existing area, which will be deallocated.
    /// 
    /// # Errors
    /// This function errors if either the new memory could not be reserved or it could not be bound.
    pub fn bind(&mut self, pool: Arc<RwLock<dyn MemoryPool>>) -> Result<(), Error> {
        // If present, deallocate old area first
        if let Some((pool, _, pointer)) = self.memory.take() {
            // Get a lock first
            let mut lock: RwLockWriteGuard<_> = match pool.write() {
                Ok(lock) => lock,
                Err(err) => { return Err(Error::PoolLockError{ err: format!("{}", err) }); }
            };

            // Free the area
            lock.free(pointer);
        }

        // Allocate some bit in the pool
        let (memory, pointer): (vk::DeviceMemory, usize) = {
            // Get a lock first
            let mut lock: RwLockWriteGuard<_> = match pool.write() {
                Ok(lock) => lock,
                Err(err) => { return Err(Error::PoolLockError{ err: format!("{}", err) }); }
            };

            // Reserve the area
            lock.allocate(self.mem_req, self.mem_props)?
        };

        // Bind the memory
        unsafe {
            if let Err(err) = self.device.bind_buffer_memory(self.buffer, memory, pointer as vk::DeviceSize) {
                return Err(Error::BufferBindError{ err });
            }
        };

        // Update the internal memory area and pool
        self.memory = Some((pool, memory, pointer));
        Ok(())
    }



    
}
