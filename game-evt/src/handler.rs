/* HANDLER.rs
 *   by Lut99
 *
 * Created:
 *   21 May 2022, 11:31:00
 * Last edited:
 *   21 May 2022, 12:39:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements code to handle a single queue of events within the
 *   EventSystem. This means it's a queue (:0) together with some handler
 *   threads (:00) that handle the events fired on the queue.
**/

use std::any::type_name;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::hash::Hash;
use std::panic;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockWriteGuard};
use std::thread::{self, JoinHandle};

use log::{debug, error};

use crate::errors::ThreadedHandlerError;
use crate::spec::EventResult;


/***** CUSTOM TYPES *****/
/// The type of the EventQueues we use.
type EventQueue<H, E> = VecDeque<(Arc<usize>, Arc<RwLock<Vec<Box<dyn FnMut(&H, E) -> EventResult>>>>)>;





/***** WORKER THREADS *****/
/// Defines the code that runs on the worker thread of the ThreadedEventHandler.
/// 
/// # Arguments
/// - `enabled`: Whether or not this thread should continue looping.
/// - `start`: The EventQueue/CondVar pair that launches the thread whenever a new event has landed on the queue.
/// - `stop`: The EventQueue/CondVar pair that is used to signal we are done.
/// - `callbacks`: The callbacks to call.
fn worker_thread<E>(enabled: Arc<bool>, start: Arc<(Arc<Mutex<EventQueue<E>>>, Condvar)>, stop: Arc<(Arc<Mutex<EventQueue<E>>>, Condvar)>) -> Result<(), ThreadedHandlerError> {
    debug!("Spawned tread {}", thread::current().name().unwrap_or("???"));

    // Loop
    loop {
        // Read the next count/event pair from the queue
        let event: (Arc<usize>, E);
        {
            let (lock, cond): &(Arc<Mutex<EventQueue<E>>>, Condvar) = &*start;

            // Get a lock
            let queue: MutexGuard<EventQueue<_>> = match lock.lock() {
                Ok(queue) => queue,
                Err(err)  => { return Err(ThreadedHandlerError::LockError{ what: "queue lock in worker thread", err: format!("{}", err) }); }
            };

            // Wait for something to become available in the queue
            if let Err(err) = cond.wait_while(queue, |queue| queue.is_empty()) {
                return Err(ThreadedHandlerError::LockError{ what: "queue lock in worker thread", err: format!("{}", err) });
            }

            // Pop it, then we can release the lock for other worker threads
            event = queue.pop_front().unwrap();
        }

        // Fire all of the associated callbacks
        {

        }
        // Get a read lock on the callbacks for this event type
        {}
        {
            
        }

        // Wait for the conditional variable to become available
        let (lock, cond) = &*start;
        if let Err(err) = cond.wait_while(lock.lock().map_err(|err| ThreadedHandlerError::LockError{ what: "start lock in worker thread", err: format!("{}", err) })?, |queue| queue.len() == 0) {
            return Err(ThreadedHandlerError::WriteLockError{ what: "start lock in worker thread", err: format!("{}", err) });
        }

        // Acquire a read lock on the queue to get the event


        // Check if we need to stop
        if !*enabled { break; }
    }

    // Done
    Ok(())
}





/***** INTERFACE *****/
/// The EventHandler trait, which defines a generalised interface to both the LocalEventHandler and the ThreadedEventHandler.
pub trait EventHandler {
    /// The Event type to handle.
    type Event;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure / function to call.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register<F>(&self, event: Self::Event, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&Self, Self::Event) -> EventResult;

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
}



/***** EVENT HANDLERS *****/
/// The LocalEventHandler handles fired events on the local thread.
pub struct LocalEventHandler<E>{
    /// A list of callbacks to call for each possible event type.
    callbacks : RefCell<HashMap<E, Vec<Box<dyn FnMut(&Self, E) -> EventResult>>>>,
}

impl<E> LocalEventHandler<E> {
    /// Constructor for the LocalEventHandler.
    pub fn new() -> Self {
        Self {
            callbacks : RefCell::new(HashMap::with_capacity(16)),
        }
    }
}

impl<E> EventHandler for LocalEventHandler<E> {
    type Event = E;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure / function to call.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register<F>(&self, event: Self::Event, callback: F) -> Result<(), Box<dyn Error>>
    where
        E: Eq + Hash,
        F: 'static + FnMut(&Self, Self::Event) -> EventResult
    {
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
    fn fire(&self, event: Self::Event) -> Result<EventResult, Box<dyn Error>>
    where
        E: Eq + Hash,
    {
        // Get a borrow to the hashmap
        let map: Ref<HashMap<_, _>> = self.callbacks.borrow();

        // If there is an Event to fire, then fire it
        if let Some(callbacks) = map.get(&event) {
            for callback in callbacks {
                match callback(self, event) {
                    // Continue the chain if allowed to do so
                    EventResult::Continue => { continue; }

                    // Otherwise, return the result
                    result => { return Ok(result); }
                }
            }
        }

        // Done
        Ok(EventResult::Continue)
    }
}



/// The ThreadedEventHandler spawns multiple threads and uses those to process the callbacks for fired events.
pub struct ThreadedEventHandler<E>{
    /// The queue that we use to pass fired events around
    queue     : Arc<Mutex<EventQueue<Self, E>>>,
    /// A list of callbacks to call for each possible event type.
    callbacks : HashMap<E, Arc<RwLock<Vec<Box<dyn FnMut(&Self, E) -> EventResult>>>>>,

    /// The boolean that enables the threads or causes them to shutdown
    enabled  : Arc<bool>,
    /// The list of consuming worker threads
    handlers : Vec<JoinHandle<Result<(), ThreadedHandlerError>>>,
}

impl<E> ThreadedEventHandler<E> {
    /// Constructor for the ThreadedEventHandler.
    /// 
    /// # Arguments
    /// - `n_threads`: The number of threads to spawn. Cannot be less than 1.
    pub fn new(n_threads: usize) -> Result<Self, ThreadedHandlerError>
    where
        E: Clone,
    {
        // Do a sanity check
        if n_threads < 1 { panic!("`n_threads` cannot be less than 1"); }

        // Prepare the queue
        let queue: Arc<_> = Arc::new(Mutex::new(VecDeque::with_capacity(16)));

        // Prepare the list of callbacks
        let callbacks: HashMap<_, _> = HashMap::with_capacity(16);

        // Spawn the handler threads
        let enabled: Arc<bool> = Arc::new(true);
        let mut handlers: Vec<JoinHandle<Result<(), ThreadedHandlerError>>> = Vec::with_capacity(n_threads);
        for i in 0..n_threads {
            handlers.push(match thread::Builder::new()
                .name(format!("{}_worker-{}", type_name::<ThreadedEventHandler<E>>(), i))
                .spawn(|| {
                    worker_thread(enabled.clone())
                })
            {
                Ok(handle) => handle,
                Err(err)   => { return Err(ThreadedHandlerError::ThreadSpawnError{ err }); }
            });
        };

        // Done, return us
        Ok(Self {
            queue,
            callbacks,

            enabled,
            handlers,
        })
    }
}

impl<E> EventHandler for ThreadedEventHandler<E> {
    type Event = E;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure / function to call.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register<F>(&self, event: Self::Event, callback: F) -> Result<(), Box<dyn Error>>
    where
        E: Eq + Hash,
        F: 'static + FnMut(&Self, Self::Event) -> EventResult
    {
        // Get a write lock on the hashmap
        let mut map: RwLockWriteGuard<HashMap<_, _>> = match self.callbacks.write() {
            Ok(map)  => map,
            Err(err) => { return Err(Box::new(ThreadedHandlerError::WriteLockError{ what: "callbacks map", err: format!("{}", err) })); }
        };

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
    fn fire(&self, event: Self::Event) -> Result<EventResult, Box<dyn Error>>
    where
        E: Clone + Eq + Hash,
    {
        // Get a lock for the callback, to make sure nobody writes to it
        let callbacks = match self.callbacks.read() {
            Ok(queue) => queue,
            Err(err)  => { return Err(Box::new(ThreadedHandlerError::ReadLockError{ what: "event queue", err: format!("{}", err) })); }
        };

        // Get the length for this event call
        let n_callbacks: usize = callbacks.get(&event).map(|v| v.len()).unwrap_or(0);

        // Write the event on the local queue
        {
            // Get the write lock
            let mut queue: MutexGuard<EventQueue<_>> = match self.queue.lock() {
                Ok(queue) => queue,
                Err(err)  => { return Err(Box::new(ThreadedHandlerError::WriteLockError{ what: "event queue", err: format!("{}", err) })); }
            };

            // Push it
            queue.push_back((Arc::new(n_callbacks), event));

            // Signal the conditional variable
            
        }

        // Now wait until the Event has completed
        

        // Get a borrow to the hashmap
        let map: Ref<HashMap<_, _>> = self.callbacks.borrow();

        // If there is an Event to fire, then fire it
        if let Some(callbacks) = map.get(&event) {
            for callback in callbacks {
                match callback(self, event) {
                    // Continue the chain if allowed to do so
                    EventResult::Continue => { continue; }

                    // Otherwise, return the result
                    result => { return Ok(result); }
                }
            }
        }

        // Done
        Ok(EventResult::Continue)
    }
}

impl<E> Drop for ThreadedEventHandler<E> {
    fn drop(&mut self) {
        // Stop the worker threads and join them
        *self.enabled = false;
        for handle in self.handlers {
            match handle.join() {
                Ok(res)  => if let Err(err) = res {
                    error!("{} worker thread failed: {}", type_name::<ThreadedEventHandler<E>>(), err);   
                },
                Err(err) => { panic::resume_unwind(err); }   
            }
        }
    }
}
