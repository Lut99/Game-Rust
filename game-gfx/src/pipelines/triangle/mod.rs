/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:34:49
 * Last edited:
 *   30 Apr 2022, 17:35:39
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the triangle module within the pipelines module.
**/


/// Implements the pipeline
pub mod pipeline;


// Bring some stuff into the module scope
pub use pipeline::Pipeline;
