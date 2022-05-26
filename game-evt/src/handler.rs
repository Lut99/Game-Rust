/* HANDLER.rs
 *   by Lut99
 *
 * Created:
 *   21 May 2022, 11:31:00
 * Last edited:
 *   26 May 2022, 16:19:59
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements code to handle a single queue of events within the
 *   EventSystem. This means it's a queue (:0) together with some handler
 *   threads (:00) that handle the events fired on the queue.
**/

use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use async_trait::async_trait;
use futures::future::join_all;

use crate::spec::{Callback, Event, EventResult, ThreadedEventResult};


/***** EVENTHANDLER TRAIT *****/
/// The EventHandler trait, which defines a generalised interface to both the LocalEventHandler and the ThreadedEventHandler.
#[async_trait]
pub trait EventHandler {
    /// The Event type to handle.
    type Event: 'static + Event;
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
    fn register(&self, event: Self::Event, callback: impl Callback<Self::Delegate, Self::Event, Self::EventResult>) -> Result<(), Box<dyn Error>>
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
    async fn fire(&self, event: Self::Event) -> Self::EventResult;
}





/***** EVENT DELEGATES *****/
/// Represents a LocalEventHandler on matching callbacks that may be used to fire new events.
pub struct LocalEventDelegate<E, R> {
    /// A list of callbacks to call for each possible event type.
    callbacks : Arc<RwLock<HashMap<E, Vec<Box<Mutex<dyn Callback<Arc<Self>, E, R>>>>>>>,
}

impl<E, R> LocalEventDelegate<E, R> {
    /// Constructor for the LocalEventDelegate.
    #[inline]
    fn new() -> Arc<Self> {
        Arc::new(Self {
            callbacks : Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
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
    pub fn fire(this: &Arc<Self>, event: E) -> R
    where
        E: Event,
        R: EventResult,
    {
        // Get a read lock
        let map: RwLockReadGuard<HashMap<_, _>> = this.callbacks.read().expect("Could not get read lock on callbacks");

        // If there is an Event to fire, then fire it
        if let Some(callbacks) = map.get(&event) {
            for callback in callbacks {
                // Get the lock on this callback
                let mut callback: MutexGuard<_> = callback.lock().expect("Could not get lock on callback");

                // Call the callback
                let res = callback(this.clone(), event.clone());

                // Continue if it's a Continue value; return otherwise
                if R::cont() == res { continue; }
                return res;
            }
        }

        // Done
        R::cont()
    }
}



/// Implements the delegate for the ThreadedEventHandler, which is the actual handler sent to the callbacks.
pub struct ThreadedEventDelegate<E> {
    /// The list of callbacks from which we fire new events
    callbacks : Arc<RwLock<HashMap<E, Vec<Arc<Mutex<dyn Callback<Arc<Self>, E, ThreadedEventResult>>>>>>>,
}

impl<E> ThreadedEventDelegate<E> {
    /// Constructor for the ThreadedEventDelegate.
    #[inline]
    fn new() -> Arc<Self> {
        Arc::new(Self {
            callbacks : Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }



    /// Fires a new Event while processing callbacks.
    /// 
    /// Results of events are not passed. If any event fails, then it is handled by the EventSystem itself.
    /// 
    /// # Arguments
    /// - `event`: The Event to fire.
    pub async fn fire(this: &Arc<Self>, event: E) -> ThreadedEventResult
    where
        E: 'static + Event,
    {
        // Get the relevant callbacks for thie event as futures
        let futures: Vec<_> = {
            // Get a read lock on the callbacks
            let callbacks: RwLockReadGuard<HashMap<_, _>> = this.callbacks.read().expect("Could not get read lock on callbacks");

            // Get the callbacks for this event as futures
            match callbacks.get(&event) {
                Some(callbacks) => callbacks.iter().map(|clb| ThreadedEventDelegate::run_callback(this.clone(), clb.clone(), event.clone())).collect(),
                None            => { return ThreadedEventResult::Continue; }
            }
        };

        // Execute the list
        let results: Vec<_> = join_all(futures).await;

        // Search for any errors
        for res in results {
            match res {
                ThreadedEventResult::Continue => { continue; }
                res                           => { return res; }
            }
        }

        // Done!
        ThreadedEventResult::Continue
    }

    /// Helper function that runs the given callback as an async
    #[inline]
    async fn run_callback(this: Arc<Self>, callback: Arc<Mutex<dyn Callback<Arc<Self>, E, ThreadedEventResult>>>, event: E) -> ThreadedEventResult {
        callback.lock().expect("Could not lock callback")(this, event)
    }
}





/***** EVENT HANDLERS *****/
/// The LocalEventHandler handles fired events on the local thread.
pub struct LocalEventHandler<E, R> {
    /// The delegate that we use to pass to events
    delegate : Arc<LocalEventDelegate<E, R>>,
}

impl<E, R> LocalEventHandler<E, R> {
    /// Constructor for the LocalEventHandler.
    pub fn new() -> Self {
        Self {
            delegate : LocalEventDelegate::new(),
        }
    }
}

#[async_trait]
impl<E, R> EventHandler for LocalEventHandler<E, R>
where
    E: 'static + Event,
    R: EventResult,
{
    type Delegate = Arc<LocalEventDelegate<E, R>>;
    type Event = E;
    type EventResult = R;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl Callback<Self::Delegate, Self::Event, Self::EventResult>) -> Result<(), Box<dyn Error>> {
        // Get a write lock
        let mut map: RwLockWriteGuard<HashMap<_, _>> = self.delegate.callbacks.write().expect("Could not get write lock on callbacks");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match map.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                map.insert(event.clone(), Vec::with_capacity(4));

                // Return the new queue
                map.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(Mutex::new(callback)));

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
    #[inline]
    async fn fire(&self, event: Self::Event) -> Self::EventResult {
        LocalEventDelegate::fire(&self.delegate, event)
    }
}



/// The ThreadedEventHandler spawns multiple threads and uses those to process the callbacks for fired events.
pub struct ThreadedEventHandler<E>{
    /// A list of callbacks to call for each possible event type.
    delegate : Arc<ThreadedEventDelegate<E>>,
}

impl<E> ThreadedEventHandler<E> {
    /// Constructor for the ThreadedEventHandler.
    pub fn new() -> Self {
        Self {
            delegate : ThreadedEventDelegate::new(),
        }
    }
}

#[async_trait]
impl<E> EventHandler for ThreadedEventHandler<E>
where
    E: 'static + Event,
{
    type Delegate = Arc<ThreadedEventDelegate<E>>;
    type Event = E;
    type EventResult = ThreadedEventResult;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl Callback<Self::Delegate, Self::Event, Self::EventResult>) -> Result<(), Box<dyn Error>> {
        // Get a write lock
        let mut callbacks: RwLockWriteGuard<HashMap<_, _>> = self.delegate.callbacks.write().expect("Could not get write lock on callbacks");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match callbacks.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                callbacks.insert(event.clone(), Vec::with_capacity(4));

                // Return the new queue
                callbacks.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Arc::new(Mutex::new(callback)));

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
    #[inline]
    async fn fire(&self, event: Self::Event) -> Self::EventResult
    where
        E: 'static + Event,
    {
        ThreadedEventDelegate::fire(&self.delegate, event).await
    }
}
