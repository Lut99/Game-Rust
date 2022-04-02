/* EVENT LOOP.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:35:29
 * Last edited:
 *   02 Apr 2022, 14:56:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the actual event loop.
**/

use winit::event_loop::EventLoop as WEventLoop;

use crate::spec::EventHandler;


/**** EVENT LOOP *****/
/// Wraps around winit's EventLoop to provide more complex functions.
pub struct EventLoop {
    /// The winit event loop around which we wrap
    event_loop : WEventLoop<()>,

    /// The list of handlers we call when necessary
    event_handlers : Vec<String>,
}

impl EventLoop {
    /// Constructor for the EventLoop.
    pub fn new() -> Self {
        // Create the winit event loop
        let event_loop = WEventLoop::new();

        // Create a new EventLoop around it
        Self {
            event_loop,
        }
    }



    /// Registers a new event handler.
    pub fn register(&mut self, handler: &dyn EventHandler) {
        
    }



    /// Returns the internal event loop.
    #[inline]
    pub fn event_loop(&self) -> &WEventLoop<()> { &self.event_loop }
}
