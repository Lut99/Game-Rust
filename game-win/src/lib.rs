/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   28 Mar 2022, 21:06:39
 * Last edited:
 *   03 Apr 2022, 12:01:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the crate that handles the windows for the game.
**/

/// Collects the errors of this crate.
pub mod errors;
/// Implements the Window class used.
pub mod window;


// Bring some stuff in the lib namespace
pub use window::{Error, Window};
