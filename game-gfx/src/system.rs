//  SYSTEM.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 18:07:31
//  Last edited:
//    06 Aug 2022, 20:38:49
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the base RenderSystem.
// 

use std::any::type_name;
use std::cell::RefCell;
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
use rust_vk::sync::{Fence, Semaphore};
use rust_win::Window;
use rust_win::spec::{WindowInfo, WindowMode};
use semver::Version;
use winit::event_loop::EventLoop;
use winit::window::WindowId as WinitWindowId;

use game_tgt::RenderTarget;

pub use crate::errors::RenderSystemError as Error;
use crate::spec::{RenderPipeline, RenderPipelineId, WindowId};
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
/// The RenderSystem, which handles the (rasterized) rendering & windowing part of the game.
pub struct RenderSystem {
    /// The Entity Component System where the RenderSystem reads objects to render from.
    ecs : Rc<RefCell<Ecs>>,

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
    windows   : HashMap<WindowId, Rc<RefCell<Window>>>,
    /// The map of render pipelines which we use to render to.
    pipelines : HashMap<RenderPipelineId, Box<dyn RenderPipeline>>,

    /// The Semaphores that signal when an image is ready (one per image that is in-flight).
    image_ready  : Vec<Rc<Semaphore>>,
    /// The Semaphores that signal when a frame is done being rendered (one per image that is in-flight).
    render_ready : Vec<Rc<Semaphore>>,
    /// The number of images that are in-flight (one per image that is in-flight).
    in_flight    : Vec<Rc<Fence>>,
    /// The current frame we render from the in-flight frames.
    current_frame : usize,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// Sets up the base RenderSystem, by registring components in the ECS and initializing the Vulkan backend.
    /// 
    /// This is only part of initializing the full RenderSystem; also initialize the relevant subsystems (see register()).
    /// 
    /// # Generic arguments
    /// - `S1`: The &str-like type of the application's name.
    /// - `S2`: The &str-like type of the application's engine's name.
    /// 
    /// # Arguments
    /// - `ecs`: The ECS to register new components with.
    /// - `name`: The name of the application to register in the Vulkan driver.
    /// - `version`: The version of the application to register in the Vulkan driver.
    /// - `engine_name`: The name of the application's engine to register in the Vulkan driver.
    /// - `engine_version`: The version of the application's engine to register in the Vulkan driver.
    /// - `event_loop`: The EventLoop to use for triggering Window events and such.
    /// - `gpu`: The index of the GPU to use for rendering.
    /// - `window_mode`: The WindowMode of the Window.
    /// - `targets_in_flight`: The maximum number of frames that are in-flight while rendering.
    /// - `debug`: If true, enables the validation layers in the Vulkan backend.
    /// 
    /// # Returns
    /// A new instance of the RenderSystem on success.
    /// 
    /// # Errors
    /// This function throws errors whenever either the Instance or the Device failed to be created.
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(
        ecs: Rc<RefCell<Ecs>>,
        name: S1, version: Version,
        engine: S2, engine_version: Version,
        event_loop: &EventLoop<()>,
        gpu: usize, window_mode: WindowMode,
        targets_in_flight: usize,
        debug: bool
    ) -> Result<Self, Error> {
        // Register components
        /* TBD */



        // Create the instance
        let layers = if debug {
            let mut layers = Vec::from(INSTANCE_LAYERS);
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            Vec::from(INSTANCE_LAYERS)
        };
        let instance = match Instance::new(name, version, engine, engine_version, INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Get the GPU
        let device = match Device::new(instance.clone(), gpu, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
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
        let main_window: Rc<RefCell<Window>> = match Window::new(device.clone(), event_loop, WindowInfo{ title: format!("Game-Rust v{}", env!("CARGO_PKG_VERSION")), window_mode }, 3) {
            Ok(window) => Rc::new(RefCell::new(window)),
            Err(err)   => { return Err(Error::WindowCreateError{ err }); }
        };

        // Initiate the map of windows
        let windows: HashMap<WindowId, Rc<RefCell<Window>>> = HashMap::with_capacity(1);
        windows.insert(WindowId::Main(main_window.borrow().id()), main_window);

        // Initiate the render pipelines
        let mut pipelines: HashMap<RenderPipelineId, Box<dyn RenderPipeline>> = HashMap::with_capacity(1);
        pipelines.insert(RenderPipelineId::Triangle, match pipelines::TrianglePipeline::new(device.clone(), memory_pool.clone(), command_pool.clone(), windows[&WindowId::Main].clone()) {
            Ok(pipeline) => Box::new(pipeline),
            Err(err)     => { return Err(Error::RenderPipelineCreateError{ name: "TrianglePipeline", err }); }
        });



        // Prepare the necessary synchronization primitives
        let mut image_ready: Vec<Rc<Semaphore>>  = Vec::with_capacity(targets_in_flight);
        let mut render_ready: Vec<Rc<Semaphore>> = Vec::with_capacity(targets_in_flight);
        let mut in_flight: Vec<Rc<Fence>>        = Vec::with_capacity(targets_in_flight);
        for _ in 0..targets_in_flight {
            // Add each of the primitives
            image_ready.push(match Semaphore::new(device.clone()) {
                Ok(semaphore) => semaphore,
                Err(err)      => { return Err(Error::SemaphoreCreateError{ err }); }
            });
            render_ready.push(match Semaphore::new(device.clone()) {
                Ok(semaphore) => semaphore,
                Err(err)      => { return Err(Error::SemaphoreCreateError{ err }); }
            });
            in_flight.push(match Fence::new(device.clone(), true) {
                Ok(fence) => fence,
                Err(err)  => { return Err(Error::FenceCreateError{ err }); }
            });
        }



        // Use that to create the system
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            ecs,

            _instance     : instance,
            device,
            _command_pool : command_pool,
            _memory_pool  : memory_pool,

            windows,
            pipelines,

            image_ready,
            render_ready,
            in_flight,
            current_frame : 0,
        })
    }



    // /// Performs a single render pass using the given pipeline, rendering to the given target.
    // /// 
    // /// # Arguments
    // /// - `pipeline`: The UID of the pipeline to render to.
    // /// - `target`: The UID of the target to render to.
    // /// 
    // /// # Returns
    // /// Nothing on success, except that the RenderTarget should have new pixels drawn to it.
    // /// 
    // /// # Errors
    // /// This function errors if the given Pipeline errors.
    // pub fn render(&mut self, pipeline_id: RenderPipelineId) -> Result<(), Error> {
    //     // If the next fence is not yet available, early quit
    //     match self.in_flight[self.current_frame].poll() {
    //         Ok(res)  => if !res { return Ok(()); },
    //         Err(err) => { return Err(Error::FencePollError{ err }) }
    //     };

    //     // Fetch the target RenderTarget

    //     // Fetch the RenderTarget and the RenderPipeline for this render call
    //     let target: &mut dyn RenderTarget     = self.targets.get_mut(&target_id).unwrap_or_else(|| panic!("RenderTarget '{}' is not registered in the RenderSystem", target_id)).as_mut();
    //     let pipeline: &mut dyn RenderPipeline = self.pipelines.get_mut(&pipeline_id).unwrap_or_else(|| panic!("RenderPipeline '{}' is not registered in the RenderSystem", pipeline_id)).as_mut();

    //     // Get the next image index from the render target
    //     let frame_index: usize = match target.get_index(Some(&self.image_ready[self.current_frame])) {
    //         Ok(Some(index)) => index,
    //         Ok(None)        => {
    //             // Get the new size from the target
    //             let new_size = target.real_extent();
    //             // If it's zero, then skip and wait until the window has a valid size again
    //             if new_size.w == 0 && new_size.h == 0 { return Ok(()); }

    //             // Rebuild the target and then the window
    //             debug!("Resizing {} and {} to: {}", target_id, pipeline_id, new_size);
    //             if let Err(err) = target.rebuild(&new_size) { return Err(Error::TargetRebuildError{ id: target_id, err }); }
    //             if let Err(err) = pipeline.rebuild(target) { return Err(Error::PipelineRebuildError{ id: pipeline_id, err }); }

    //             // Simply go through it again to do the proper render call
    //             return self.render(pipeline_id, target_id);
    //         },
    //         Err(err) => { return Err(Error::TargetGetIndexError{ err }); },
    //     };

    //     // Tell the pipeline to render
    //     if let Err(err) = pipeline.render(frame_index, &[&self.image_ready[self.current_frame]], &[&self.render_ready[self.current_frame]], &self.in_flight[self.current_frame]) {
    //         return Err(Error::RenderError{ err });
    //     }

    //     // Even though the frame is not being rendered and such, schedule its presentation
    //     match target.present(frame_index, &[&self.render_ready[self.current_frame]]) {
    //         Ok(_)    => Ok(()),
    //         Err(err) => Err(Error::PresentError{ err }),
    //     }
    // }

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
    pub fn render_window(&self, window_id: WinitWindowId) -> Result<(), Error> {
        // If the next fence is not yet available, early quit
        match self.in_flight[self.current_frame].poll() {
            Ok(res)  => if !res { return Ok(()); },
            Err(err) => { return Err(Error::FencePollError{ err }) }
        };

        // Match the window ID so we know how to render
        match WindowId::from(window_id) {
            
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



    /// Returns a(n immuteable) reference to the RenderTarget with the given ID.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderTarget to return.
    /// 
    /// # Returns
    /// The RenderTarget uncasted (so still as a RenderTarget trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown.
    #[inline]
    pub fn get_target(&self, id: RenderTargetId) -> &dyn RenderTarget {
        match self.targets.get(&id) {
            Some(target) => target.as_ref(),
            None         => { panic!("Unknown RenderTarget {}", id); }
        }
    }

    /// Returns a (muteable) reference to the RenderTarget with the given ID.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderTarget to return.
    /// 
    /// # Returns
    /// The RenderTarget uncasted (so still as a RenderTarget trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown.
    #[inline]
    pub fn get_target_mut(&mut self, id: RenderTargetId) -> &mut dyn RenderTarget {
        match self.targets.get_mut(&id) {
            Some(target) => target.as_mut(),
            None         => { panic!("Unknown RenderTarget {}", id); }
        }
    }

    /// Returns a(n immuteable) reference to the RenderTarget with the given ID.
    /// 
    /// This function also casts the given RenderTarget to the given type.
    /// 
    /// # Generic types
    /// - `Target`: The Type to cast to.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderTarget to return.
    /// 
    /// # Returns
    /// The RenderTarget uncasted (so still as a RenderTarget trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown or the cast failed.
    #[inline]
    pub fn get_target_as<Target: RenderTarget>(&self, id: RenderTargetId) -> &Target {
        match self.targets.get(&id) {
            Some(target) => {
                target.as_any().downcast_ref::<Target>().unwrap_or_else(|| panic!("Could not downcast RenderTarget to {}", type_name::<Target>()))
            },
            None => { panic!("Unknown RenderTarget {}", id); }
        }
    }

    /// Returns a (muteable) reference to the RenderTarget with the given ID.
    /// 
    /// This function also casts the given RenderTarget to the given type.
    /// 
    /// # Generic types
    /// - `Target`: The Type to cast to.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderTarget to return.
    /// 
    /// # Returns
    /// The RenderTarget uncasted (so still as a RenderTarget trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown or the cast failed.
    #[inline]
    pub fn get_target_as_mut<Target: RenderTarget>(&mut self, id: RenderTargetId) -> &mut Target {
        match self.targets.get_mut(&id) {
            Some(target) => {
                target.as_any_mut().downcast_mut::<Target>().unwrap_or_else(|| panic!("Could not downcast RenderTarget to {}", type_name::<Target>()))
            },
            None => { panic!("Unknown RenderTarget {}", id); }
        }
    }



    /// Returns a(n immuteable) reference to the RenderPipeline with the given ID.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderPipeline to return.
    /// 
    /// # Returns
    /// The RenderPipeline uncasted (so still as a RenderPipeline trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown.
    #[inline]
    pub fn get_pipeline(&self, id: RenderPipelineId) -> &dyn RenderPipeline {
        match self.pipelines.get(&id) {
            Some(pipeline) => pipeline.as_ref(),
            None           => { panic!("Unknown RenderPipeline {}", id); }
        }
    }

    /// Returns a (muteable) reference to the RenderPipeline with the given ID.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderPipeline to return.
    /// 
    /// # Returns
    /// The RenderPipeline uncasted (so still as a RenderPipeline trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown.
    #[inline]
    pub fn get_pipeline_mut(&mut self, id: RenderPipelineId) -> &mut dyn RenderPipeline {
        match self.pipelines.get_mut(&id) {
            Some(pipeline) => pipeline.as_mut(),
            None           => { panic!("Unknown RenderPipeline {}", id); }
        }
    }

    /// Returns a(n immuteable) reference to the RenderPipeline with the given ID.
    /// 
    /// This function also casts the given RenderPipeline to the given type.
    /// 
    /// # Generic types
    /// - `Target`: The Type to cast to.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderPipeline to return.
    /// 
    /// # Returns
    /// The RenderPipeline uncasted (so still as a RenderPipeline trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown or the cast failed.
    #[inline]
    pub fn get_pipeline_as<Target: RenderPipeline>(&self, id: RenderPipelineId) -> &Target {
        match self.pipelines.get(&id) {
            Some(pipeline) => {
                pipeline.as_any().downcast_ref::<Target>().unwrap_or_else(|| panic!("Could not downcast RenderPipeline to {}", type_name::<Target>()))
            },
            None => { panic!("Unknown RenderPipeline {}", id); }
        }
    }

    /// Returns a (muteable) reference to the RenderPipeline with the given ID.
    /// 
    /// This function also casts the given RenderPipeline to the given type.
    /// 
    /// # Generic types
    /// - `Target`: The Type to cast to.
    /// 
    /// # Arguments
    /// - `id`: The ID of the RenderPipeline to return.
    /// 
    /// # Returns
    /// The RenderPipeline uncasted (so still as a RenderPipeline trait).
    /// 
    /// # Errors
    /// This function does not error explicitly, but does panic if the ID is unknown or the cast failed.
    #[inline]
    pub fn get_pipeline_as_mut<Target: RenderPipeline>(&mut self, id: RenderPipelineId) -> &mut Target {
        match self.pipelines.get_mut(&id) {
            Some(pipeline) => {
                pipeline.as_any_mut().downcast_mut::<Target>().unwrap_or_else(|| panic!("Could not downcast RenderPipeline to {}", type_name::<Target>()))
            },
            None => { panic!("Unknown RenderPipeline {}", id); }
        }
    }
}
