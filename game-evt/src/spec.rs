/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 12:03:22
 * Last edited:
 *   15 May 2022, 13:46:59
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines interfaces and such used by the EventSystem.
**/

use std::error::Error;

use game_utl::traits::AsAny;


/***** LIBRARY TRAITS *****/
/// Defines the possible Events that the EventSystem may fire on.
pub trait Event: AsAny {
    /// Returns whether this Event is asynchronous (i.e., event fire order does not matter) or not.
    fn is_async(&self) -> bool;
    
    /// Returns whether this Event is meta (i.e., wants to register/deregister callbacks or fire meta events) or not.
    fn is_meta(&self) -> bool;

    /// Returns the concrete EventKind of this Event.
    fn kind(&self) -> EventKind;
}





/***** LIBRARY EVENT ENUMS *****/
/// Defines synchronous, non-meta Events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SyncEvent {
    
}

impl Event for SyncEvent {
    /// Returns whether this Event is asynchronous (i.e., event fire order does not matter) or not.
    #[inline]
    fn is_async(&self) -> bool { false }
    
    /// Returns whether this Event is meta (i.e., wants to register/deregister callbacks or fire meta events) or not.
    #[inline]
    fn is_meta(&self) -> bool { false }

    /// Returns the concrete EventKind of this Event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::Sync }
}



/// Defines synchronous meta Events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MetaSyncEvent {
    
}

impl Event for MetaSyncEvent {
    /// Returns whether this Event is asynchronous (i.e., event fire order does not matter) or not.
    #[inline]
    fn is_async(&self) -> bool { false }
    
    /// Returns whether this Event is meta (i.e., wants to register/deregister callbacks or fire meta events) or not.
    #[inline]
    fn is_meta(&self) -> bool { true }

    /// Returns the concrete EventKind of this Event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::MetaSync }
}



/// Defines asynchronous, non-meta Events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AsyncEvent {
    
}

impl Event for AsyncEvent {
    /// Returns whether this Event is asynchronous (i.e., event fire order does not matter) or not.
    #[inline]
    fn is_async(&self) -> bool { true }
    
    /// Returns whether this Event is meta (i.e., wants to register/deregister callbacks or fire meta events) or not.
    #[inline]
    fn is_meta(&self) -> bool { false }

    /// Returns the concrete EventKind of this Event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::Async }
}



/// Defines asynchronous meta Events.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MetaAsyncEvent {
    
}

impl Event for MetaAsyncEvent {
    /// Returns whether this Event is asynchronous (i.e., event fire order does not matter) or not.
    #[inline]
    fn is_async(&self) -> bool { true }
    
    /// Returns whether this Event is meta (i.e., wants to register/deregister callbacks or fire meta events) or not.
    #[inline]
    fn is_meta(&self) -> bool { true }

    /// Returns the concrete EventKind of this Event.
    #[inline]
    fn kind(&self) -> EventKind { EventKind::MetaAsync }
}





/***** LIBRARY AUXILLARY ENUMS *****/
/// Defines the possible EventTypes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventKind {
    /// These events do not manage other events/callbacks or fire events that do. They are supposed to be fired synchronously though, and can thus prevent events from bubbling up an event chain.
    Sync,
    /// These events manage other events/callbacks or fire events that do. They are also supposed to be fired synchronously though, and can thus prevent events from bubbling up an event chain.
    MetaSync,

    /// These events do not manage other events/callbacks or fire events that do, neither do they require an ordering when fired.
    Async,
    /// These events manage other events/callbacks or fire events that do. However, they don't require an ordering when fired.
    MetaAsync,
}



/// Defines the action that a callbacks wants to take after an Event has been fired.
/// 
/// This specific enum is for synchronous events, who may also block the Event from propagating to other callbacks.
#[derive(Debug)]
pub enum SyncResult {
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

impl Default for SyncResult {
    #[inline]
    fn default() -> Self { SyncResult::Continue }
}



/// Marries the SyncResult and AsyncResult together in one enum.
#[derive(Debug)]
pub enum EventResult {
    /// Nothing special needs to happen.
    Continue,

    /// The program should stop.
    Exit,

    /// The program ran into an error.
    Error(Box<dyn Error>),
    /// The program ran into an error so severe the Game should quit.
    Fatal(Box<dyn Error>),
}

impl EventResult {
    /// Constructor for the EventResult that maps from a SyncResult.
    /// 
    /// This may fail if the SyncResult is `SyncResult::Block`.
    /// 
    /// # Returns
    /// The new EventResult or else None if we could not map the value.
    #[inline]
    pub fn from(value: SyncResult) -> Option<Self> {
        match value {
            SyncResult::Continue   => Some(EventResult::Continue),
            SyncResult::Exit       => Some(EventResult::Exit),
            SyncResult::Error(err) => Some(EventResult::Error(err)),
            SyncResult::Fatal(err) => Some(EventResult::Fatal(err)),

            // Fail in other cases
            _ => None,
        }
    }
}

impl Default for EventResult {
    #[inline]
    fn default() -> Self { EventResult::Continue }
}

impl From<EventResult> for SyncResult {
    #[inline]
    fn from(value: EventResult) -> Self {
        match value {
            EventResult::Continue   => SyncResult::Continue,
            EventResult::Exit       => SyncResult::Exit,
            EventResult::Error(err) => SyncResult::Error(err),
            EventResult::Fatal(err) => SyncResult::Fatal(err),
        }
    }
}
