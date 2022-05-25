/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 12:03:22
 * Last edited:
 *   25 May 2022, 21:26:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines interfaces and such used by the EventSystem.
**/

use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;

use winit::window::WindowId;

use game_utl::traits::AsAny;


/***** LIBRARY TRAITS *****/
/// The global trait stitching the Events together.
pub trait Event: AsAny + Debug + Eq + Hash + Send + Sync {
    /// Returns the kind of this event.
    fn kind(&self) -> EventKind;
}



/// The global EventResult trait stitching EventResults together.
pub trait EventResult: Debug + PartialEq {
    /// Returns the ::Continue variant.
    fn cont() -> Self;
}





/***** LIBRARY EVENT ENUMS *****/
/// Defines Events relating to the general control flow.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ControlEvent {
    /// The game is closing
    Closing,
}

impl Event for ControlEvent {
    /// Returns the kind of this event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::ControlEvent }
}



/// Defines Events eminating from Window stuff.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WindowEvent {
    /// Requests all Windows to redraw
    RequestRedraw,
    /// Redraws a specific Window
    Redraw(WindowId),
}

impl Event for WindowEvent {
    /// Returns the kind of this event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::WindowEvent }
}



/// Defines Events eminating from mouse/keyboard/controller stuff.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InputEvent {
    
}

impl Event for InputEvent {
    /// Returns the kind of this event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::InputEvent }
}



/// Defines time-based events
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TickEvent {
    /// A game tick has passed
    Tick,
}

impl Event for TickEvent {
    /// Returns the kind of this event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::TickEvent }
}





/***** LIBRARY AUXILLARY ENUMS *****/
/// Enumerates the type of Events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventKind {
    ControlEvent,
    WindowEvent,
    InputEvent,
    TickEvent,
}



/// Defines the results allowed for a LocalEvent callback.
#[derive(Debug)]
pub enum LocalEventResult {
    /// Nothing special needs to happen.
    Continue,

    /// Stop propagating the event.
    Block,

    /// The callback ran into an error.
    Error(Box<dyn Error>),
    /// The callback ran into an error so severe the Game should quit.
    Fatal(Box<dyn Error>),
}

impl Default for LocalEventResult {
    #[inline]
    fn default() -> Self { LocalEventResult::Continue }
}

impl EventResult for LocalEventResult {
    /// Returns the ::Continue variant.
    #[inline]
    fn cont() -> Self { Self::Continue }
}

impl PartialEq for LocalEventResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Continue, Self::Continue) => true,
            (Self::Block,    Self::Block)    => true,
            (Self::Error(_), Self::Error(_)) => true,
            (Self::Fatal(_), Self::Fatal(_)) => true,
            _                                => false,
        }
    }
}



/// Defines the results allowed for a ControlEvent callback.
#[derive(Debug)]
pub enum ControlEventResult {
    /// Nothing special needs to happen.
    Continue,

    /// Stop propagating the event.
    Block,
    /// Stop the application altogether.
    Exit,

    /// The callback ran into an error.
    Error(Box<dyn Error>),
    /// The callback ran into an error so severe the Game should quit.
    Fatal(Box<dyn Error>),
}

impl Default for ControlEventResult {
    #[inline]
    fn default() -> Self { ControlEventResult::Continue }
}

impl EventResult for ControlEventResult {
    /// Returns the ::Continue variant.
    #[inline]
    fn cont() -> Self { Self::Continue }
}

impl PartialEq for ControlEventResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Continue, Self::Continue) => true,
            (Self::Block,    Self::Block)    => true,
            (Self::Exit,     Self::Exit)     => true,
            (Self::Error(_), Self::Error(_)) => true,
            (Self::Fatal(_), Self::Fatal(_)) => true,
            _                                => false,
        }
    }
}



/// Defines the results allowed for a ThreadedEvent callback.
#[derive(Debug)]
pub enum ThreadedEventResult {
    /// Nothing special needs to happen.
    Continue,

    /// The callback ran into an error.
    Error(Box<dyn Error>),
    /// The callback ran into an error so severe the Game should quit.
    Fatal(Box<dyn Error>),
}

impl Default for ThreadedEventResult {
    #[inline]
    fn default() -> Self { ThreadedEventResult::Continue }
}

impl EventResult for ThreadedEventResult {
    /// Returns the ::Continue variant.
    #[inline]
    fn cont() -> Self { Self::Continue }
}

impl PartialEq for ThreadedEventResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Continue, Self::Continue) => true,
            (Self::Error(_), Self::Error(_)) => true,
            (Self::Fatal(_), Self::Fatal(_)) => true,
            _                                => false,
        }
    }
}
