/* HANDLER.rs
 *   by Lut99
 *
 * Created:
 *   21 May 2022, 11:31:00
 * Last edited:
 *   25 May 2022, 20:16:24
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
use std::fmt::Debug;
use std::hash::Hash;
use std::panic;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{self, JoinHandle};

use log::{debug, error, warn};

use crate::errors::ThreadedHandlerError;
use crate::spec::{Callback, EventHandler, EventResult};


/***** CUSTOM TYPES *****/
/// The type of the EventQueues we use.
type EventQueue<H, E> = VecDeque<(Arc<dyn Callback<H, E>>, E)>;





/***** WORKER THREADS *****/
/// Defines the code that runs on the worker thread of the ThreadedEventHandler.
/// 
/// # Arguments
/// - `handler`: The ThreadedEventHandler for which we work. Both contains stuff like the enabled var and the event queue, as being able to pass it to callbacks.
fn worker_thread<E>(
    handler: Arc<ThreadedEventHandler<E>>,
) -> Result<(), ThreadedHandlerError>
where
    E: Send + Sync + Debug,
{
    debug!("Spawned tread {}", thread::current().name().unwrap_or("???"));

    // Loop
    loop {
        // Read the next count/event pair from the queue
        let event: (Arc<dyn Callback<ThreadedEventHandler<E>, E>>, E);
        {
            // Get a lock
            let queue: MutexGuard<EventQueue<_, _>> = match handler.queue.lock() {
                Ok(queue) => queue,
                Err(err)  => { return Err(ThreadedHandlerError::LockError{ what: "event queue in worker thread", err: format!("{}", err) }); }
            };

            // Wait for something to become available in the queue
            if let Err(err) = handler.signal.wait_while(queue, |queue| queue.is_empty()) {
                return Err(ThreadedHandlerError::LockError{ what: "event queue in worker thread (aft condvar)", err: format!("{}", err) });
            }

            // Pop it, then we can release the lock for other worker threads
            event = queue.pop_front().unwrap();
        }

        // Process the callback
        match event.0(handler.as_ref(), event.1) {
            // Everything's going fine (do nothing)
            EventResult::Continue => {},

            // Ignore Block too
            EventResult::Block => { warn!("EventResult::Block returned from threaded event callback (no use)"); }
            // On exit, set the flag in the handler
            EventResult::Exit  => { *handler.enabled = false; }

            // If Error, then notify the user
            EventResult::Error(err) => { error!("{}::{:?} callback: {}", type_name::<E>(), event.1, err); }
            // If Fatal, then notify the user _and_ quit
            EventResult::Fatal(err) => { error!("{}::{:?} callback: {}", type_name::<E>(), event.1, err); *handler.enabled = false; }
        }

        // Check if we need to stop
        if !*handler.enabled { break; }
    }

    // Done
    Ok(())
}





/***** EVENT HANDLERS *****/
/// The LocalEventHandler handles fired events on the local thread.
pub struct LocalEventHandler<E>{
    /// A list of callbacks to call for each possible event type.
    callbacks : RefCell<HashMap<E, Vec<Box<dyn Callback<Self, E>>>>>,
}

impl<E> LocalEventHandler<E> {
    /// Constructor for the LocalEventHandler.
    pub fn new() -> Self {
        Self {
            callbacks : RefCell::new(HashMap::with_capacity(16)),
        }
    }
}

impl<E> EventHandler for LocalEventHandler<E>
where
    E: Eq + Hash,
{
    type Event = E;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Callback<Self, Self::Event>) -> Result<(), Box<dyn Error>> {
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



    /// Stops the EventHandler (telling threads to stop and junk)
    #[inline]
    fn stop(&mut self) {
        // Nothing to do
    }
}



/// The ThreadedEventHandler spawns multiple threads and uses those to process the callbacks for fired events.
pub struct ThreadedEventHandler<E>{
    /// The queue that we use to pass fired events around
    queue     : Arc<Mutex<EventQueue<Self, E>>>,
    /// A list of callbacks to call for each possible event type.
    callbacks : RwLock<HashMap<E, Vec<Arc<dyn Callback<Self, E>>>>>,

    /// The boolean that enables the threads or causes them to shutdown
    enabled  : Arc<bool>,
    /// The conditional variable used for waking threads.
    signal   : Arc<Condvar>,
    /// The list of consuming worker threads
    handlers : Vec<JoinHandle<Result<(), ThreadedHandlerError>>>,
}

impl<E> ThreadedEventHandler<E> {
    /// Constructor for the ThreadedEventHandler.
    /// 
    /// # Arguments
    /// - `n_threads`: The number of threads to spawn. Cannot be less than 1.
    pub(crate) fn new(n_threads: usize) -> Result<Arc<Self>, ThreadedHandlerError>
    where
        E: 'static + Send + Sync + Debug,
    {
        // Do a sanity check
        if n_threads < 1 { panic!("`n_threads` cannot be less than 1"); }

        // Prepare the queue
        let queue: Arc<Mutex<EventQueue<_, _>>> = Arc::new(Mutex::new(VecDeque::with_capacity(16)));
        // Prepare the list of callbacks
        let callbacks = RwLock::new(HashMap::with_capacity(16));

        // Prepare the enabled & conditional variables
        let enabled: Arc<bool> = Arc::new(true);
        let signal = Arc::new(Condvar::new());

        // Wrap them in an instance we may pass to the threads
        let this = Arc::new(Self {
            queue,
            callbacks,

            enabled,
            signal,
            handlers : Vec::with_capacity(n_threads),
        });

        // Spawn the handler threads
        for i in 0..n_threads {
            let handle = match thread::Builder::new()
                .name(format!("game_rust-{}_worker-{}", type_name::<ThreadedEventHandler<E>>(), i))
                .spawn(|| {
                    worker_thread(this.clone())
                })
            {
                Ok(handle) => handle,
                Err(err)   => { return Err(ThreadedHandlerError::ThreadSpawnError{ err }); }
            };
        };

        // Done, return us
        Ok(this)
    }
}

impl<E> EventHandler for ThreadedEventHandler<E>
where
    E: Eq + Hash,
{
    type Event = E;


    /// Registers a new callback for the given Event type.
    /// 
    /// # Arguments
    /// - `event`: The specific Event variant to fire on.
    /// - `callback`: The function to register for calling once `event` has fired.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    fn register(&self, event: Self::Event, callback: impl 'static + Callback<Self, Self::Event>) -> Result<(), Box<dyn Error>> {
        // Get the write lock
        let mut callbacks: RwLockWriteGuard<HashMap<_, _>> = match self.callbacks.write() {
            Ok(callbacks) => callbacks,
            Err(err)      => { return Err(Box::new(ThreadedHandlerError::WriteLockError{ what: "callback map", err: format!("{}", err) })); }
        };

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match callbacks.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                callbacks.insert(event, Vec::with_capacity(4));

                // Return the new queue
                callbacks.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Arc::new(callback));

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
    /// Because this is the multi-threaded implementation, it always returns 'EventResult::Continue'. However, any other results are processed to (roughly) result in the same outcome.
    /// 
    /// # Errors
    /// This function may error if the actual struct does (for example, could not get a lock).
    #[inline]
    fn fire(&self, event: Self::Event) -> Result<EventResult, Box<dyn Error>>
    where
        E: Eq + Hash,
    {
        // Get a read lock on the callback list
        let callbacks: RwLockReadGuard<HashMap<_, _>> = match self.callbacks.read() {
            Ok(callbacks) => callbacks,
            Err(err)      => { return Err(Box::new(ThreadedHandlerError::ReadLockError{ what: "callback map", err: format!("{}", err) })); }
        };

        // Get the write lock on the queue
        let mut queue: MutexGuard<EventQueue<_, _>> = match self.queue.lock() {
            Ok(queue) => queue,
            Err(err)  => { return Err(Box::new(ThreadedHandlerError::WriteLockError{ what: "event queue", err: format!("{}", err) })); }
        };

        // Push the callbacks to the queue
        if let Some(callbacks) = callbacks.get(&event) {
            for callback in callbacks {
                queue.push_back((callback.clone(), event));
            }
        }

        // Notify the worker threads to wakeup (if they weren't running already)
        self.signal.notify_all();

        // Done (we don't wait)
        Ok(EventResult::Continue)
    }



    /// Stops the EventHandler (telling threads to stop and junk)
    #[inline]
    fn stop(&mut self) {
        // Set stop. When we're dropped, we'll properly stop.
        *self.enabled = false;
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
