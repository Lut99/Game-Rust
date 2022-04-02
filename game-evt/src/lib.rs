/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:33:55
 * Last edited:
 *   02 Apr 2022, 14:52:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the EventLoop library for the Game.
**/

/// Module that collects the interfaces etc for this crate.
pub mod spec;
/// Module that collects the errors for this crate.
pub mod errors;
/// Module that implements the actual event loop.
pub mod event_loop;


// Bring some stuff into the crate namespace
pub use spec::{EventHandler, EventType};
pub use event_loop::EventLoop;
