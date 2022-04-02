/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 14:08:00
 * Last edited:
 *   02 Apr 2022, 14:13:24
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Triangle subsystem for the RenderSystem.
**/

/// The module that contains the errors for this subsystem
pub mod errors;
/// The module that contains the specifications (like the CreateInfo) for this subsystem
pub mod spec;
/// The module that contains the subsystem itself
pub mod subsystem;


// Bring some elements in to this namespace
pub use errors::CreateError;
pub use spec::CreateInfo;
pub use subsystem::System;
