//  SPEC.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:05:56
//  Last edited:
//    31 Jul 2022, 12:06:28
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines super-crate interfaces and structs.
// 

use std::error::Error;


/***** LIBRARY *****/
/// Defines the possible events that might occur.
#[derive(Debug)]
pub enum Event {
    /// Defines an event that is called whenever a Target needs to be redrawn.
    /// 
    /// Contains the ID of the thing that we want to redraw.
    Draw,



    /// Defines an event that is called whenever a game tick has occurred.
    Tick,

    /// Signals that a single 'game loop' has been completed.
    GameLoopComplete,



    /// Defines an event that is called whenever the game is closing.
    /// 
    /// Contains whether or not this exit was caused by an error (the error) or not (None).
    Exit(Option<Box<dyn Error>>),
}
