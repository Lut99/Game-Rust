//  SYSTEM.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 18:07:31
//  Last edited:
//    13 Aug 2022, 13:01:41
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the base RenderSystem.
// 

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use log::debug;
use rust_ecs::Ecs;
use rust_vk::auxillary::enums::DeviceExtension;
use rust_vk::auxillary::structs::{DeviceFeatures, DeviceInfo, MonitorInfo};
use rust_vk::instance::Instance;
use rust_vk::device::Device;
use rust_vk::pools::command::Pool as CommandPool;
use rust_vk::pools::memory::MetaPool;
use rust_win::spec::WindowInfo;
use semver::Version;
use winit::event_loop::EventLoop;
use winit::window::WindowId as WinitWindowId;

use game_pip::SquarePipeline;
use game_pip::spec::RenderPipeline;
use game_tgt::window::WindowTarget;

pub use crate::errors::RenderSystemError as Error;
use crate::spec::{AppInfo, VulkanInfo, WindowId};


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
/// The RenderSystem, which handles the (rasterized) rendering & windowing part of the game.
pub struct RenderSystem {
    /// The Entity Component System where the RenderSystem reads objects to render from.
    _ecs : Rc<RefCell<Ecs>>,

    /// The Instance on which this RenderSystem is based.
    _instance     : Rc<Instance>,
    /// The Device we'll use for rendering.
    device       : Rc<Device>,
    /// The CommandPool from which we allocate commands.
    _command_pool : Rc<RefCell<CommandPool>>,
    // /// The MemoryPool we use to allocate persistent buffers.
    _memory_pool  : Rc<RefCell<MetaPool>>,
    // /// The DescriptorPool from which we allocate descriptors.

    /// A list of all Windows. These are also referenced in the targets map.
    windows    : HashMap<WindowId, Rc<RefCell<WindowTarget>>>,
    /// Maps winit window IDs to our own semantic Window IDs.
    window_ids : HashMap<WinitWindowId, WindowId>,
    /// The map of render pipelines which we use to render to.
    pipelines  : HashMap<WindowId, Box<dyn RenderPipeline>>,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// Sets up the base RenderSystem, by registring components in the ECS and initializing the Vulkan backend.
    /// 
    /// This is only part of initializing the full RenderSystem; also initialize the relevant subsystems (see register()).
    /// 
    /// # Generic arguments
    /// - `T`: The type of the custom event in the given `event_loop`.
    /// 
    /// # Arguments
    /// - `ecs`: The ECS to register new components with.
    /// - `app_info`: The AppInfo struct that determines some application information.
    /// - `event_loop`: The EventLoop to use for triggering Window events and such.
    /// - `gpu`: The index of the GPU to use for rendering.
    /// - `window_mode`: The WindowMode of the Window.
    /// - `debug`: If true, enables the validation layers in the Vulkan backend.
    /// 
    /// # Returns
    /// A new instance of the RenderSystem on success.
    /// 
    /// # Errors
    /// This function throws errors whenever either the Instance or the Device failed to be created.
    pub fn new<T>(
        ecs: Rc<RefCell<Ecs>>,
        event_loop: &EventLoop<T>,
        app_info: AppInfo,
        window_info: WindowInfo,
        vulkan_info: VulkanInfo,
    ) -> Result<Self, Error> {
        // Register components
        /* TBD */



        // Create the instance
        let layers = if vulkan_info.debug {
            let mut layers = Vec::from(INSTANCE_LAYERS);
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            Vec::from(INSTANCE_LAYERS)
        };
        let instance = match Instance::new(app_info.name, app_info.version, app_info.engine_name, app_info.engine_version, INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Get the GPU
        let device = match Device::new(instance.clone(), vulkan_info.gpu, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
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



        // Build the main window
        let main_window: Rc<RefCell<WindowTarget>> = match WindowTarget::new(device.clone(), event_loop, window_info) {
            Ok(window) => Rc::new(RefCell::new(window)),
            Err(err)   => { return Err(Error::WindowCreateError{ err }); }
        };
        let main_window_id = main_window.borrow().window().id();

        // Initiate the map of windows
        let windows    : HashMap<WindowId, Rc<RefCell<WindowTarget>>> = HashMap::from([ (WindowId::Main, main_window) ]);
        let window_ids : HashMap<WinitWindowId, WindowId>             = HashMap::from([ (main_window_id, WindowId::Main) ]);

        // Initiate the render pipelines
        let mut pipelines: HashMap<WindowId, Box<dyn RenderPipeline>> = HashMap::with_capacity(1);
        pipelines.insert(WindowId::Main, match SquarePipeline::new(device.clone(), memory_pool.clone(), command_pool.clone(), windows[&WindowId::Main].clone(), 3) {
            Ok(pipeline) => Box::new(pipeline),
            Err(err)     => { return Err(Error::RenderPipelineCreateError{ name: "SquarePipeline", err }); }
        });



        // Use that to create the system
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            _ecs : ecs,

            _instance     : instance,
            device,
            _command_pool : command_pool,
            _memory_pool  : memory_pool,

            windows,
            window_ids,
            pipelines,
        })
    }



    /// Initiates a new render callback for all Windows.
    /// 
    /// Specifically, calls `Window::request_redraw()` for all of the RenderSystem's windows.
    /// 
    /// # Returns
    /// Nothing, but does launch new callbacks in the Event system.
    pub fn game_loop_complete(&self) {
        // Go through all of the windows
        for window in self.windows.values() {
            // Get a borrow on it
            let window: Ref<WindowTarget> = window.borrow();

            // Run the callback thingy
            window.window().request_redraw();
        }
    }

    /// Renders the given Window.
    /// 
    /// Based on the specific Window ID, renders multiple pipelines (or at least, schedules them).
    /// 
    /// # Arguments
    /// - `window_id`: The WindowID of the Window to render to.
    /// 
    /// # Errors
    /// This function may error if any of the to-be-rendered Windows failed _or_ if an interaction with Window's the swapchain failed.
    /// 
    /// # Panics
    /// This function panics if the given `window_id` does not exist.
    pub fn render_window(&mut self, window_id: WinitWindowId) -> Result<(), Error> {
        // Resolve the winit window ID
        let window_id = match self.window_ids.get(&window_id) {
            Some(id) => id,
            None     => { panic!("Unknown window ID '{:?}'", window_id); }
        };

        // Resolve the window ID to a pipeline
        let pipeline = match self.pipelines.get_mut(&window_id) {
            Some(pipeline) => pipeline,
            None           => { panic!("Unknown window ID '{}'", window_id); }
        };

        // This is the pipeline that we want to render
        match pipeline.render() {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::RenderError{ name: pipeline.name(), err }),
        }
    }

    /// Blocks the current thread until the Device is idle
    #[inline]
    pub fn wait_for_idle(&self) -> Result<(), Error> {
        match self.device.drain(None) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::IdleError{ err }),
        }
    }



    /// Automatically selects the best GPU.
    /// 
    /// Creates a new instance with the proper layers and extensions, and then tries to find the GPU with the best "CPU disconnectedness".
    /// 
    /// # Arguments
    /// - `debug`: If set to true, will take into account whether GPUs should support certain debug validation layers.
    /// 
    /// # Returns
    /// The index of the chosen GPU.
    /// 
    /// # Errors
    /// This function fails if the Instance failed to be created, if we could not query it for the available devices or if no device was found.
    pub fn auto_select(debug: bool) -> Result<usize, Error> {
        // Create the instance
        let layers = if debug {
            let mut layers = Vec::from(INSTANCE_LAYERS);
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            Vec::from(INSTANCE_LAYERS)
        };
        let instance = match Instance::new("Dummy Application", Version::new(0, 1, 0), "Dummy Engine", Version::new(0, 1, 0), &INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Call the list on the GPU class
        match Device::auto_select(instance, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(index) => Ok(index),
            Err(err)  => Err(Error::DeviceAutoSelectError{ err }),
        }
    }

    /// Lists all GPUs it can find.
    /// 
    /// Creates a new instance with the proper layers and extensions, and then sorts the GPUs into supported and non-supported.
    /// 
    /// # Arguments
    /// - `debug`: If set to true, will take into account whether GPUs should support certain debug validation layers to be considered supported.
    /// 
    /// # Returns
    /// A tuple of a supported (0) and unsupported (1) lists of GPUs. Each entry is a tuple itself of (index, name, kind).
    /// 
    /// # Errors
    /// This function fails if the Instance failed to be created or if we could not query it for the available devices.
    pub fn list_gpus(debug: bool) -> Result<(Vec<DeviceInfo>, Vec<DeviceInfo>), Error> {
        // Create the instance
        let layers = if debug {
            let mut layers = Vec::from(INSTANCE_LAYERS);
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            Vec::from(INSTANCE_LAYERS)
        };
        let instance = match Instance::new("Dummy Application", Version::new(0, 1, 0), "Dummy Engine", Version::new(0, 1, 0), &INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Call the list on the GPU class
        match Device::list(instance, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(result) => Ok(result),
            Err(err)   => Err(Error::DeviceListError{ err }),
        }
    }

    /// Lists all monitors it can find.
    /// 
    /// # Returns
    /// A list of all monitors, as MonitorInfos.
    /// 
    /// # Errors
    /// This function fails if the winit backend failed to enumerate the monitors.
    #[inline]
    pub fn list_monitors() -> Result<Vec<MonitorInfo>, Error> {
        // Simply run it super fast-o
        Ok(winit::event_loop::EventLoop::<()>::new().available_monitors().enumerate().map(|(i, monitor)| {
            MonitorInfo {
                index      : i,
                name       : monitor.name().unwrap_or(String::from("<unnamed monitor>")),
                resolution : monitor.size().into(),

                video_modes : monitor.video_modes().map(|v| v.into()).collect(),
            }
        }).collect())
    }
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        // Wait for the device to become idle first
        if let Err(_) = self.wait_for_idle() {}
    }
}
