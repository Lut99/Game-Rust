//  SYSTEM.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 18:10:00
//  Last edited:
//    31 Jul 2022, 12:23:03
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the main code of the RenderSystem.
// 

use std::cell::RefCell;
use std::rc::Rc;

use winit::event_loop::EventLoop;

use game_ecs::Ecs;
use game_spc::spec::Event;
use game_vk::auxillary::enums::DeviceExtension;
use game_vk::auxillary::structs::DeviceFeatures;
use game_vk::instance::Instance;
use game_vk::device::Device;
use game_vk::pools::command::Pool as CommandPool;
use game_vk::pools::memory::MetaPool;

pub use crate::errors::RenderError as Error;
use crate::spec::{AppInfo, VulkanInfo, WindowInfo};
use crate::components::{RenderTarget, Window};
use crate::windows;
use crate::pipelines;


/***** CONSTANTS *****/
/// The list of instance extensions we want to enable (besides the required surface ones).
const INSTANCE_EXTENSIONS: &[&str] = &[];

/// The list of instance layers we want to enable (besides the debug one).
const INSTANCE_LAYERS: &[&str] = &[];

/// The list of device extensions we want to enable.
const DEVICE_EXTENSIONS: &[&str] = &[ DeviceExtension::Swapchain.as_str() ];

/// The list of device layers we want to enable.
const DEVICE_LAYERS: &[&str] = &[];

// Constants that are lazily loaded
lazy_static!{
    /// The list of device features we want to enable.
    static ref DEVICE_FEATURES: DeviceFeatures = Default::default();
}





/***** LIBRARY *****/
/// The RenderSystem is in charge of rendering objects to targets (windows or other images / buffers).
pub struct RenderSystem {
    /// The Entity Component System around which the RenderSystem is build.
    ecs : Rc<RefCell<Ecs>>,

    /// The Instance which we'll use throughout the Game.
    instance     : Rc<Instance>,
    /// The Device which we'll use throughout the Game.
    device       : Rc<Device>,
    /// The CommandPool where we'll allocate command buffers from.
    command_pool : Rc<RefCell<CommandPool>>,
    /// The MetaPool where we'll allocate (some) buffers from.
    memory_pool  : Rc<RefCell<MetaPool>>,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The Entity Component System where all of the RenderSystem's entities will be stored.
    /// - `event_loop`: The EventLoop to which to bind new Windows.
    /// - `app_info`: The AppInfo struct that has some metadata about the application.
    /// - `vulkan_info`: The VulkanInfo struct that has Vulkan configuration options in it.
    /// - `window_info`: The WindowInfo struct that defines properties of the Game's Window.
    /// 
    /// # Returns
    /// A new RenderSystem.
    /// 
    /// # Errors
    /// The constructor may fail if we failed to initialize the Vulkan backend.
    pub fn new(ecs: Rc<RefCell<Ecs>>, event_loop: &EventLoop<Event>, app_info: AppInfo, vulkan_info: VulkanInfo, window_info: WindowInfo) -> Result<Self, Error> {
        // Register our components
        Ecs::register::<RenderTarget>(&ecs);
        Ecs::register::<Window>(&ecs);

        // Register the components of the pipelines
        pipelines::register(&ecs);



        // Instantiate the Vulkan objects we'll need; start with the Instance.
        let instance: Rc<Instance> = match Instance::new(app_info.name, app_info.version, app_info.engine_name, app_info.engine_version, INSTANCE_EXTENSIONS, INSTANCE_LAYERS) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }
        };

        // Do the Device next
        let device: Rc<Device> = match Device::new(instance.clone(), vulkan_info.gpu, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(device) => device,
            Err(err)   => { return Err(Error::DeviceCreateError{ err }); }
        };

        // Allocate the pools on the GPU
        let command_pool = match CommandPool::new(device.clone()) {
            Ok(pool) => pool,
            Err(err) => { return Err(Error::CommandPoolCreateError{ err }); }
        };

        // Allocate the memory pools on the GPU
        let memory_pool = MetaPool::new(device.clone(), 4096);



        // Next, create a Window to which we may desire to render
        let window = match windows::create(&ecs, event_loop, device.clone(), &window_info.title, window_info.mode) {
            Ok(window) => window,
            Err(err)   => { return Err(Error::WindowCreateError{ title: window_info.title, err }); }  
        };

        // Turn that window into a RenderPipeline.
        if let Err(err) = pipelines::triangle::create(&ecs, window) {
            return Err(Error::PipelineCreateError{ name: pipelines::triangle::NAME, err });
        }



        // Finally, return ourselves with the necessary objects
        Ok(Self {
            ecs,

            instance,
            device,
            command_pool,
            memory_pool,
        })
    }



    /// Returns the internal Entity Component System.
    #[inline]
    pub fn ecs(&self) -> &Rc<RefCell<Ecs>> { &self.ecs }
}
