//  SPEC.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:42:16
//  Last edited:
//    07 Aug 2022, 18:06:05
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs for the EventSystem.
// 

use winit::window::WindowId;

pub use crate::errors::EventError as Error;


/***** LIBRARY *****/
/// Defines the possible events that might occur.
pub enum Event {
    /// A Window needs to be redrawn.
    /// 
    /// Contains the ID of the to-be-redrawn Window.
    WindowDraw(WindowId),

    /// A single iteration of the game loop has been completed.
    GameLoopComplete,
    /// The game is quitting.
    /// 
    /// Contains whether the game quits naturally (None) or due to an Error (in which case it describes it).
    Exit(Option<Error>),
}
