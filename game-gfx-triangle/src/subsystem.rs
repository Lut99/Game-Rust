/* SUBSYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:13:48
 * Last edited:
 *   02 Apr 2022, 14:45:02
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the actual subsystem itself.
**/

use log::debug;

use game_evt::EventLoop;
use game_gfx::{RenderSubsystem, RenderSubsystemBuilder};
use game_vk::instance::Instance;
use game_win::Window;

use crate::errors::CreateError;
use crate::spec::CreateInfo;


/***** TRIANGLE SUBSYSTEM *****/
/// The Triangle subsystem for the RenderSystem, which simply renders a triangle to a screen.
pub struct System {
    /// The Window this system renders to
    window : Window,
}

impl RenderSubsystem for System {
    
}

impl RenderSubsystemBuilder for System {
    type CreateInfo = CreateInfo;
    type CreateError = CreateError;


    /// Constructor for the RenderSubsystem.
    /// 
    /// This function initializes a new subsystem using the given CreateInfo to tune it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// When this function errors determines on the actual implementation of the constructor. However, it is defined that if it does, it returns the given CreateError.
    fn new(event_loop: &EventLoop, instance: &Instance, create_info: Self::CreateInfo) -> Result<Self, Self::CreateError> {
        // Initialize a Window
        let window = match Window::new(event_loop.event_loop(), instance.entry(), instance.instance(), "Rust-Game - Triangle") {
            Ok(window) => window,
            Err(err)   => { return Err(CreateError::WindowCreateError{ err }); }
        };



        // Done
        debug!("Initialized Triangle Subsystem v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            window,
        })
    }
}
