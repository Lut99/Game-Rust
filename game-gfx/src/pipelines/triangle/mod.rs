/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:34:49
 * Last edited:
 *   03 Jul 2022, 14:41:19
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the triangle module within the pipelines module.
**/

/// Specifies the vertex definition for this pipeline
pub mod vertex;
/// Implements the pipeline
pub mod pipeline;


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v"]
struct Shaders;


// Bring some stuff into the module scope
pub use vertex::Vertex;
pub use pipeline::Pipeline;
