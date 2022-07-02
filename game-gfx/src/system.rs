/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   02 Jul 2022, 14:05:27
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

use game_ecs::Ecs;
use game_vk::auxillary::{DeviceExtension, DeviceFeatures, DeviceInfo};
use game_vk::instance::Instance;
use game_vk::device::Device;
use game_vk::pools::command::Pool as CommandPool;
use game_vk::sync::{Fence, Semaphore};

pub use crate::errors::RenderSystemError as Error;
use crate::spec::{RenderPipeline, RenderPipelineId, RenderTarget, RenderTargetId};
use crate::targets;
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
    /// The Instance on which this RenderSystem is based.
    _instance     : Rc<Instance>,
    /// The Device we'll use for rendering.
    device       : Rc<Device>,
    // /// The MemoryPool we use to allocate CPU-accessible buffers.
    // /// The MemoryPool we use to allocate GPU-local buffers.
    // /// The DescriptorPool from which we allocate descriptors.
    /// The CommandPool from which we allocate commands.
    _command_pool : Arc<RwLock<CommandPool>>,

    /// The map of render targets to which we render.
    targets   : HashMap<RenderTargetId, Box<dyn RenderTarget>>,
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
    /// - `targets_in_flight`: The maximum number of frames that are in-flight while rendering.
    /// - `debug`: If true, enables the validation layers in the Vulkan backend.
    /// 
    /// # Returns
    /// A new instance of the RenderSystem on success.
    /// 
    /// # Errors
    /// This function throws errors whenever either the Instance or the Device failed to be created.
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(_ecs: &mut Ecs, name: S1, version: Version, engine: S2, engine_version: Version, event_loop: &EventLoop<()>, gpu: usize, targets_in_flight: usize, debug: bool) -> Result<Self, Error> {
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

        // Allocate the memory pools on the GPU
        // let pool = LinearPool::new(device.clone());

        // Allocate the pools on the GPU
        let command_pool = match CommandPool::new(device.clone()) {
            Ok(pool) => pool,
            Err(err) => { return Err(Error::CommandPoolCreateError{ err }); }
        };



        // Initiate the render targets
        let mut targets: HashMap<RenderTargetId, Box<dyn RenderTarget>> = HashMap::with_capacity(1);
        targets.insert(RenderTargetId::TriangleWindow, match targets::Window::new(device.clone(), event_loop, "Game-Rust - Triangle", 800, 600, 3) {
            Ok(target) => Box::new(target),
            Err(err)   => { return Err(Error::RenderTargetCreateError{ name: "Window", err: Box::new(err) }); } 
        });
    

    
        // Initiate the render pipelines
        let mut pipelines: HashMap<RenderPipelineId, Box<dyn RenderPipeline>> = HashMap::with_capacity(1);
        pipelines.insert(RenderPipelineId::Triangle, match pipelines::TrianglePipeline::new(device.clone(), targets.get(&RenderTargetId::TriangleWindow).unwrap().as_ref(), command_pool.clone()) {
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
            _instance     : instance,
            device,
            _command_pool : command_pool,

            targets,
            pipelines,

            image_ready,
            render_ready,
            in_flight,
            current_frame : 0,
        })
    }



    // /// Registers a new render target.
    // /// 
    // /// Each RenderTarget is responsible for producing some image that may be rendered to. Then, in the present() step, it is also responsible for somehow getting the result back to the user.
    // /// 
    // /// # Arguments
    // /// - `create_info`: The RenderTarget-specific CreateInfo to pass arguments to its constructor.
    // /// 
    // /// # Returns
    // /// An identifier to reference the newly added target later on.
    // /// 
    // /// # Errors
    // /// This function errors if the given target could not be initialized properly.
    // pub fn register_target<'a, R, C>(&mut self, create_info: C) -> Result<RenderTargetId, Error> 
    // where
    //     R: RenderTargetBuilder<'a, CreateInfo=C>,
    //     C: Sized,
    // {
    //     // Generate a new ID for this RenderTarget
    //     let id = self.last_target_id.increment();
        
    //     // Call the constructor
    //     let target = match R::new(self.device.clone(), create_info) {
    //         Ok(target) => target,
    //         Err(err)   => { return Err(Error::RenderTargetCreateError{ type_name: std::any::type_name::<R>(), err: format!("{}", err) }); }
    //     };

    //     // Add it in the map
    //     self.targets.insert(id, Box::new(target));

    //     // Return the ID
    //     debug!("Registered new render target of type {} as ID {}", std::any::type_name::<R>(), id);
    //     Ok(id)
    // }

    // /// Registers a new render pipeline.
    // /// 
    // /// Each RenderPipeline is responsible for taking vertices and junk and outputting that to a RenderTarget.
    // /// 
    // /// # Arguments
    // /// - `target`: The ID of the render target where this pipeline will render to.
    // /// - `create_info`: The RenderPipeline-specific CreateInfo to pass arguments to its constructor.
    // /// 
    // /// # Returns
    // /// An identifier to reference the newly added target later on.
    // /// 
    // /// # Errors
    // /// This function errors if the given target could not be initialized properly.
    // pub fn register_pipeline<'a, R, C>(&mut self, target: RenderTargetId, create_info: C) -> Result<RenderPipelineId, Error> 
    // where
    //     R: RenderPipelineBuilder<'a, CreateInfo=C>,
    //     C: Sized,
    // {
    //     // Try to get the referenced render target
    //     let target: &dyn RenderTarget = self.targets.get(&target).unwrap_or_else(|| panic!("Given RenderTargetId '{}' is not registered", target)).as_ref();

    //     // Generate a new ID for this RenderTarget
    //     let id = self.last_pipeline_id.increment();

    //     // Call the constructor
    //     let pipeline = match R::new(self.device.clone(), target, self.command_pool.clone(), create_info) {
    //         Ok(pipeline) => pipeline,
    //         Err(err)     => { return Err(Error::RenderPipelineCreateError{ type_name: std::any::type_name::<R>(), err: format!("{}", err) }); }
    //     };

    //     // Add it in the map
    //     self.pipelines.insert(id, Box::new(pipeline));

    //     // Return the ID
    //     debug!("Registered new render pipeline of type {} as ID {}", std::any::type_name::<R>(), id);
    //     Ok(id)
    // }



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
    pub fn render(&mut self, pipeline_id: RenderPipelineId, target_id: RenderTargetId) -> Result<(), Error> {
        // If the next fence is not yet available, early quit
        match self.in_flight[self.current_frame].poll() {
            Ok(res)  => if !res { return Ok(()); },
            Err(err) => { return Err(Error::FencePollError{ err }) }
        };

        // Fetch the RenderTarget and the RenderPipeline for this render call
        let target: &mut dyn RenderTarget     = self.targets.get_mut(&target_id).unwrap_or_else(|| panic!("RenderTarget '{}' is not registered in the RenderSystem", target_id)).as_mut();
        let pipeline: &mut dyn RenderPipeline = self.pipelines.get_mut(&pipeline_id).unwrap_or_else(|| panic!("RenderPipeline '{}' is not registered in the RenderSystem", pipeline_id)).as_mut();

        // Get the next image index from the render target
        let frame_index: usize = match target.get_index(Some(&self.image_ready[self.current_frame])) {
            Ok(Some(index)) => index,
            Ok(None)        => {
                // Get the new size from the target
                let new_size = target.real_extent();
                // If it's zero, then skip and wait until the window has a valid size again
                if new_size.w == 0 && new_size.h == 0 { return Ok(()); }

                // Rebuild the target and then the window
                debug!("Resizing {} and {} to: {}", target_id, pipeline_id, new_size);
                if let Err(err) = target.rebuild(&new_size) { return Err(Error::TargetRebuildError{ id: target_id, err }); }
                if let Err(err) = pipeline.rebuild(target) { return Err(Error::PipelineRebuildError{ id: pipeline_id, err }); }

                // Simply go through it again to do the proper render call
                return self.render(pipeline_id, target_id);
            },
            Err(err) => { return Err(Error::TargetGetIndexError{ err }); },
        };

        // Tell the pipeline to render
        if let Err(err) = pipeline.render(frame_index, &[&self.image_ready[self.current_frame]], &[&self.render_ready[self.current_frame]], &self.in_flight[self.current_frame]) {
            return Err(Error::RenderError{ err });
        }

        // Even though the frame is not being rendered and such, schedule its presentation
        match target.present(frame_index, &[&self.render_ready[self.current_frame]]) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::PresentError{ err }),
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
    pub fn list(debug: bool) -> Result<(Vec<DeviceInfo>, Vec<DeviceInfo>), Error> {
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
