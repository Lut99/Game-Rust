/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   20 Apr 2022, 17:11:26
 * Last edited:
 *   30 Apr 2022, 17:45:16
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the module that contains the different pipelines that we
 *   define.
**/

/// Collects the errors for all pipelines
pub mod errors;
/// The simple triangle pipeline
pub mod triangle;


// Bring some stuff into the module scope
pub use triangle::Pipeline as TrianglePipeline;
