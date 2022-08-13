//  MOD.rs
//    by Lut99
// 
//  Created:
//    30 Apr 2022, 17:34:49
//  Last edited:
//    13 Aug 2022, 12:59:52
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the triangle module within the pipelines module.
// 

// Declare modules
pub mod vertex;
pub mod pipeline;


// Define constants
/// The name of this specific pipeline
pub const NAME: &'static str = "Triangle";


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/triangle/shaders/spir-v"]
struct Shaders;


// Bring some stuff into the module scope
pub use vertex::TriangleVertex as Vertex;
pub use pipeline::TrianglePipeline as Pipeline;
