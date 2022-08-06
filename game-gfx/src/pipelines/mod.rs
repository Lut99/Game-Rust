//  MOD.rs
//    by Lut99
// 
//  Created:
//    20 Apr 2022, 17:11:26
//  Last edited:
//    06 Aug 2022, 17:55:12
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the module that contains the different pipelines that
//!   we
// 

/// The simple triangle pipeline
pub mod triangle;


// Bring some stuff into the module scope
pub use triangle::Pipeline as TrianglePipeline;
