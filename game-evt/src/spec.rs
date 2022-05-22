/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 12:03:22
 * Last edited:
 *   22 May 2022, 14:25:21
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
/// The EventHandler trait, which defines a generalised interface to both the LocalEventHandler and the ThreadedEventHandler.
pub trait EventHandler {
    /// The Event type to handle.
    type Event: Eq + Hash;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Callback<Self, Self::Event>) -> Result<(), Box<dyn Error>>
    where
        Self: Sized;

    /// Fires the given Event.
    /// 
    /// Firing an event may trigger other Events. Thus, it is good practise not to have a cyclic dependency.
    /// 
    /// # Arguments
    /// - `event`: The Event to fire.
    /// 
    /// # Returns
    /// Returns the value of the last EventResult callback in the chain, or (if there are no callbacks for this Event) `EventResult::Continue`.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn fire(&self, event: Self::Event) -> Result<EventResult, Box<dyn Error>>;



    /// Stops the EventHandler (telling threads to stop and junk)
    fn stop(&mut self);
}



/// The traits implementing the closure
pub trait Callback<H: EventHandler<Event=E>, E>: Send + Sync + FnMut(&H, E) -> EventResult {}

impl<H: EventHandler<Event=E>, E, T> Callback<H, E> for T where T: Send + Sync + FnMut(&H, E) -> EventResult {}



/// The global trait stitching the Events together.
pub trait Event: AsAny + Debug + Eq + Hash {
    /// Returns the kind of this event.
    fn kind(&self) -> EventKind;
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

impl EventResult {
    /// Returns some kind of 'importance' score; results with a higher score will not be kicked out in multi-threaded scenario's.
    #[inline]
    pub fn importance(&self) -> u32 {
        use EventResult::*;
        match self {
            Continue => 0,

            Block => 1,
            Exit  => 2,

            Error(_) => 1,
            Fatal(_) => 2,
        }
    }
}

impl Default for EventResult {
    #[inline]
    fn default() -> Self { EventResult::Continue }
}
