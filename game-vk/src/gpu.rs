/* GPU.rs
 *   by Lut99
 *
 * Created:
 *   27 Mar 2022, 13:19:36
 * Last edited:
 *   27 Mar 2022, 16:36:45
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Gpu struct, which handles both physical and logical
 *   devices in the Vulkan backend.
**/

use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::ptr;

use ash::vk;
use log::debug;

use game_utl::to_cstring;

pub use crate::errors::GpuError as Error;
use crate::instance::Instance;


/***** HELPER FUNCTIONS *****/
/// Checks if the given physical device supports the given lists of device extensions, device layers and device features.
/// 
/// # Errors
/// 
/// This function returns errors if the given device does not support all of the required extensions, layers and features.
fn supports(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    physical_device_index: usize,
    physical_device_name: &str,
    p_device_extensions: &[*const i8],
    p_device_layers: &[*const i8],
    features: &vk::PhysicalDeviceFeatures,
) -> Result<(), Error> {
    // Test if all of the given extensions are supported on this device
    let avail_extensions = match unsafe { instance.enumerate_device_extension_properties(physical_device) } {
        Ok(extensions) => extensions,
        Err(err)       => { return Err(Error::DeviceExtensionEnumerateError{ err }); }
    };
    for req_ext in p_device_extensions {
        // Cast it to a CStr
        let req_ext: &CStr = unsafe { &CStr::from_ptr(*req_ext) };

        // Iterate through the available extensions
        let mut found = false;
        for avail_ext in &avail_extensions {
            // Make sure it's a CStr
            let avail_ext: &CStr = unsafe { &CStr::from_ptr(avail_ext.extension_name.as_ptr()) };

            // Compare them
            if req_ext == avail_ext { found = true; break; }
        }

        // If still not found, error
        if !found { return Err(Error::UnsupportedDeviceExtension{ index: physical_device_index, name: physical_device_name.to_string(), extension: req_ext.to_owned() }); }
    }

    // Next, test if all layers are supported
    let avail_layers = match unsafe { instance.enumerate_device_layer_properties(physical_device) } {
        Ok(layers) => layers,
        Err(err)   => { return Err(Error::DeviceLayerEnumerateError{ err }); }
    };
    for req_lay in p_device_layers {
        // Cast it to a CStr
        let req_lay: &CStr = unsafe { &CStr::from_ptr(*req_lay) };

        // Iterate through the available extensions
        let mut found = false;
        for avail_lay in &avail_layers {
            // Make sure it's a CStr
            let avail_lay: &CStr = unsafe { &CStr::from_ptr(avail_lay.layer_name.as_ptr()) };

            // Compare them
            if req_lay == avail_lay { found = true; break; }
        }

        // If still not found, error
        if !found { return Err(Error::UnsupportedDeviceLayer{ index: physical_device_index, name: physical_device_name.to_string(), layer: req_lay.to_owned() }); }
    }

    // Finally, test if features are supported
    let avail_features: vk::PhysicalDeviceFeatures = unsafe { instance.get_physical_device_features(physical_device) };
    /* TODO */

    // We support it
    Ok(())
}





/***** POPULATE FUNCTIONS *****/
/// Populates a DeviceQueueCreateInfo struct.
/// 
/// Uses the given parameters to describe a new set of queues from a single queue family.
/// 
/// The number of queues we will construct for this family depends on the length of the given queue_priorities list.
#[inline]
fn populate_queue_info(family_index: u32, queue_priorities: &[f32]) -> vk::DeviceQueueCreateInfo {
    vk::DeviceQueueCreateInfo {
        // Define the often-used fields on these structs
        s_type : vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        p_next : ptr::null(),
        flags  : vk::DeviceQueueCreateFlags::empty(),

        // Define to which queue family the new queues belong
        queue_family_index : family_index,
        // Define the queue priorities. The length of this list determines how many queues.
        p_queue_priorities : queue_priorities.as_ptr(),
        queue_count        : queue_priorities.len() as u32,
    }
}

/// Populates a DeviceCreateInfo struct.
/// 
/// Uses the given properties to initialize a DeviceCreateInfo struct. Some checks are done beforehand, like if all extensions / layers / features are supported on this device.
/// 
/// # Errors
/// 
/// Error only occur when the given device does not support all of the given extensions / layers / features.
fn populate_device_info(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    physical_device_index: usize,
    physical_device_name: &str,
    queue_infos: &[vk::DeviceQueueCreateInfo],
    p_device_extensions: &[*const i8],
    p_device_layers: &[*const i8],
    features: &vk::PhysicalDeviceFeatures,
) -> Result<vk::DeviceCreateInfo, Error> {
    // Make sure that the physical device supports everything
    supports(instance, physical_device, physical_device_index, physical_device_name, p_device_extensions, p_device_layers, features)?;

    // With the checks complete, throw everything in the resulting struct
    Ok(vk::DeviceCreateInfo {
        // Do the standard stuff
        s_type : vk::StructureType::DEVICE_CREATE_INFO,
        p_next : ptr::null(),
        flags  : vk::DeviceCreateFlags::empty(),

        // Define the queue create infos
        p_queue_create_infos    : queue_infos.as_ptr(),
        queue_create_info_count : queue_infos.len() as u32,

        // Define the extensions
        pp_enabled_extension_names : p_device_extensions.as_ptr(),
        enabled_extension_count    : p_device_extensions.len() as u32,

        // Define the layer
        pp_enabled_layer_names : p_device_layers.as_ptr(),
        enabled_layer_count    : p_device_layers.len() as u32,

        // Finally, define the features
        p_enabled_features : features,
    })
}





/***** AUXILLARY STRUCTS *****/
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
    /// # Examples
    /// 
    /// ```
    /// use ash::vk::PhysicalDevice;
    /// 
    /// use game_vk::gpu::QueueFamilyInfo;
    /// 
    /// // We assume the user gets some PhysicalDevice somehow
    /// let physical_device: PhysicalDevice = ...;
    /// 
    /// // Construct the QueueFamilyInfo
    /// let family_info = QueueFamilyInfo::new(physical_device)
    ///     .expect("Given physical device does not support all the required queue operations.");
    /// 
    /// println!("Family to use for graphics operations: {}", family_info.graphics);
    /// println!("Family to use for memory operations: {}", family_info.memory);
    /// println!("Family to use for compute operations: {}", family_info.compute);
    /// ```
    /// 
    /// # Errors
    /// 
    /// Throws an Error::OperationUnsupported for the given physical device if it does not support all kind of operations.
    fn new(instance: &Instance, physical_device: vk::PhysicalDevice, physical_device_index: usize, physical_device_name: &str) -> Result<Self, Error> {
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
            return Err(Error::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::GRAPHICS });
        }
        if memory.is_none() {
            return Err(Error::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::TRANSFER });
        }
        if compute.is_none() {
            return Err(Error::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::COMPUTE });
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
    fn new(family_info: &'a QueueFamilyInfo) -> Self {
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
    fn new(device: &ash::Device, family_info: &QueueFamilyInfo) -> Self {
        Self {
            graphics : unsafe { device.get_device_queue(family_info.graphics, 0) },
            memory   : unsafe { device.get_device_queue(family_info.memory, 0) },
            compute  : unsafe { device.get_device_queue(family_info.compute, 0) },
        }
    }
}





/***** LIBRARY *****/
/// The Gpu struct provides logic to work with both Vulkan's PhysicalDevices and Devices.
pub struct Gpu {
    /// The PhysicalDevice around which we wrap.
    physical_device : vk::PhysicalDevice,
    /// The logical Device around which we wrap.
    device          : ash::Device,
    /// The queues for the internal device.
    queues          : Queues,
    
    /// The name of the device
    name           : String,
    /// The type of the device (as a String as well)
    kind           : String,
    /// The QueueFamilyInfo that describes the queue families for this device.
    queue_families : QueueFamilyInfo,
}

impl Gpu {
    /// Constructor for the Gpu.
    /// 
    /// This function tries to build a logical Device around the given physical Device, checking if it supports the given surface.
    /// 
    /// Also attempts to enable the given extensions and features on the device.
    /// 
    /// # Examples 
    /// 
    /// ```
    /// use game_vk::gpu::Gpu;
    /// use game_vk::instance::Instance;
    /// 
    /// let instance = Instance::new(...);
    /// 
    /// let gpu = Gpu::new(&instance, 0, &vec![], &vec![], &Default::default())
    ///     .unwrap_or_else(|err| panic!("Could not create new device: {}", err));
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors whenever the backend Vulkan errors.
    pub fn new<'a, 'b>(instance: &Instance, physical_device_index: usize, device_extensions: &[&'a str], device_layers: &[&'b str], device_features: &vk::PhysicalDeviceFeatures) -> Result<Self, Error> {
        // We enumerate through all the physical devices to find the appropriate one
        let physical_devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => devices,
            Err(err)    => { return Err(Error::PhysicalDeviceEnumerateError{ err }); }  
        };
        let mut target_physical_device: Option<vk::PhysicalDevice> = None;
        for (i, physical_device) in physical_devices.iter().enumerate() {
            // Check if this has the index we want
            if i == physical_device_index {
                // It is; we'll take it
                target_physical_device = Some(*physical_device);
            }
        }
        let physical_device = match target_physical_device {
            Some(device) => device,
            None         => { return Err(Error::PhysicalDeviceNotFound{ index: physical_device_index }); }
        };
    


        // Get the properties of this device
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };

        // Get a readable name and type
        let device_name: String = match unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) }.to_str() {
            Ok(name) => name.to_string(),
            Err(err) => { return Err(Error::PhysicalDeviceNameError{ index: physical_device_index, err }); }
        };
        let device_type: String = match device_properties.device_type {
            vk::PhysicalDeviceType::CPU            => "CPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU    => "Virtual GPU",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU   => "Discrete GPU",
            _                                      => "Unknown type",
        }.to_string();



        // Collect the queue families for this device
        let family_info = QueueFamilyInfo::new(&instance, physical_device, physical_device_index, &device_name)?;



        // Do some debug prints about the selected device
        debug!("Using physical device {} '{}' ({})", physical_device_index, &device_name, &device_type);
        debug!("Selected queue families:");
        debug!(" - Graphics : {}", family_info.graphics);
        debug!(" - Memory   : {}", family_info.memory);
        debug!(" - Compute  : {}", family_info.compute);



        // Prepare getting the queues from the device
        let queue_priorities = vec![ 1.0 ];
        let queue_infos: Vec<vk::DeviceQueueCreateInfo> = family_info.unique().map(|family| populate_queue_info(family, &queue_priorities)).collect();



        // Map the given device extensions and layers to pointers
        let device_extensions: Vec<CString> = device_extensions.iter().map(|extension| to_cstring!(extension)).collect();
        let device_layers: Vec<CString>     = device_layers.iter().map(|layer| to_cstring!(layer)).collect();
        let p_device_extensions: Vec<*const i8> = (0..device_extensions.len()).map(|i| device_extensions[i].as_ptr()).collect();
        let p_device_layers: Vec<*const i8>     = (0..device_layers.len()).map(|i| device_layers[i].as_ptr()).collect();



        // Create the DeviceCreateInfo with all this
        let device_info = populate_device_info(&instance, physical_device, physical_device_index, &device_name, &queue_infos, &p_device_extensions, &p_device_layers, &device_features)?;

        // Use that to create the device
        debug!("Initializing device...");
        let device: ash::Device = unsafe {
            match instance.create_device(physical_device, &device_info, None) {
                Ok(device) => device,
                Err(err)   => { return Err(Error::DeviceCreateError{ err }); }
            }
        };

        // Get the queues
        let queues = Queues::new(&device, &family_info);



        // Done! Return the new GPU
        Ok(Self {
            physical_device,
            device,
            queues,

            name           : device_name,
            kind           : device_type,
            queue_families : family_info,
        })
    }



    /// Tries to automatically select the best GPU.
    /// 
    /// Iterates through all the GPUs that can be found in the given instance, and then tries to select the most appropriate one for the Game.
    /// 
    /// # Examples
    /// 
    /// 
    /// # Errors
    /// 
    /// This function errors when we could not enumerate the physical devices or if no GPU is found that can support this application.
    pub fn auto_select<'a, 'b>(instance: &Instance, device_extensions: &[&'a str], device_layers: &[&'b str], device_features: &vk::PhysicalDeviceFeatures) -> Result<usize, Error> {
        // Map the given device extensions and layers to pointers
        let device_extensions: Vec<CString> = device_extensions.iter().map(|extension| to_cstring!(extension)).collect();
        let device_layers: Vec<CString>     = device_layers.iter().map(|layer| to_cstring!(layer)).collect();
        let p_device_extensions: Vec<*const i8> = (0..device_extensions.len()).map(|i| device_extensions[i].as_ptr()).collect();
        let p_device_layers: Vec<*const i8>     = (0..device_layers.len()).map(|i| device_layers[i].as_ptr()).collect();

        // Iterate over all physical devices
        let physical_devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => devices,
            Err(err)    => { return Err(Error::PhysicalDeviceEnumerateError{ err }); }  
        };
        let mut best_device: Option<(usize, u32)> = None;
        for (i, physical_device) in physical_devices.iter().enumerate() {
            // Get the properties of this device
            let device_properties = unsafe { instance.get_physical_device_properties(*physical_device) };

            // Get a readable name and type
            let device_name: String = match unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) }.to_str() {
                Ok(name) => name.to_string(),
                Err(err) => { return Err(Error::PhysicalDeviceNameError{ index: i, err }); }
            };

            // Check if this device is supported
            if supports(instance, *physical_device, i, &device_name, &p_device_extensions, &p_device_layers, &device_features).is_err() { continue; }

            // It is; now base its ranking on its 'CPU disconnectedness'
            let device_ranking: u32 = match device_properties.device_type {
                vk::PhysicalDeviceType::CPU            => 1,
                vk::PhysicalDeviceType::VIRTUAL_GPU    => 2,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 3,
                vk::PhysicalDeviceType::DISCRETE_GPU   => 4,
                _                                      => 0,
            };

            // Select it as best if first or higher ranking
            if best_device.is_none() || (device_ranking > best_device.as_ref().unwrap().1) {
                best_device = Some((i, device_ranking));
            }
        }
        
        // If there is none, error
        match best_device {
            Some((index, _)) => Ok(index),
            None             => Err(Error::NoSupportedPhysicalDevices),
        }
    }

    /// Lists all GPUs that Vulkan can find and that support the given extensions to stdout.
    /// 
    /// # Errors
    /// 
    /// This function errors when we could not enumerate the physical devices.
    pub fn list<'a, 'b>(instance: &Instance, device_extensions: &[&'a str], device_layers: &[&'b str], device_features: &vk::PhysicalDeviceFeatures) -> Result<(), Error> {
        // Map the given device extensions and layers to pointers
        let device_extensions: Vec<CString> = device_extensions.iter().map(|extension| to_cstring!(extension)).collect();
        let device_layers: Vec<CString>     = device_layers.iter().map(|layer| to_cstring!(layer)).collect();
        let p_device_extensions: Vec<*const i8> = (0..device_extensions.len()).map(|i| device_extensions[i].as_ptr()).collect();
        let p_device_layers: Vec<*const i8>     = (0..device_layers.len()).map(|i| device_layers[i].as_ptr()).collect();

        // Iterate over all physical devices
        let physical_devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => devices,
            Err(err)    => { return Err(Error::PhysicalDeviceEnumerateError{ err }); }  
        };
        let mut supported_devices: Vec<(usize, String, String)>   = Vec::with_capacity(physical_devices.len());
        let mut unsupported_devices: Vec<(usize, String, String)> = Vec::with_capacity(physical_devices.len());
        for (i, physical_device) in physical_devices.iter().enumerate() {
            // Get the properties of this device
            let device_properties = unsafe { instance.get_physical_device_properties(*physical_device) };

            // Get a readable name and type
            let device_name: String = match unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) }.to_str() {
                Ok(name) => name.to_string(),
                Err(err) => { return Err(Error::PhysicalDeviceNameError{ index: i, err }); }
            };
            let device_type: String = match device_properties.device_type {
                vk::PhysicalDeviceType::CPU            => "CPU",
                vk::PhysicalDeviceType::VIRTUAL_GPU    => "Virtual GPU",
                vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
                vk::PhysicalDeviceType::DISCRETE_GPU   => "Discrete GPU",
                _                                      => "Unknown type",
            }.to_string();

            // Determine to which list to add it
            if supports(instance, *physical_device, i, &device_name, &p_device_extensions, &p_device_layers, &device_features).is_ok() {
                supported_devices.push((i, device_name, device_type));
            } else {
                unsupported_devices.push((i, device_name, device_type));
            }
        }

        // Print everything neatly
        println!();
        println!("Supported devices:");
        if !supported_devices.is_empty() {
            for (index, name, kind) in supported_devices {
                println!("  {}) {} ({})", index, name, kind);
            }
        } else {
            println!("  <no devices>");
        }
        println!();
        
        println!("Unsupported devices:");
        if !unsupported_devices.is_empty() {
            for (index, name, kind) in unsupported_devices {
                println!("  {}) {} ({})", index, name, kind);
            }
        } else {
            println!("  <no devices>");
        }
        println!();
        println!();

        // Done!
        Ok(())
    }



    /// Returns the name of this device.
    #[inline]
    pub fn name(&self) -> &str { &self.name }
    
    /// Returns the type of this device (as a String).
    #[inline]
    pub fn kind(&self) -> &str { &self.kind }
    
    /// Returns information about the QueueFamilies for this device.
    #[inline]
    pub fn families(&self) -> &QueueFamilyInfo { &self.queue_families }

    /// Returns the internal device.
    #[inline]
    pub fn device(&self) -> &ash::Device { &self.device }

    /// Returns the internal physical device.
    #[inline]
    pub fn physical_device(&self) -> &vk::PhysicalDevice { &self.physical_device }

    /// Returns the internal Queues struct, which contains the queues used on this device.
    #[inline]
    pub fn queues(&self) -> &Queues { &self.queues }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        // Destroy the internal device
        unsafe { self.device.destroy_device(None); };
    }
}

impl Deref for Gpu {
    type Target = ash::Device;
    
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
