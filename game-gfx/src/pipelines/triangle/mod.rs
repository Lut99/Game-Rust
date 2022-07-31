//  MOD.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:15:56
//  Last edited:
//    31 Jul 2022, 12:37:08
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the simplest pipeline of the bunch: one that simply renders
//!   a triangle to the screen. It does use Vertex buffers, but not yet
//!   complicated stuff like materials.
// 

pub mod spec;
pub mod components;
pub mod pipeline;


// Load the shader files
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v"]
struct Shaders;


// Bring stuff into this namespace
pub use pipeline::*;



/***** CONSTANTS *****/
/// The name of this pipeline.
pub const NAME: &str = "triangle";
