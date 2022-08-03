/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   20 Apr 2022, 17:05:45
 * Last edited:
 *   05 May 2022, 20:37:34
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the module that defines the different RenderTargets.
**/

// Defines the Window render target
pub mod window;

// Bring some stuff into scope
pub use window::Window;
