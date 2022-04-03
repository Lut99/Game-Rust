/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   03 Apr 2022, 16:47:11
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base RenderSystem.
**/

use std::any::type_name;
use std::collections::HashMap;

use ash::vk;
use log::debug;
use semver::Version;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use game_ecs::Ecs;
use game_vk::gpu::Gpu;
use game_vk::instance::Instance;

pub use crate::errors::RenderSystemError as Error;
use crate::spec::{RenderTarget, RenderTargetBuilder, RenderTargetKind, RenderTargetStage};


/***** CONSTANTS *****/
/// The list of instance extensions we want to enable (besides the required surface ones).
const INSTANCE_EXTENSIONS: &[&str] = &[];

/// The list of instance layers we want to enable (besides the debug one).
const INSTANCE_LAYERS: &[&str] = &[];

/// The list of device extensions we want to enable.
const DEVICE_EXTENSIONS: &[&str] = &[ "VK_KHR_swapchain" ];

/// The list of device layers we want to enable.
const DEVICE_LAYERS: &[&str] = &[];

// Constants that are lazily loaded
lazy_static!{
    /// The list of device features we want to enable.
    static ref DEVICE_FEATURES: vk::PhysicalDeviceFeatures = Default::default();
}





/***** LIBRARY *****/
/// The RenderSystem, which handles the (rasterized) rendering & windowing part of the game.
pub struct RenderSystem {
    // NOTE: The order of the fields defined in the RenderSystem matters, as they are dropped in-order (and there is an interlinked dependency between them)

    /// The map of render targets to which we render.
    /// 
    /// It is a map of each target stage to each target, defined by a unique identifier.
    targets : HashMap<RenderTargetStage, HashMap<RenderTargetKind, HashMap<usize, Box<dyn RenderTarget>>>>,

    // /// The CommandPool from which we allocate commands.
    // /// The DescriptorPool from which we allocate descriptors.
    // /// The MemoryPool we use to allocate GPU-local buffers.
    // /// The MemoryPool we use to allocate CPU-accessible buffers.
    /// The Gpu we'll use for rendering.
    gpu      : Gpu,
    /// The Instance on which this RenderSystem is based.
    instance : Instance,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// Sets up the base RenderSystem, by registring components in the ECS and initializing the Vulkan backend.
    /// 
    /// This is only part of initializing the full RenderSystem; also initialize the relevant subsystems (see register()).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use semver::Version;
    /// use game_ecs::Ecs;
    /// use game_gfx::RenderSystem;
    /// 
    /// // Initialize the entity component system first
    /// let mut ecs = Ecs::default();
    /// 
    /// // Initialize the RenderSystem
    /// let render_system = RenderSystem::new(&mut ecs, "Hello World App", "Hello World Engine", Version::new(0, 1, 0), true)
    ///     .unwrap_or_else(|err| panic!("Failed to initialize base RenderSystem: {}", err));
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function throws errors whenever the Vulkan backend does.
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(ecs: &mut Ecs, name: S1, version: Version, engine: S2, engine_version: Version, gpu: usize, debug: bool) -> Result<Self, Error> {
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
        let gpu = match Gpu::new(&instance, gpu, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(gpu)  => gpu,
            Err(err) => { return Err(Error::GpuCreateError{ err }); }  
        };

        // Use that to create the system
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            instance,
            gpu,

            targets : HashMap::with_capacity(1),
        })
    }



    /// Registers a new render target.
    /// 
    /// Each Render Target is either a window or an image to render to. They are then in charge of their own pipeline(s), and will handle all resources that are derived from the Surface (or similar).
    /// 
    /// The Render Target may be called at different stages, which are defined in the RenderTargetStage enum.
    /// 
    /// Note that the most common-used target, the Window, is modular in itself in that is supports multiple RenderPipelines as backend that define the actual rendering done.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors if the given target could not be initialized properly.
    pub fn register<R, C>(&mut self, event_loop: &EventLoop<()>, id: usize, stage: RenderTargetStage, create_info: C) -> Result<(), Error> 
    where
        R: RenderTargetBuilder<CreateInfo=C>,
        C: Sized,
    {
        // Check if this ID already exists
        // Iterate through the different stages
        for targets in self.targets.values() {
            // Iterate through the different types
            for targets in targets.values() {
                // Iterate through the duplicates of those
                if targets.contains_key(&id) {
                    return Err(Error::DuplicateTarget{ type_name: std::any::type_name::<R>(), id });
                }
            }
        }

        // Simply call the constructor
        let target = match R::new(event_loop, &self.instance, &self.gpu, create_info) {
            Ok(target) => target,
            Err(err)   => { return Err(Error::RenderTargetCreateError{ type_name: std::any::type_name::<R>(), err: format!("{}", err) }); }
        };

        // First, add it in the global namespace
        let kind = target.kind();
        if let Some(targets) = self.targets.get_mut(&stage) {
            // Next, add it to the stage namespace
            if let Some(targets) = targets.get_mut(&kind) {
                // Add it to the list
                targets.insert(id, Box::new(target));

            } else {
                // Create the new HashMap for this type
                let mut type_targets: HashMap<usize, Box<dyn RenderTarget>> = HashMap::with_capacity(1);
                type_targets.insert(id, Box::new(target));

                // Insert the hashmap in the big one
                targets.insert(kind, type_targets);
            }
        } else {
            // Create the new HashMap for this stage
            let mut stage_targets: HashMap<RenderTargetKind, HashMap<usize, Box<dyn RenderTarget>>> = HashMap::with_capacity(1);

            // Create the new HashMap for this type
            let mut type_targets: HashMap<usize, Box<dyn RenderTarget>> = HashMap::with_capacity(1);
            type_targets.insert(id, Box::new(target));

            // Insert the hashmaps in the big ones
            stage_targets.insert(kind, type_targets);
            self.targets.insert(stage, stage_targets);
        }

        // Done!
        debug!("Registered new render target {} of type {} ({}) at stage {}", id, std::any::type_name::<R>(), kind, stage);
        Ok(())
    }



    /// Automatically selects the best GPU.
    /// 
    /// Creates a new instance with the proper layers and extensions, and then tries to find the GPU with the best "CPU disconnectedness".
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors if we could not connect to Vulkan, create the Instance, enumerate the physical devices or found no supported devices.
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
        match Gpu::auto_select(&instance, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(index) => Ok(index),
            Err(err)  => Err(Error::GpuAutoSelectError{ err }),
        }
    }

    /// Lists all GPUs it can find to stdout.
    /// 
    /// Creates a new instance with the proper layers and extensions, and then sorts the GPUs into supported and non-supported.
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// ```
    /// 
    /// # Errors
    /// This function errors if we could not connect to Vulkan, create the Instance or enumerate the physical devices.
    pub fn list(debug: bool) -> Result<(), Error> {
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
        match Gpu::list(&instance, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(())   => Ok(()),
            Err(err) => Err(Error::GpuListError{ err }),
        }
    }



    /// Handles events from winit's EventLoop.
    /// 
    /// This function will ignore any events that are non-relevant for the RenderSystem.
    /// 
    /// Note that this function returns the next control flow, as the user may want to quit the system if they close the game.
    /// 
    /// # Example
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function may error due to many different reasons, as this is also where the render pipelines are run.
    pub fn handle_events(&mut self, event: &Event<()>, control_flow: &ControlFlow) -> Result<ControlFlow, Error> {
        // Switch on the event type
        match event {
            | Event::WindowEvent{ window_id: _window_id, event } => {
                // Match the event again
                match event {
                    | WindowEvent::CloseRequested => {
                        // For now, we close on _any_ window close, but this should obviously be marginally more clever
                        Ok(ControlFlow::Exit)
                    },

                    // Ignore the others
                    _ => {
                        Ok(*control_flow)
                    }
                }
            },

            | Event::MainEventsCleared => {
                // Request a redraw of all internal windows
                let targets = match self.targets.get(&RenderTargetStage::MainLoop) {
                    Some(targets) => targets,
                    None          => { return Ok(*control_flow); }
                };
                let targets = match targets.get(&RenderTargetKind::Window) {
                    Some(targets) => targets,
                    None          => { return Ok(*control_flow); }
                };
                for target in targets.values() {
                    // Request the redraw
                    target.window_request_redraw();
                }

                // Done with this event
                Ok(*control_flow)
            },

            | Event::RedrawRequested(window_id) => {
                // Request a redraw of all internal windows
                let targets = match self.targets.get_mut(&RenderTargetStage::MainLoop) {
                    Some(targets) => targets,
                    None          => { return Ok(*control_flow); }
                };
                let targets = match targets.get_mut(&RenderTargetKind::Window) {
                    Some(targets) => targets,
                    None          => { return Ok(*control_flow); }
                };
                for target in targets.values_mut() {
                    // Call render only if the ID matches
                    if &target.window_id().expect("Iterating over Windows, but found non-Window; this should never happen!") == window_id {
                        if let Err(err) = target.render() {
                            return Err(Error::RenderError{ err });
                        };
                        return Ok(*control_flow);
                    }
                }

                // Done with this event
                Ok(*control_flow)
            },

            // We do nothing for all other events
            _ => {
                Ok(*control_flow)
            }
        }
    }



    /// Returns the render target with the given ID as the given type.
    /// 
    /// Will panic if the given ID does not exist or if the given type is not the one for this target.
    pub fn get_target<T: RenderTarget>(&self, id: usize) -> &T {
        // Get the target
        for targets in self.targets.values() {
            for targets in targets.values() {
                let target = match targets.get(&id) {
                    Some(target) => target,
                    None         => { continue; }
                };

                // Cast it down to the proper type, then return
                return target.as_any().downcast_ref::<T>().unwrap_or_else(|| panic!("RenderTarget with ID {} does not cast to a {}", id, type_name::<T>()));
            }
        }

        // We failed to find the target with the given ID
        panic!("RenderTarget with ID {} not found", id);
    }

    /// Returns the render target with the given ID as the given type.
    /// 
    /// Will panic if the given ID does not exist or if the given type is not the one for this target.
    pub fn get_target_mut<T: RenderTarget>(&mut self, id: usize) -> &mut T {
        // Get the target
        for targets in self.targets.values_mut() {
            for targets in targets.values_mut() {
                let target = match targets.get_mut(&id) {
                    Some(target) => target,
                    None         => { continue; }
                };

                // Cast it down to the proper type
                return target.as_any_mut().downcast_mut::<T>().unwrap_or_else(|| panic!("RenderTarget with ID {} does not cast to a {}", id, type_name::<T>()));
            }
        }

        // We failed to find the target with the given ID
        panic!("RenderTarget with ID {} not found", id);
    }
}
