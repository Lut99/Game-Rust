/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   05 Apr 2022, 17:50:51
 * Last edited:
 *   18 Apr 2022, 14:38:14
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint for the image submodule.
**/

/// Submodule that defines our own image wrapper.
pub mod image;
/// Submodule that defines our own image view wrapper.
pub mod view;

// Bring some stuff into the submodule namespace
pub use image::{Error as ImageError, Image};
pub use view::{CreateInfo as ViewInfo, Error as ViewError, View};
