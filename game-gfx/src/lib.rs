//  LIB.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 11:56:00
//  Last edited:
//    30 Jul 2022, 11:56:08
//  Auto updated?
//    Yes
//
//  Description:
//!   The `game-gfx` crate manages rendering within the Game. Specifically, it
//!   interfaces with Vulkan and winit and specific shaders.
//

pub mod errors;
pub mod spec;
pub mod components;
pub mod system;


// Bring some stuff into the global namespace
pub use system::{Error, RenderSystem};
