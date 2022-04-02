/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:00:33
 * Last edited:
 *   02 Apr 2022, 14:17:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the GFX library, which implements the render system and
 *   window management.
**/

// Get some external crate macros
#[macro_use] extern crate lazy_static;

/// The module that contains common specifications.
pub mod spec;
/// The module for the the component lists.
pub mod errors;
/// The module that implements the main RenderSystem.
pub mod system;

// Bring some components into the general package namespace
pub use spec::RenderSubsystem;
pub use spec::RenderSubsystemBuilder;
pub use system::Error;
pub use system::RenderSystem;
