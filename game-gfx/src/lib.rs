//  LIB.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 13:00:33
//  Last edited:
//    06 Aug 2022, 17:53:50
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the GFX library, which implements the render system
//!   and
// 

// Get some external crate macros
#[macro_use] extern crate lazy_static;

/// The module for the the component lists.
pub mod errors;
/// The module that contains common specifications.
pub mod spec;
/// The module that contains ECS definitions.
pub mod components;
/// The module that implements the main RenderSystem.
pub mod system;
/// The module that implements the different render targets.
pub mod targets;
/// The module that implements the different pipelines.
pub mod pipelines;

// Bring some components into the general package namespace
pub use system::{Error, RenderSystem};
