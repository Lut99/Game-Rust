/* POOL.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   28 May 2022, 17:24:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the MemoryPool implementation, which manages general GPU
 *   memory structures.
**/

use std::collections::HashMap;
use std::rc::Rc;

use crate::auxillary::{DeviceMemoryType, MemoryPropertyFlags};


/***** HELPER STRUCTS *****/
/// A continiously allocated block of memory.
struct DeviceMemory {
    /// The supported memory properties by this block of memory.
    properties : MemoryPropertyFlags,
}





/***** LIBRARY *****/
/// The CommandPool defines a Pool for command buffers.
pub struct MemoryPool {
    /// Maps memory types to allocated memory object.
    pools : HashMap<DeviceMemoryType, DeviceMemory>,
}

impl MemoryPool {
    /// Constructor for the MemoryPool.
    #[inline]
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            pools : HashMap::with_capacity(4),
        })
    }
}
