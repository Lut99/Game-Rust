//  MOD.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2022, 15:55:22
//  Last edited:
//    13 Aug 2022, 12:59:58
//  Auto updated?
//    Yes
// 
//  Description:
//!   This module implements the SquarePipeline, which is much the same as
//!   the TrianglePipeline except that it uses an index buffer.
// 

// Declare submodules
pub mod vertex;
pub mod pipeline;


// Define constants
/// The name of this specific pipeline
pub const NAME: &'static str = "Square";


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/square/shaders/spir-v"]
struct Shaders;


// Bring some stuff into the module scope
pub use vertex::SquareVertex as Vertex;
pub use pipeline::SquarePipeline as Pipeline;
