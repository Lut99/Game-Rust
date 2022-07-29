/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   29 Jul 2022, 12:40:10
 * Last edited:
 *   29 Jul 2022, 12:51:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the library that contains the Window system, which
 *   manages Windows.
**/


/// Contains errors that relate to the WindowSystem.
pub mod errors;
/// Contains ECS component definitions for this crate.
pub mod components;
/// Contains the system implementation itself.
pub mod system;


// Bring some stuff into the crate namespace
pub use components::Window;
pub use system::{WindowSystem, Error};
