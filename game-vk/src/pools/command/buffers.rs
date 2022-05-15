/* BUFFERS.rs
 *   by Lut99
 *
 * Created:
 *   05 May 2022, 10:45:36
 * Last edited:
 *   14 May 2022, 12:50:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the buffer definitions for this type of Pool.
**/

use std::ptr;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use ash::vk;

pub use crate::pools::errors::CommandPoolError as Error;
use crate::log_destroy;
use crate::auxillary::{BindPoint, CommandBufferUsageFlags, Rect2D};
use crate::device::Device;
use crate::pipeline::Pipeline;
use crate::render_pass::RenderPass;
use crate::framebuffer::Framebuffer;
use crate::pools::command::Pool as CommandPool;


/***** POPULATE FUNCTIONS *****/
/// Populates the begin info for recording a new command buffer.
/// 
/// # Arguments
/// - `flags`: The CommandBufferUsage flags to set for this buffer.
/// - `inheritance_info`: The VkCommandBufferInheritenceInfo struct that describes what a secondary command buffer has to inherit from a primary one. Should be NULL if this is a primary buffer.
#[inline]
fn populate_begin_info(flags: vk::CommandBufferUsageFlags, inheritance_info: *const vk::CommandBufferInheritanceInfo) -> vk::CommandBufferBeginInfo {
    vk::CommandBufferBeginInfo {
        // Do the standard stuff
        s_type : vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next : ptr::null(),
        
        // Set the flags for the buffers
        flags,

        // Set the inheritence info
        p_inheritance_info : inheritance_info,
    }
}

/// Populates a VkRenderPassBeginInfo struct.
/// 
/// # Arguments
/// - `render_pass`: The VkRenderPass to begin.
/// - `framebuffer`: The VkFramebuffer to render to in this pass.
/// - `render_area`: A VkRect2D detailling the area of the framebuffer to render to.
/// - `clear_values`: A list of 4D colour vectors that indicate the colour to reset the framebuffer for when loading it (if set so in the render pass).
#[inline]
fn populate_render_pass_begin_info(render_pass: vk::RenderPass, framebuffer: vk::Framebuffer, render_area: vk::Rect2D, clear_values: &[vk::ClearValue]) -> vk::RenderPassBeginInfo {
    vk::RenderPassBeginInfo {
        // Set default stuff
        s_type : vk::StructureType::RENDER_PASS_BEGIN_INFO,
        p_next : ptr::null(),

        // Set the render pass and framebuffer
        render_pass,
        framebuffer,

        // Set the render area
        render_area,

        // Set the list of clear values
        clear_value_count : clear_values.len() as u32,
        p_clear_values    : clear_values.as_ptr(),
    }
}





/***** LIBRARY *****/
/// The CommandBuffer is used to record various GPU commands in.
pub struct CommandBuffer {
    /// The parent CommandPool where this buffer was allocated from.
    pub(crate) device  : Rc<Device>,
    /// The parent CommandPool where this buffer was allocated from.
    pub(crate) pool    : Arc<RwLock<CommandPool>>,
    /// The parent VkCommandPool where this buffer was allocated from.
    pub(crate) vk_pool : vk::CommandPool,

    /// The VkCommandBuffer around which we wrap.
    pub(crate) buffer : vk::CommandBuffer,
}

impl CommandBuffer {
    /// Prepares the CommandBuffer for recording.
    /// 
    /// # Arguments
    /// - `flags`: The CommandBufferUsageFlags that define some optional begin states.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend could not begin the command buffer.
    pub fn begin(&self, flags: CommandBufferUsageFlags) -> Result<(), Error> {
        // Populate the begin info
        let begin_info = populate_begin_info(flags.into(), ptr::null());

        // Begin the buffer
        unsafe {
            if let Err(err) = self.device.begin_command_buffer(self.buffer, &begin_info) {
                return Err(Error::CommandBufferBeginError{ err });
            }
        }

        // Success
        Ok(())
    }

    /// Records the beginning of a RenderPass.
    /// 
    /// # Arguments
    /// - `render_pass`: The RenderPass to begin.
    /// - `framebuffer`: The Framebuffer to render to in this pass.
    /// - `render_area`: A Rect2D detailling the area of the framebuffer to render to.
    /// - `clear_values`: A list of 4D colour vectors that indicate the colour to reset the framebuffer for when loading it (if set so in the render pass).
    /// 
    /// # Errors
    /// This function does not error directly, but may pass errors on to `CommandBuffer::end()`.
    pub fn begin_render_pass(&self, render_pass: &Rc<RenderPass>, framebuffer: &Rc<Framebuffer>, render_area: Rect2D<i32, u32>, clear_values: &[[f32; 4]]) {
        // Cast the clear values
        let vk_clear_values: Vec<vk::ClearValue> = clear_values.iter().map(|value| {
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: *value,
                }
            }
        }).collect();

        // Prepare the begin info
        let begin_info = populate_render_pass_begin_info(render_pass.vk(), framebuffer.vk(), render_area.into(), &vk_clear_values);

        // Begin!
        unsafe {
            self.device.cmd_begin_render_pass(self.buffer, &begin_info, vk::SubpassContents::INLINE);
        }
    }

    /// Binds the given pipeline to this RenderPass.
    /// 
    /// # Arguments
    /// - `bind_point`: The BindPoint where to bind the pipeline.
    /// - `pipeline`: The Pipeline to bind.
    /// 
    /// # Errors
    /// This function does not error directly, but may pass errors on to `CommandBuffer::end()`.
    pub fn bind_pipeline(&self, bind_point: BindPoint, pipeline: &Rc<Pipeline>) {
        unsafe {
            self.device.cmd_bind_pipeline(self.buffer, bind_point.into(), pipeline.vk());
        }
    }

    /// Records a draw call.
    /// 
    /// # Arguments
    /// - `n_vertices`: The number of vertices to draw.
    /// - `n_instances`: The number of instances to draw.
    /// - `first_vertex`: The first vertex in the buffer to draw.
    /// - `first_instance`: The first instance in the buffer to draw.
    /// 
    /// # Errors
    /// This function does not error directly, but may pass errors on to `CommandBuffer::end()`.
    pub fn draw(&self, n_vertices: u32, n_instances: u32, first_vertex: u32, first_instance: u32) {
        unsafe {
            self.device.cmd_draw(self.buffer, n_vertices, n_instances, first_vertex, first_instance);
        }
    }

    /// Records the end of a RenderPass.
    /// 
    /// # Errors
    /// This function does not error directly, but may pass errors on to `CommandBuffer::end()`.
    pub fn end_render_pass(&self) {
        unsafe {
            self.device.cmd_end_render_pass(self.buffer);
        }
    }

    /// Ends recording in the CommandBuffer.
    /// 
    /// # Errors
    /// This function errors if any of the other record steps that delayed any errors has errored.
    pub fn end(&self) -> Result<(), Error> {
        unsafe {
            if let Err(err) = self.device.end_command_buffer(self.buffer) {
                return Err(Error::CommandBufferRecordError{ err });
            }
        }
        Ok(())
    }



    /// Returns the parent Device where this buffer lives.
    #[inline]
    pub fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the parent Pool where this buffer lives.
    #[inline]
    pub fn pool(&self) -> &Arc<RwLock<CommandPool>> { &self.pool }

    /// Returns the internal buffer.
    #[inline]
    pub fn vk(&self) -> vk::CommandBuffer { self.buffer }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        // Call free on the parent pool
        log_destroy!(self, CommandBuffer);
        unsafe { self.device.free_command_buffers(self.vk_pool, &[self.buffer]); }
    }
}
