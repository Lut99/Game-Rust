/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:53:31
 * Last edited:
 *   21 May 2022, 11:31:55
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the EventSystem crate in the Game.
**/

/// Collects the errors for this crate
pub mod errors;
/// Defines the (public) interfaces used by the EventSystem.
pub mod spec;
/// Defines the EventHandler class.
pub mod handler;
/// Defines the EventSystem itself
pub mod system;


// Bring some stuff into the main namespace
pub use spec::Event;
pub use system::{Error, EventSystem as System};
