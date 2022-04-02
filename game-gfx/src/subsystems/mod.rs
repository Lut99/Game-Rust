/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 12:48:24
 * Last edited:
 *   02 Apr 2022, 13:23:37
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the subsystems module.
**/

/// Contains the module traits and interfaces
pub mod spec;
/// Contains the simplest subsystem we have
pub mod triangle;

// Bring some nested things into this namespace
pub use spec::{RenderSubsystem, RenderSubsystemBuilder};
