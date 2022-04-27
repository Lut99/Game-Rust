/* LAYOUT.rs
 *   by Lut99
 *
 * Created:
 *   27 Apr 2022, 11:41:07
 * Last edited:
 *   27 Apr 2022, 12:40:15
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines a wrapper around a VkPipelineLayout struct.
**/

use std::ptr;
use std::sync::Arc;

use ash::vk;

pub use crate::errors::PipelineLayoutError as Error;
use crate::device::Device;
use crate::descriptors::{Error as DescriptorSetLayoutError, DescriptorSetLayout};


/***** POPULATE FUNCTIONS *****/
/// Populates a vk::PipelineLayoutCreateInfo struct based on the given arguments.
/// 
/// # Arguments
/// - `layouts`: The list of DescriptorSetLayouts to attach to the PipelineLayout.
/// 
/// # Returns
/// A new vk::PipelineLayoutCreateInfo with the same lifetime as the given vectors.
#[inline]
fn populate_layout_info(layouts: &[vk::DescriptorSetLayout]) -> vk::PipelineLayoutCreateInfo {
    vk::PipelineLayoutCreateInfo {
        // Set the default stuff
        s_type : vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        p_next : ptr::null(),
        flags  : vk::PipelineLayoutCreateFlags::empty(),

        // Attach the layouts
        p_set_layouts    : layouts.as_ptr(),
        set_layout_count : layouts.len() as u32,

        // Attach the push constants
        p_push_constant_ranges    : ptr::null(),
        push_constant_range_count : 0,
    }
}





/***** LIBRARY *****/
/// Defines a wrapper around a VkPipelineLayout struct.
pub struct PipelineLayout {
    /// Reference to the parent device of this layout
    device : Arc<Device>,
    /// The PipelineLayout we wrap
    layout : vk::PipelineLayout,
}

impl PipelineLayout {
    /// Constructor for the PipelineLayout.
    /// 
    /// # Arguments
    /// - `device`: The Device to build the pipeline layout on.
    /// - `layouts`: A list of DescriptorSetLayouts for this layout.
    /// - `push_constants`: A list of PushConstants for this layout.
    /// 
    /// # Returns
    /// A new PipelineLayout instance on success.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend could not create the new layout.
    pub fn new(device: Arc<Device>, layouts: &[DescriptorSetLayout]) -> Result<Arc<Self>, Error> {
        // Cast the layouts to their Vulkan counterparts
        let layouts: Vec<vk::DescriptorSetLayout> = layouts.iter().map(|layout| layout.vk()).collect();

        // Create the create info
        let layout_info = populate_layout_info(&layouts);

        // Create the pipeline layout itself
        let layout = unsafe {
            match device.create_pipeline_layout(&layout_info, None) {
                Ok(layout) => layout,
                Err(err)   => { return Err(Error::PipelineLayoutCreateError{ err }); }
            }
        };

        // Wrap it in this struct and done
        Ok(Arc::new(Self {
            device,
            layout,
        }))
    }
    
    /// Constructor for the PipelineLayout that takes DescriptorSetLayout results instead of the already resolved layouts.
    /// 
    /// # Arguments
    /// - `device`: The Device to build the pipeline layout on.
    /// - `layouts`: A list of DescriptorSetLayout results for this layout.
    /// - `push_constants`: A list of PushConstants for this layout.
    /// 
    /// # Returns
    /// A new PipelineLayout instance on success.
    /// 
    /// # Errors
    /// This function errors if the underlying Vulkan backend could not create the new layout.
    pub fn try_new(device: Arc<Device>, layouts: &[Result<Arc<DescriptorSetLayout>, DescriptorSetLayoutError>]) -> Result<Arc<Self>, Error> {
        // Cast the layouts to their Vulkan counterparts
        let mut vk_layouts: Vec<vk::DescriptorSetLayout> = Vec::with_capacity(layouts.len());
        for layout in layouts {
            // Try to resolve it
            let layout = match layout {
                Ok(layout) => layout,
                Err(err)   => { return Err(Error::DescriptorSetLayoutCreateError{ err: err.clone() }); }
            };

            // Add it to the list
            vk_layouts.push(layout.vk());
        }

        // Create the create info
        let layout_info = populate_layout_info(&vk_layouts);

        // Create the pipeline layout itself
        let layout = unsafe {
            match device.create_pipeline_layout(&layout_info, None) {
                Ok(layout) => layout,
                Err(err)   => { return Err(Error::PipelineLayoutCreateError{ err }); }
            }
        };

        // Wrap it in this struct and done
        Ok(Arc::new(Self {
            device,
            layout,
        }))
    }



    /// Returns the parent device of this layout
    #[inline]
    pub fn device(&self) -> &Arc<Device> { &self.device }

    /// Returns the internal VkPipelineLayout struct.
    #[inline]
    pub fn vk(&self) -> vk::PipelineLayout { self.layout }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe { self.device.destroy_pipeline_layout(self.layout, None); }
    }
}
