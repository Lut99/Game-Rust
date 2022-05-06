/* QUEUE.rs
 *   by Lut99
 *
 * Created:
 *   06 May 2022, 18:28:29
 * Last edited:
 *   06 May 2022, 18:29:11
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the Queue object, which wraps around a device queue.
**/

use std::ptr;
use std::rc::Rc;

use ash::vk;

pub use crate::errors::QueueError as Error;
use crate::device::Device;


/***** LIBRARY *****/
/// The Queue struct wraps around a Device Queue to submit easily.
pub struct Queue {
    /// The parent Device.
    device : Rc<Device>,
    /// The Queue object to wrap.
    queue  : vk::Queue,
}

impl Queue {
    /// Submits the given command buffer to this queue.
    pub fn submit(&self) -> Result<(), Error> {
        
    }



    /// Returns the parent Device.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the internal VkQueue object.
    #[inline]
    pub fn vk(&self) -> vk::Queue { self.queue }
}
