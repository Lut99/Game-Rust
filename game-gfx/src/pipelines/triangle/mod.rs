/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:34:49
 * Last edited:
 *   01 May 2022, 11:58:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the triangle module within the pipelines module.
**/

/// Implements the pipeline
pub mod pipeline;


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v"]
struct Shaders;


// Bring some stuff into the module scope
pub use pipeline::Pipeline;
