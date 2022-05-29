/* BUFFERS.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:11:03
 * Last edited:
 *   29 May 2022, 15:54:12
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines buffers that are used in the MemoryPool.
**/

use crate::auxillary::{BufferUsageFlags, MemoryPropertyFlags};


/***** LIBRARY *****/
/// An allocated piece of memory in the MemoryPool.
pub struct Buffer {
    /// The usage flags for this Buffer.
    usage_flags : BufferUsageFlags,
    /// The memory properties of the memory backing this Buffer.
    mem_props   : MemoryPropertyFlags,
    /// The size (in bytes) of this Buffer.
    size        : usize,
}
