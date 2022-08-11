//  LIB.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2022, 15:35:15
//  Last edited:
//    11 Aug 2022, 15:40:52
//  Auto updated?
//    Yes
// 
//  Description:
//!   This crate implements various pipelines used while rendering the
//!   Game.
// 

// Declare submodules
pub mod errors;
pub mod spec;
pub mod triangle;

// Pull some stuff into the general namespace
pub use errors::RenderPipelineError as Error;
pub use spec::RenderPipeline;
pub use triangle::{Pipeline as TrianglePipeline};
