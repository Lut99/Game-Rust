/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   27 Mar 2022, 16:36:15
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base RenderSystem.
**/

use ash::vk;
use log::debug;
use semver::Version;

use game_ecs::Ecs;
use game_vk::gpu::Gpu;
use game_vk::instance::Instance;

pub use crate::errors::RenderSystemError as Error;


/***** CONSTANTS *****/
/// The list of instance extensions we want to enable (besides the required surface ones).
const INSTANCE_EXTENSIONS: Vec<&str> = vec![];

/// The list of instance layers we want to enable (besides the debug one).
const INSTANCE_LAYERS: Vec<&str> = vec![];

/// The list of device extensions we want to enable.
const DEVICE_EXTENSIONS: Vec<&str> = vec![];

/// The list of device layers we want to enable.
const DEVICE_LAYERS: Vec<&str> = vec![];

// Constants that are lazily loaded
lazy_static!{
    /// The list of device features we want to enable.
    static ref DEVICE_FEATURES: vk::PhysicalDeviceFeatures = Default::default();
}





/***** LIBRARY *****/
/// The RenderSystem, which handles the (rasterized) rendering & windowing part of the game.
pub struct RenderSystem {
    /// The Instance on which this RenderSystem is based.
    instance : Instance,
    /// The Gpu we'll use for rendering.
    gpu      : Gpu,
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
            let mut layers = INSTANCE_LAYERS.clone();
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            INSTANCE_LAYERS
        };
        let instance = match Instance::new(name, version, engine, engine_version, &INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Get the GPU
        let gpu = match Gpu::new(&instance, gpu, &DEVICE_EXTENSIONS, &DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(gpu)  => gpu,
            Err(err) => { return Err(Error::GpuCreateError{ err }); }  
        };

        // Use that to create the system
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            instance,
            gpu,
        })
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
            let mut layers = INSTANCE_LAYERS.clone();
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            INSTANCE_LAYERS
        };
        let instance = match Instance::new("Dummy Application", Version::new(0, 1, 0), "Dummy Engine", Version::new(0, 1, 0), &INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Call the list on the GPU class
        match Gpu::auto_select(&instance, &DEVICE_EXTENSIONS, &DEVICE_LAYERS, &*DEVICE_FEATURES) {
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
            let mut layers = INSTANCE_LAYERS.clone();
            layers.append(&mut vec!["VK_LAYER_KHRONOS_validation"]);
            layers
        } else {
            INSTANCE_LAYERS
        };
        let instance = match Instance::new("Dummy Application", Version::new(0, 1, 0), "Dummy Engine", Version::new(0, 1, 0), &INSTANCE_EXTENSIONS, &layers) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };

        // Call the list on the GPU class
        match Gpu::list(&instance, &DEVICE_EXTENSIONS, &DEVICE_LAYERS, &*DEVICE_FEATURES) {
            Ok(())   => Ok(()),
            Err(err) => Err(Error::GpuListError{ err }),
        }
    }
}
