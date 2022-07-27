/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   27 Jul 2022, 14:26:26
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base RenderSystem.
**/

use std::any::type_name;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use log::debug;
use semver::Version;
use winit::event_loop::EventLoop;

use game_cfg::spec::WindowMode;
use game_ecs::{Ecs, Entity};
use game_vk::auxillary::enums::DeviceExtension;
use game_vk::auxillary::structs::{DeviceFeatures, DeviceInfo, MonitorInfo};
use game_vk::instance::Instance;
use game_vk::device::Device;
use game_vk::pools::command::Pool as CommandPool;
use game_vk::pools::memory::MetaPool;
use game_vk::sync::{Fence, Semaphore};

pub use crate::errors::RenderSystemError as Error;
use crate::errors::PipelineError;
use crate::spec::{RenderPipeline, RenderPipelineId};
use crate::components;
use crate::window;
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
    /// The Entity Component System where the RenderSystem stores some of its data.
    _ecs : Rc<Ecs>,

    /// The Instance on which this RenderSystem is based.
    _instance     : Rc<Instance>,
    /// The Device we'll use for rendering.
    device       : Rc<Device>,
    // /// The MemoryPool we use to allocate CPU-accessible buffers.
    // /// The MemoryPool we use to allocate GPU-local buffers.
    // /// The DescriptorPool from which we allocate descriptors.
    /// The CommandPool from which we allocate commands.
    _command_pool : Arc<RwLock<CommandPool>>,

    /// The map of render pipelines which we use to render to.
    pipelines : HashMap<RenderPipelineId, Box<dyn RenderPipeline>>,
    /// Set of windows known to the RenderSystem.
    _windows  : Vec<Entity>,

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
        ecs: Rc<Ecs>,
        name: S1, version: Version,
        engine: S2, engine_version: Version,
        event_loop: &EventLoop<()>,
        gpu: usize, window_mode: WindowMode,
        targets_in_flight: usize,
        debug: bool
    ) -> Result<Self, Error> {
        let mut ecs = ecs;



        // Register components
        Ecs::register::<components::Window>(&mut ecs);
        Ecs::register::<components::Target>(&mut ecs);



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

        // Allocate the memory pools on the GPU
        let memory_pool = MetaPool::new(device.clone(), 4096);

        // Allocate the pools on the GPU
        let command_pool = match CommandPool::new(device.clone()) {
            Ok(pool) => pool,
            Err(err) => { return Err(Error::CommandPoolCreateError{ err }); }
        };



        // Create a Window
        let win = match window::create(&ecs, device.clone(), event_loop, "Game-Rust", window_mode) {
            Ok(win)  => win,
            Err(err) => { return Err(Error::WindowCreateError{ title: "Game-Rust".into(), err }); } 
        };

        // Initiate the render pipeline around that window
        let mut pipelines: HashMap<RenderPipelineId, Box<dyn RenderPipeline>> = HashMap::with_capacity(1);
        pipelines.insert(RenderPipelineId::Triangle, match pipelines::TrianglePipeline::new(ecs.clone(), device.clone(), memory_pool.clone(), command_pool.clone(), win, targets_in_flight) {
            Ok(pipeline) => Box::new(pipeline),
            Err(err)     => { return Err(Error::RenderPipelineCreateError{ name: "TrianglePipeline", err: Box::new(err) }); }
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
            _ecs : ecs,

            _instance     : instance,
            device,
            _command_pool : command_pool,

            _windows : vec![ win ],
            pipelines,

            render_ready,
            in_flight,
            current_frame : 0,
        })
    }



    /// Performs a single render pass using the given pipeline, rendering to the given target.
    /// 
    /// # Arguments
    /// - `pipeline`: The UID of the pipeline to render to.
    /// - `target`: The UID of the target to render to.
    /// 
    /// # Returns
    /// Nothing on success, except that the RenderTarget should have new pixels drawn to it.
    /// 
    /// # Errors
    /// This function errors if the given Pipeline errors.
    pub fn render(&mut self, pipeline_id: RenderPipelineId) -> Result<(), Error> {
        // If the next fence is not yet available, early quit
        match self.in_flight[self.current_frame].poll() {
            Ok(res)  => if !res { return Ok(()); },
            Err(err) => { return Err(Error::FencePollError{ err }) }
        };

        // Fetch the RenderPipeline for this render call
        let pipeline: &mut dyn RenderPipeline = self.pipelines.get_mut(&pipeline_id).unwrap_or_else(|| panic!("RenderPipeline '{}' is not registered in the RenderSystem", pipeline_id)).as_mut();

        // Tell the pipeline to render
        match pipeline.render(self.current_frame, &[], &[&self.render_ready[self.current_frame]], &self.in_flight[self.current_frame]) {
            Ok(_)                                      => {},
            Err(PipelineError::SwapchainRebuildNeeded) => {
                // Hit the pipeline function
                if let Err(err) = pipeline.rebuild() { return Err(Error::PipelineRebuildError{ id: pipeline_id, err }); }

                // Simply go through it again to do the proper render call
                return self.render(pipeline_id);
            },
            Err(err) => { return Err(Error::RenderError{ err }); }
        }

        // If rendering has been scheduled successfully, schedule it for presentation
        if let Err(err) = pipeline.present(self.current_frame, &[&self.render_ready[self.current_frame]]) {
            return Err(Error::PresentError{ err });
        }

        // We're done processing this frame; move to the next
        self.current_frame += 1;
        Ok(())
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
