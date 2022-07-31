//  LIB.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 11:56:00
//  Last edited:
//    31 Jul 2022, 12:14:51
//  Auto updated?
//    Yes
//
//  Description:
//!   The `game-gfx` crate manages rendering within the Game. Specifically, it
//!   interfaces with Vulkan and winit and specific shaders.
//

#[macro_use]
extern crate lazy_static;

pub mod errors;
pub mod spec;
pub mod components;
pub mod windows;
pub mod pipelines;
pub mod system;


// Bring some stuff into the global namespace
pub use system::{Error, RenderSystem};
