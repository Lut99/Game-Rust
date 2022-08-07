//  LIB.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 13:00:33
//  Last edited:
//    07 Aug 2022, 13:48:09
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the GFX library, which implements the render system
//!   and
// 

// Get some external crate macros
#[macro_use] extern crate lazy_static;

// Declare modules
pub mod errors;
pub mod spec;
pub mod components;
pub mod system;
pub mod pipelines;

// Bring some components into the general package namespace
pub use system::{Error, RenderSystem};
