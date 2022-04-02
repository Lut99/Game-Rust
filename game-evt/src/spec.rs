/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:35:08
 * Last edited:
 *   02 Apr 2022, 14:54:38
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Module that collects all interfaces etc for this crate.
**/

use std::error::Error;


/***** EVENT TYPES *****/
/// The types of events we're interested in
pub enum EventType {
    /// The user closed the window
    WindowCloseRequested,
    /// The window has cleared its events, so it should redraw
    WindowMainEventsCleared,
    /// A redraw has been requested, so we know we can draw the frame
    WindowRedrawRequested,
}





/***** HANDLER INTERFACE *****/
/// Defines that a certain class is an EventHandler.
pub trait EventHandler {
    /// Handles events that occur in the EventLoop.
    /// 
    /// This is a callback for the EventLoop to call when and event is ready to be processed.
    /// 
    /// All types of events are passed. Any unneeded events can safely be ignored.
    /// 
    /// In case the program should stop gracefully, return 'true'. Return 'false' if the program should continue, or any error if it should stop ungracefully.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TODO
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function can error anytime it likes, as long as it returns a valid error type.
    fn handle(&mut self, event: &EventType) -> Result<bool, Box<dyn Error>>;
}
