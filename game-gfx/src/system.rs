/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   30 Jul 2022, 18:10:00
 * Last edited:
 *   30 Jul 2022, 18:24:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the main code of the RenderSystem.
**/

use std::cell::RefCell;
use std::rc::Rc;

use game_ecs::Ecs;
use game_vk::instance::Instance;

pub use crate::errors::RenderError as Error;
use crate::spec::AppInfo;
use crate::components::{RenderTarget, Window};


/***** LIBRARY *****/
/// The RenderSystem is in charge of rendering objects to targets (windows or other images / buffers).
pub struct RenderSystem {
    /// The Entity Component System around which the RenderSystem is build.
    ecs : Rc<RefCell<Ecs>>,

    /// The Instance which we'll use throughout the Game.
    instance : Rc<Instance>,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The Entity Component System where all of the RenderSystem's entities will be stored.
    /// - `app_info`: The AppInfo struct that has some metadata about the application.
    /// 
    /// # Returns
    /// A new RenderSystem.
    /// 
    /// # Errors
    /// The constructor may fail if we failed to initialize the Vulkan backend.
    pub fn new(ecs: Rc<RefCell<Ecs>>, app_info: AppInfo) -> Result<Self, Error> {
        // Register our components
        Ecs::register::<RenderTarget>(&ecs);
        Ecs::register::<Window>(&ecs);



        // Instantiate the Vulkan objects we'll need.
        let instance: Rc<Instance> = match Instance::new(app_info.name, app_info.version, app_info.engine_name, app_info.engine_version, &[], &[]) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }
        };



        // Finally, return ourselves with the necessary objects
        Ok(Self {
            ecs,

            instance,
        })
    }



    /// Returns the internal Entity Component System.
    #[inline]
    pub fn ecs(&self) -> &Rc<RefCell<Ecs>> { &self.ecs }
}
