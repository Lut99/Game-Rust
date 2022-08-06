//  LIB.rs
//    by Lut99
// 
//  Created:
//    06 Aug 2022, 18:02:50
//  Last edited:
//    06 Aug 2022, 18:20:04
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `game-tgt` crate manages RenderTargets for the RenderSystem.
// 

// Declare modules
pub mod errors;
pub mod spec;
pub mod window;


// Export some useful stuff
pub use spec::{Error, RenderTarget};
