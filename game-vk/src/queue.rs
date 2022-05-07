/* QUEUE.rs
 *   by Lut99
 *
 * Created:
 *   06 May 2022, 18:28:29
 * Last edited:
 *   07 May 2022, 18:16:52
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
use crate::auxillary::PipelineStage;
use crate::device::Device;
use crate::pools::command::Buffer as CommandBuffer;
use crate::sync::{Fence, Semaphore};


/***** POPULATE FUNCTIONS *****/
/// Populates a VkSubmitInfo struct.
/// 
/// # Arguments:
/// - `command_buffer`: The CommandBuffers to submit.
/// - `wait_semaphores`: The Semaphores to wait for before rendering.
/// - `wait_stage_mask`: A list of PipelineStages where each semaphore waiting should occur.
/// - `done_semaphores`: The Semaphores to signal when done with rendering.
fn populate_submit_info(command_buffers: &[vk::CommandBuffer], wait_semaphores: &[vk::Semaphore], wait_stages: &[vk::PipelineStageFlags], done_semaphores: &[vk::Semaphore]) -> vk::SubmitInfo {
    // Do a few sanity checks
    if wait_semaphores.len() != wait_stages.len() { panic!("The length of the Semaphores (wait_semaphores) and associated waiting stages (wait_stages) should be the same"); }

    // Populate hte struct
    vk::SubmitInfo {
        // Do the standard stuff
        s_type : vk::StructureType::SUBMIT_INFO,
        p_next : ptr::null(),

        // Set the command buffers to submit
        command_buffer_count : command_buffers.len() as u32,
        p_command_buffers    : command_buffers.as_ptr(),

        // Set the semaphores to wait for
        wait_semaphore_count  : wait_semaphores.len() as u32,
        p_wait_semaphores     : wait_semaphores.as_ptr(),
        p_wait_dst_stage_mask : wait_stages.as_ptr(),
        
        // Set the semaphores to signal
        signal_semaphore_count : done_semaphores.len() as u32,
        p_signal_semaphores    : done_semaphores.as_ptr(),
    }
}





/***** LIBRARY *****/
/// The Queue struct wraps around a Device Queue to submit easily.
pub struct Queue {
    /// The parent Device.
    pub(crate) device : Rc<Device>,
    /// The Queue object to wrap.
    pub(crate) queue  : vk::Queue,
}

impl Queue {
    /// Submits the given command buffer to this queue.
    /// 
    /// # Arguments
    /// - `command_buffer`: The CommandBuffer to submit to.
    /// - `wait_semaphores`: One or more Semaphores to wait for before we can start rendering.
    /// - `done_semaphores`: One or more Semaphores to signal when we're done rendering.
    /// - `done_fence`: Fence to signal when rendering is done.
    /// 
    /// # Errors
    /// This function errors if we fail to submit the queue.
    pub fn submit(&self, command_buffer: &Rc<CommandBuffer>, wait_semaphores: &[&Rc<Semaphore>], done_semaphores: &[&Rc<Semaphore>], done_fence: &Rc<Fence>) -> Result<(), Error> {
        // Cast the semaphores and generate a list of wait stages
        let vk_wait_semaphores: Vec<vk::Semaphore>      = wait_semaphores.iter().map(|sem| sem.vk()).collect();
        let vk_wait_stages: Vec<vk::PipelineStageFlags> = (0..wait_semaphores.len()).map(|_| PipelineStage::COLOUR_ATTACHMENT_OUTPUT.into()).collect();
        let vk_done_semaphores: Vec<vk::Semaphore>      = done_semaphores.iter().map(|sem| sem.vk()).collect();

        // Prepare the SubmitInfo
        let vk_command_buffers: [vk::CommandBuffer; 1] = [command_buffer.vk()];
        let submit_info = populate_submit_info(&vk_command_buffers, &vk_wait_semaphores, &vk_wait_stages, &vk_done_semaphores);

        // Submit!
        if let Err(err) = done_fence.reset() { return Err(Error::FenceResetError{ err }); }
        unsafe {
            match self.device.queue_submit(self.queue, &[submit_info], done_fence.vk()) {
                Ok(_)    => Ok(()),
                Err(err) => Err(Error::SubmitError{ err }),
            }
        }
    }



    /// Returns the parent Device.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the internal VkQueue object.
    #[inline]
    pub fn vk(&self) -> vk::Queue { self.queue }
}
