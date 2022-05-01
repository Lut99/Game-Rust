/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   01 May 2022, 12:31:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base RenderSystem.
**/

use std::any::type_name;
use std::collections::HashMap;
use std::sync::Arc;

use ash::vk;
use log::debug;
use semver::Version;

use game_ecs::Ecs;
use game_vk::auxillary::DeviceKind;
use game_vk::instance::Instance;
use game_vk::device::Device;

pub use crate::errors::RenderSystemError as Error;
use crate::spec::{RenderPipeline, RenderPipelineBuilder, RenderPipelineId, RenderTarget, RenderTargetBuilder, RenderTargetId};


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
    /// The Instance on which this RenderSystem is based.
    instance : Arc<Instance>,
    /// The Device we'll use for rendering.
    device   : Arc<Device>,
    // /// The MemoryPool we use to allocate CPU-accessible buffers.
    // /// The MemoryPool we use to allocate GPU-local buffers.
    // /// The DescriptorPool from which we allocate descriptors.
    // /// The CommandPool from which we allocate commands.

    /// The last-used RenderTargetId
    last_target_id   : RenderTargetId,
    /// The last-used RenderPipelineId
    last_pipeline_id : RenderPipelineId,
    /// The map of render targets to which we render.
    targets          : HashMap<RenderTargetId, Box<dyn RenderTarget>>,
    /// The map of render pipelines which we use to render to.
    pipelines        : HashMap<RenderPipelineId, Box<dyn RenderPipeline>>,
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
    /// - `gpu`: The index of the GPU to use for rendering.
    /// - `debug`: If true, enables the validation layers in the Vulkan backend.
    /// 
    /// # Returns
    /// A new instance of the RenderSystem on success.
    /// 
    /// # Errors
    /// This function throws errors whenever either the Instance or the Device failed to be created.
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
        let device = match Device::new(instance.clone(), gpu, DEVICE_EXTENSIONS, DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(device) => device,
            Err(err)   => { return Err(Error::DeviceCreateError{ err }); }  
        };

        // Use that to create the system
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            instance : instance,
            device   : device,

            last_target_id   : RenderTargetId::new(),
            last_pipeline_id : RenderPipelineId::new(),
            targets          : HashMap::with_capacity(1),
            pipelines        : HashMap::with_capacity(1),
        })
    }



    /// Registers a new render target.
    /// 
    /// Each RenderTarget is responsible for producing some image that may be rendered to. Then, in the present() step, it is also responsible for somehow getting the result back to the user.
    /// 
    /// # Arguments
    /// - `create_info`: The RenderTarget-specific CreateInfo to pass arguments to its constructor.
    /// 
    /// # Returns
    /// An identifier to reference the newly added target later on.
    /// 
    /// # Errors
    /// This function errors if the given target could not be initialized properly.
    pub fn register_target<'a, R, C>(&mut self, create_info: C) -> Result<RenderTargetId, Error> 
    where
        R: RenderTargetBuilder<'a, CreateInfo=C>,
        C: Sized,
    {
        // Generate a new ID for this RenderTarget
        let id = self.last_target_id.increment();
        
        // Call the constructor
        let target = match R::new(self.device.clone(), create_info) {
            Ok(target) => target,
            Err(err)   => { return Err(Error::RenderTargetCreateError{ type_name: std::any::type_name::<R>(), err: format!("{}", err) }); }
        };

        // Add it in the map
        self.targets.insert(id, Box::new(target));

        // Return the ID
        debug!("Registered new render target of type {} as ID {}", std::any::type_name::<R>(), id);
        Ok(id)
    }

    /// Registers a new render pipeline.
    /// 
    /// Each RenderPipeline is responsible for taking vertices and junk and outputting that to a RenderTarget.
    /// 
    /// # Arguments
    /// - `target`: The ID of the render target where this pipeline will render to.
    /// - `create_info`: The RenderPipeline-specific CreateInfo to pass arguments to its constructor.
    /// 
    /// # Returns
    /// An identifier to reference the newly added target later on.
    /// 
    /// # Errors
    /// This function errors if the given target could not be initialized properly.
    pub fn register_pipeline<'a, R, C>(&mut self, target: RenderTargetId, create_info: C) -> Result<RenderPipelineId, Error> 
    where
        R: RenderPipelineBuilder<'a, CreateInfo=C>,
        C: Sized,
    {
        // Try to get the referenced render target
        let target: &dyn RenderTarget = self.targets.get(&target).unwrap_or_else(|| panic!("Given RenderTargetId '{}' is not registered", target)).as_ref();

        // Generate a new ID for this RenderTarget
        let id = self.last_pipeline_id.increment();

        // Call the constructor
        let pipeline = match R::new(self.device.clone(), target, create_info) {
            Ok(pipeline) => pipeline,
            Err(err)     => { return Err(Error::RenderPipelineCreateError{ type_name: std::any::type_name::<R>(), err: format!("{}", err) }); }
        };

        // Add it in the map
        self.pipelines.insert(id, Box::new(pipeline));

        // Return the ID
        debug!("Registered new render pipeline of type {} as ID {}", std::any::type_name::<R>(), id);
        Ok(id)
    }



    /// Performs a single render pass using the given pipeline, rendering to the given target.
    /// 
    /// # Arguments
    /// - `target`: The UID of the target to render to.
    /// - `pipeline`: The UID of the pipeline to render to.
    /// 
    /// # Returns
    /// Nothing on success, except that the RenderTarget should have new pixels drawn to it.
    /// 
    /// # Errors
    /// This function errors if the given Pipeline errors.
    pub fn render(target: RenderTargetId, pipeline: RenderPipelineId) -> Result<(), Error> {
        Ok(())
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
    pub fn list(debug: bool) -> Result<(Vec<(usize, String, DeviceKind)>, Vec<(usize, String, DeviceKind)>), Error> {
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



    // /// Handles events from winit's EventLoop.
    // /// 
    // /// This function will ignore any events that are non-relevant for the RenderSystem.
    // /// 
    // /// # Arguments
    // /// - `event`: The Event that was just fired.
    // /// - `control_flow`: The previous ControlFlow. Might be overridden if the user chose to close the Window(s) we render to.
    // /// 
    // /// # Returns
    // /// The next ControlFlow. Will likely be the one passed, unless overriden somehow.
    // /// 
    // /// # Errors
    // /// Because this function also performs render calls, it may error due to many different reasons.
    // pub fn handle_events(&mut self, event: &Event<()>, control_flow: &ControlFlow) -> Result<ControlFlow, Error> {
    //     // Switch on the event type
    //     match event {
    //         | Event::WindowEvent{ window_id: _window_id, event } => {
    //             // Match the event again
    //             match event {
    //                 | WindowEvent::CloseRequested => {
    //                     // For now, we close on _any_ window close, but this should obviously be marginally more clever
    //                     Ok(ControlFlow::Exit)
    //                 },

    //                 // Ignore the others
    //                 _ => {
    //                     Ok(*control_flow)
    //                 }
    //             }
    //         },

    //         | Event::MainEventsCleared => {
    //             // Request a redraw of all internal windows
    //             for target in self.targets.values() {
    //                 // Request the redraw; if it's not a Window target, then this function will take care of it
    //                 target.window_request_redraw();
    //             }

    //             // Done with this event
    //             Ok(*control_flow)
    //         },

    //         | Event::RedrawRequested(window_id) => {
    //             // Request a redraw of all internal windows
    //             for target in self.targets.values_mut() {
    //                 // Call render only if the ID matches
    //                 if &target.window_id().expect("Iterating over Windows, but found non-Window; this should never happen!") == window_id {
    //                     if let Err(err) = target.render() {
    //                         return Err(Error::RenderError{ err });
    //                     };
    //                     return Ok(*control_flow);
    //                 }
    //             }

    //             // Done with this event
    //             Ok(*control_flow)
    //         },

    //         // We do nothing for all other events
    //         _ => {
    //             Ok(*control_flow)
    //         }
    //     }
    // }



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
