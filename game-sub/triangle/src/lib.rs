/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 13:15:24
 * Last edited:
 *   03 Apr 2022, 13:16:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Triangle RenderPipeline.
**/

/// Collects specifications for this crate (including the create info).
pub mod spec;
/// Collects errors for this crate.
pub mod errors;
/// Implements the actual pipeline for this crate.
pub mod pipeline;


// Bring some stuff into the crate namespace
pub use spec::CreateInfo;
pub use pipeline::TrianglePipeline as Pipeline;
