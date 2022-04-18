/* AUXILLARY.rs
 *   by Lut99
 *
 * Created:
 *   18 Apr 2022, 12:27:51
 * Last edited:
 *   18 Apr 2022, 12:59:00
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains auxillary wrapped structs around Vulkan structs, to not
 *   expose any ash to the outside world.
**/

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FResult};
use std::sync::Arc;

use ash::vk;

pub use crate::errors::QueueError;
use crate::instance::Instance;


/***** DEVICES *****/
/// Enumerates the possible Device types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceKind {
    /// A discrete GPU. Is given the highest 'CPU disconnectedness' score.
    Discrete,
    /// An intergrated but dedicated GPU. Is given the second highest 'CPU disconnectedness' score.
    Integrated,
    /// A Virtual GPU, which is given the third-worst 'CPU disconnectedness' score.
    Virtual,
    /// No dedicated GPU at all, just the CPU doing GPU stuff. Is given the fourth-worst 'CPU disconnectedness' score.
    Cpu,
    /// A GPU type which we do not know, which we prefer the least (worst 'CPU disconnectedness' score).
    Other,
}

impl DeviceKind {
    /// Returns a so-ca,lled 'CPU disconnectedness' score, which we hope to equate to a device's power when comparing multiple.
    /// 
    /// We assume that devices with a higher score are more discrete, and thus more powerful.
    /// 
    /// # Returns
    /// The score as an unsigned integer.
    #[inline]
    pub fn score(&self) -> u32 {
        use DeviceKind::*;
        match self {
            Discrete   => 4,
            Integrated => 3,
            Virtual    => 2,
            Cpu        => 1,
            Other      => 0,
        }
    }
}

impl Ord for DeviceKind {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by CPU disconnectedness
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for DeviceKind {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for DeviceKind {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use DeviceKind::*;
        match self {
            Discrete   => write!(f, "Discrete"),
            Integrated => write!(f, "Integrated"),
            Virtual    => write!(f, "Virtual"),
            Cpu        => write!(f, "Cpu"),
            Other      => write!(f, "Other"),
        }
    }
}

impl From<vk::PhysicalDeviceType> for DeviceKind {
    #[inline]
    fn from(value: vk::PhysicalDeviceType) -> Self {
        match value {
            vk::PhysicalDeviceType::DISCRETE_GPU   => DeviceKind::Discrete,
            vk::PhysicalDeviceType::INTEGRATED_GPU => DeviceKind::Integrated,
            vk::PhysicalDeviceType::VIRTUAL_GPU    => DeviceKind::Virtual,
            vk::PhysicalDeviceType::CPU            => DeviceKind::Cpu,
            _                                      => DeviceKind::Other,
        }
    }
}

impl From<DeviceKind> for vk::PhysicalDeviceType {
    #[inline]
    fn from(value: DeviceKind) -> Self {
        match value {
            DeviceKind::Discrete   => vk::PhysicalDeviceType::DISCRETE_GPU,
            DeviceKind::Integrated => vk::PhysicalDeviceType::INTEGRATED_GPU,
            DeviceKind::Virtual    => vk::PhysicalDeviceType::VIRTUAL_GPU,
            DeviceKind::Cpu        => vk::PhysicalDeviceType::CPU,
            DeviceKind::Other      => vk::PhysicalDeviceType::OTHER,
        }
    }
}





/***** QUEUES *****/
/// Contains information about the queue families for an instantiated GPU.
pub struct QueueFamilyInfo {
    /// The index of the queue we're going to use for graphics operations
    pub graphics : u32,
    /// The index of the queue we're going to use for memory operations
    pub memory   : u32,
    /// The index of the queue we're going to use for compute operations
    pub compute  : u32,
}

impl QueueFamilyInfo {
    /// Constructor for the QueueFamilyInfo.
    /// 
    /// Maps the queue families of the given PhysicalDevice to their usage. Will try to use as many different queue families as possible.
    /// 
    /// # Arguments
    /// - `instance`: A reference to an Instance pointer used to query the properties of a physical device.
    /// - `physical_device_index`: The index of the physical device we are trying to get info from. Only used for debugging purposes.
    /// - `physical_device_name`: The name of the physical device we are trying to get info from. Only used for debugging purposes.
    /// 
    /// # Returns
    /// The new QueueFamilyInfo struct on success, or else a QueueError::OperationNotSupported error if the given device does not support all required queue family types.
    pub(crate) fn new(instance: &Arc<Instance>, physical_device: vk::PhysicalDevice, physical_device_index: usize, physical_device_name: &str) -> Result<Self, QueueError> {
        // Prepare placeholders for the different queues
        let mut graphics : Option<(u32, usize)> = None;
        let mut memory : Option<(u32, usize)>   = None;
        let mut compute : Option<(u32, usize)>  = None;

        // Iterate over the queue families
        let families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for (i, family) in families.iter().enumerate() {
            // We need at least one queue in each family, obviously
            if family.queue_count == 0 { continue; }

            // Count the number of operations this queue can do
            let mut n_operations = 0;
            let supports_graphics = if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) { n_operations += 1; true } else { false };
            let supports_memory   = if family.queue_flags.contains(vk::QueueFlags::TRANSFER) { n_operations += 1; true } else { false };
            let supports_compute  = if family.queue_flags.contains(vk::QueueFlags::COMPUTE) { n_operations += 1; true } else { false };
            
            // Note the queue on every slot it supports, except we already have a more specialized one
            if supports_graphics && (graphics.is_none() || n_operations < graphics.as_ref().unwrap().1) {
                graphics = Some((i as u32, n_operations));
            }
            if supports_memory && (memory.is_none() || n_operations < memory.as_ref().unwrap().1) {
                memory = Some((i as u32, n_operations));
            }
            if supports_compute && (compute.is_none() || n_operations < compute.as_ref().unwrap().1) {
                compute = Some((i as u32, n_operations));
            }
        }

        // If we didn't find one of the queues, error
        if graphics.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::GRAPHICS });
        }
        if memory.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::TRANSFER });
        }
        if compute.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::COMPUTE });
        }

        // Otherwise, we can populate ourselves!
        Ok(QueueFamilyInfo {
            graphics : graphics.unwrap().0,
            memory   : memory.unwrap().0,
            compute  : compute.unwrap().0,
        })
    }



    /// Returns an iterator over the **different** families in the QueueFamilyInfo.
    #[inline]
    pub fn unique(&self) -> QueueFamilyInfoUniqueIterator {
        QueueFamilyInfoUniqueIterator::new(self)
    }

    /// Returns the number of **different** families in the QueueFamilyInfo.
    pub fn unique_len(&self) -> usize {
        if self.graphics != self.memory && self.graphics != self.compute && self.memory != self.compute {
            3
        } else if self.graphics != self.memory || self.graphics != self.compute || self.memory != self.compute {
            2
        } else {
            1
        }
    }
}



/// Implements an iterator over the unique family indices in the QueueFamilyInfo.
pub struct QueueFamilyInfoUniqueIterator<'a> {
    /// The QueueFamilyInfo over which we iterate
    family_info : &'a QueueFamilyInfo,
    /// The current 'position' in the family info
    index       : usize,
}

impl<'a> QueueFamilyInfoUniqueIterator<'a> {
    /// Constructor for the QueueFamilyInfoUniqueIterator.
    /// 
    /// Prepares a new iterator over the given QueueFamilyInfo.
    /// 
    /// Note that it's passed by reference, so it's probably not a good idea to modify queue families while iterating over them.
    #[inline]
    pub(crate) fn new(family_info: &'a QueueFamilyInfo) -> Self {
        Self {
            family_info,
            index : 0,
        }
    }
}

impl<'a> Iterator for QueueFamilyInfoUniqueIterator<'a> {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Match based on the index
        match self.index {
            0 => { self.index += 1; Some(self.family_info.graphics) },
            1 => {
                // Only do this one if it's unique
                self.index += 1;
                if self.family_info.memory != self.family_info.graphics {
                    Some(self.family_info.memory)
                } else {
                    // Skip to the next value
                    self.next()
                }
            },
            2 => {
                // Only do this one if it's unique
                self.index += 1;
                if self.family_info.compute != self.family_info.graphics && self.family_info.compute != self.family_info.memory {
                    Some(self.family_info.compute)
                } else {
                    // Skip to the next value
                    self.next()
                }
            }
            _ => None,
        }
    }
}



/// Central place where we store the queues of the created logical device.
pub struct Queues {
    /// The graphics queue
    pub graphics : vk::Queue,
    /// The memory queue
    pub memory   : vk::Queue,
    /// The compute queue
    pub compute  : vk::Queue,
}

impl Queues {
    /// Constructor for the Queues.
    /// 
    /// Requests the three queues from the queue families in the given QueueFamilyInfo on the given vk::Device.
    #[inline]
    pub(crate) fn new(device: &ash::Device, family_info: &QueueFamilyInfo) -> Self {
        Self {
            graphics : unsafe { device.get_device_queue(family_info.graphics, 0) },
            memory   : unsafe { device.get_device_queue(family_info.memory, 0) },
            compute  : unsafe { device.get_device_queue(family_info.compute, 0) },
        }
    }
}





/***** SURFACES *****/
/// Collects information about the SwapchainSupport for this device.
pub struct SwapchainSupport {
    /// Lists the capabilities of the chosen device/surface combo.
    pub capabilities  : vk::SurfaceCapabilitiesKHR,
    /// Lists the formats supported by the chosen device/surface combo.
    pub formats       : Vec<vk::SurfaceFormatKHR>,
    /// Lists the present modes supported by the chosen device/surface combo.
    pub present_modes : Vec<vk::PresentModeKHR>,
}
