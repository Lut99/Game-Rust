/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 12:03:22
 * Last edited:
 *   21 May 2022, 11:02:46
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines interfaces and such used by the EventSystem.
**/

use std::error::Error;


/***** LIBRARY TRAITS *****/
/// The global trait stitching the Events together.
pub trait Event {
    
}





/***** LIBRARY EVENT ENUMS *****/
/// Defines Events eminating from Window stuff.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WindowEvent {
    
}

impl Event for WindowEvent {}

/// Defines Events eminating from mouse/keyboard/controller stuff.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputEvent {
    
}

impl Event for InputEvent {}

/// Defines time-based events
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TickEvent {
    /// A game tick has passed
    Tick,
}

impl Event for TickEvent {}





/***** LIBRARY AUXILLARY ENUMS *****/
/// Marries the SyncResult and AsyncResult together in one enum.
#[derive(Debug)]
pub enum EventResult {
    /// Nothing special needs to happen.
    Continue,

    /// The event should not propagate further down to other callbacks.
    Block,
    /// The program should stop.
    Exit,

    /// The program ran into an error.
    Error(Box<dyn Error>),
    /// The program ran into an error so severe the Game should quit.
    Fatal(Box<dyn Error>),
}

impl Default for EventResult {
    #[inline]
    fn default() -> Self { EventResult::Continue }
}
