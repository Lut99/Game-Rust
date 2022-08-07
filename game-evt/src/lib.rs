//  LIB.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:29:26
//  Last edited:
//    07 Aug 2022, 18:17:12
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the EventSystem library, which manages the events
//!   within
// 

// Define the submodules of this crate
pub mod errors;
pub mod spec;
pub mod system;

// Pull some things into the crate namespace
pub use system::{Error, EventSystem};
