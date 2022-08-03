/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   18 Jul 2022, 18:25:39
 * Last edited:
 *   18 Jul 2022, 19:08:24
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the components in the ECS for the event system. This thus
 *   mostly encompasses (general) events.
**/

use std::error::Error;

use winit::window::WindowId;

use game_ecs::spec::Component;


/***** LIBRARY *****/
/// Defines a Draw callback, which is called whenever the window needs redrawing.
pub struct DrawCallback {
    /// The callback to call.
    /// 
    /// # Arguments
    /// - ``: 
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub draw_callback: Box<dyn FnMut(WindowId) -> Result<(), Box<dyn Error>>>,
}

impl Component for DrawCallback {}



/// Defines a Tick callback, which means that the given closure will be fired when a game tick happens.
pub struct TickCallback {
    /// The callback to call.
    /// 
    /// # Arguments
    /// - ``: 
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub tick_callback : Box<dyn FnMut() -> Result<(), Box<dyn Error>>>,
}

impl Component for TickCallback {}



/// The ExitCallback component is used to mark entities that need to handle stuff on program exit.
pub struct ExitCallback {
    /// The callback to call.
    /// 
    /// # Arguments
    /// - ``: 
    /// 
    /// # Returns
    /// Whether or not the exiting should continue (true) or not (false).
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub exit_callback : Box<dyn FnMut() -> Result<bool, Box<dyn Error>>>,
}

impl Component for ExitCallback {}
