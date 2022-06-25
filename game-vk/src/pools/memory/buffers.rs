/* BUFFERS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:17:19
 * Last edited:
 *   25 Jun 2022, 16:17:26
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines buffers that are used in the MemoryPool.
**/

use ash::vk;

use crate::auxillary::{BufferUsageFlags, MemoryPropertyFlags};


/***** LIBRARY *****/
/// An allocated piece of memory in the MemoryPool.
pub struct Buffer {
    /// The VkBuffer object we wrap.
    pub(crate) buffer : vk::Buffer,

    /// The usage flags for this Buffer.
    pub(crate) usage_flags : BufferUsageFlags,
    /// The memory properties of the memory backing this Buffer.
    pub(crate) mem_props   : MemoryPropertyFlags,
    /// The size (in bytes) of this Buffer.
    pub(crate) size        : usize,
}
