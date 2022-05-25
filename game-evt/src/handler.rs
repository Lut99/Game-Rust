/* HANDLER.rs
 *   by Lut99
 *
 * Created:
 *   21 May 2022, 11:31:00
 * Last edited:
 *   25 May 2022, 21:32:39
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements code to handle a single queue of events within the
 *   EventSystem. This means it's a queue (:0) together with some handler
 *   threads (:00) that handle the events fired on the queue.
**/

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use async_trait::async_trait;
use futures::future::Future;
use futures::stream::FuturesOrdered;

use crate::spec::{Event, EventResult, ThreadedEventResult};


/***** CUSTOM TYPES *****/
// /// The type of the EventQueues we use.
// type EventQueue<H, E> = VecDeque<(Arc<dyn Callback<H, E>>, E)>;





/***** EVENTHANDLER TRAIT *****/
/// The EventHandler trait, which defines a generalised interface to both the LocalEventHandler and the ThreadedEventHandler.
#[async_trait]
pub trait EventHandler {
    /// The Event type to handle.
    type Event: Event;
    /// The Delegate to send to callbacks.
    type Delegate;
    /// The return type of the Callbacks.
    type EventResult: EventResult;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Send + Sync + FnMut(&Self::Delegate, Self::Event) -> Self::EventResult) -> Result<(), Box<dyn Error>>
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
    async fn fire(&self, event: Self::Event) -> Result<Self::EventResult, Box<dyn Error>>;
}





/***** EVENT HANDLERS *****/
/// The LocalEventHandler handles fired events on the local thread.
pub struct LocalEventHandler<E, R>{
    /// A list of callbacks to call for each possible event type.
    callbacks : Arc<RwLock<HashMap<E, Vec<Box<dyn Send + Sync + FnMut(&Self, E) -> R>>>>>,
}

impl<E, R> LocalEventHandler<E, R> {
    /// Constructor for the LocalEventHandler.
    pub fn new() -> Self {
        Self {
            callbacks : Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

#[async_trait]
impl<E, R> EventHandler for LocalEventHandler<E, R>
where
    E: Event,
    R: EventResult,
{
    type Event = E;
    type Delegate = Self;
    type EventResult = R;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Send + Sync + FnMut(&Self::Delegate, Self::Event) -> Self::EventResult) -> Result<(), Box<dyn Error>> {
        // Get a write lock
        let mut map: RwLockWriteGuard<HashMap<_, _>> = self.callbacks.write().expect("Could not get write lock on callbacks");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match map.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                map.insert(event, Vec::with_capacity(4));

                // Return the new queue
                map.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));

        // Done
        Ok(())
    }



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
    async fn fire(&self, event: Self::Event) -> Result<Self::EventResult, Box<dyn Error>>
    where
        E: Event,
        R: EventResult,
    {
        // Get a read lock
        let map: RwLockReadGuard<HashMap<_, _>> = self.callbacks.read().expect("Could not get read lock on callbacks");

        // If there is an Event to fire, then fire it
        if let Some(callbacks) = map.get(&event) {
            for callback in callbacks {
                let res = callback(self, event);
                // Continue if it's a Continue value; return otherwise
                if R::cont() == res { continue; }
                return Ok(res);
            }
        }

        // Done
        Ok(R::cont())
    }
}



/// The ThreadedEventHandler spawns multiple threads and uses those to process the callbacks for fired events.
pub struct ThreadedEventHandler<E>{
    /// A list of callbacks to call for each possible event type.
    callbacks : RefCell<HashMap<E, Vec<Box<dyn Send + Sync + FnMut(&ThreadedEventDelegate, E) -> ThreadedEventResult>>>>,
}

impl<E> ThreadedEventHandler<E> {
    /// Constructor for the ThreadedEventHandler.
    pub fn new() -> Self {
        Self {
            callbacks : RefCell::new(HashMap::with_capacity(16)),
        }
    }
}

#[async_trait]
impl<E> EventHandler for ThreadedEventHandler<E>
where
    E: Event,
{
    type Event = E;
    type Delegate = &ThreadedEventDelegate;
    type EventResult = ThreadedEventResult;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Send + Sync + FnMut(&Self::Delegate, Self::Event) -> Self::EventResult) -> Result<(), Box<dyn Error>> {
        // Borrow the hashmap muteably
        let mut map: RefMut<HashMap<_, _>> = self.callbacks.borrow_mut();

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match map.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                map.insert(event, Vec::with_capacity(4));

                // Return the new queue
                map.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));

        // Done
        Ok(())
    }



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
    async fn fire(&self, event: Self::Event) -> Result<Self::EventResult, Box<dyn Error>>
    where
        E: Event,
    {
        // Borrow the list of callbacks
        let callbacks: Ref<HashMap<_, _>> = self.callbacks.borrow();

        // Get the relevant queue of callbacks
        let callbacks: Vec<_> = match callbacks.get(&event) {
            Some(callbacks) => callbacks.iter().map(|callback| callback.as_mut()).collect(),
            None            => { return Ok(ThreadedEventResult::Continue); }
        };

        // Create the empty futuresordered
        let execute_queue: ThreadedEventDelegate = ThreadedEventDelegate::new(callbacks, event);
    }
}



/// Implements the delegate for the ThreadedEventHandler, which is the actual handler sent to the callbacks.
pub struct ThreadedEventDelegate<'a> {
    /// The FuturesOrdered struct that we use to run everything
    execute_queue : FuturesOrdered<Pin<Box<dyn 'a + Future<Output = ThreadedEventResult>>>>,
}

impl<'a> ThreadedEventDelegate<'a> {
    /// Constructor for the ThreadedEventDelegate.
    /// 
    /// # Arguments
    /// - `callbacks`: The initial list of callbacks to pass to the delegate.
    fn new<E>(callbacks: Vec<&'a mut dyn FnMut(&Self, E) -> ThreadedEventResult>, event: E) -> Self {
        // Create the empty FuturesOrdered queue
        let execute_queue: FuturesOrdered<_> = FuturesOrdered::new();

        // Create the instance around it
        let mut this = Self {
            execute_queue,
        };

        // Add all of the callback futures to it
        for callback in callbacks {
            this.execute_queue.push(Box::pin(this.run_callback(callback, event)));
        }

        // Done
        this
    }

    /// Helper function that runs the given callback as an async
    #[inline]
    async fn run_callback<E>(&self, callback: &'a mut dyn FnMut(&Self, E) -> ThreadedEventResult, event: E) -> ThreadedEventResult {
        callback(self, event)
    }
}
