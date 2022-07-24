/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   18 Jul 2022, 18:29:26
 * Last edited:
 *   18 Jul 2022, 18:42:57
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the EventSystem library, which manages the events within
 *   the Game.
**/

/// Contains the errors of this crate.
pub mod errors;
/// Contains (public) interfaces and junk for this crate.
pub mod spec;
/// Contains the ECS definitions for this crate.
pub mod components;
/// Contains the system itself.
pub mod system;
