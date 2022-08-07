//  MOD.rs
//    by Lut99
// 
//  Created:
//    30 Apr 2022, 17:34:49
//  Last edited:
//    07 Aug 2022, 12:54:28
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the triangle module within the pipelines module.
// 

/// Specifies the vertex definition for this pipeline
pub mod vertex;
/// Implements the pipeline
pub mod pipeline;


// Define constants
/// The name of this specific pipeline
pub const NAME: &'static str = "Triangle";


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v"]
struct Shaders;


// Bring some stuff into the module scope
pub use vertex::Vertex;
pub use pipeline::Pipeline;
