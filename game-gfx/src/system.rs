/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 18:07:31
 * Last edited:
 *   26 Mar 2022, 18:17:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base RenderSystem.
**/

#[cfg(feature = "log")]
use log::debug;
use semver::Version;

use game_ecs::Ecs;
use game_vk::instance::Instance;

pub use crate::errors::RenderSystemError as Error;


/***** LIBRARY *****/
/// The RenderSystem, which handles the (rasterized) rendering & windowing part of the game.
pub struct RenderSystem {
    /// The Instance on which this RenderSystem is based.
    instance : Instance,
}

impl RenderSystem {
    /// Constructor for the RenderSystem.
    /// 
    /// **Generic types**
    ///  * `S1`: The &str-like type of the application name.
    ///  * `S2`: The &str-like type of the engine name.
    /// 
    /// **Arguments**
    ///  * `ecs`: The Entity Component System to register new components with.
    ///  * `name`: The name of application.
    ///  * `engine`: The name of application's engine.
    ///  * `version`: The version of application's engine.
    /// 
    /// **Returns**  
    /// The new RenderSystem on success, or else an Error.
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(ecs: &mut Ecs, name: S1, engine: S2, version: Version) -> Result<Self, Error> {
        // Register components
        /* TBD */

        // Create the instance
        let instance = match Instance::new(name, engine, version, vec![], vec![]) {
            Ok(instance) => instance,
            Err(err)     => { return Err(Error::InstanceCreateError{ err }); }  
        };
        
        // Use that to create the system
        #[cfg(feature = "log")]
        debug!("Initialized RenderSystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            instance,
        })
    }
}
