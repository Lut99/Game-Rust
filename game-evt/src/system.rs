/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:54:47
 * Last edited:
 *   15 May 2022, 16:13:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the actual EventSystem itself.
**/

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use winit::event_loop::EventLoop;

pub use crate::errors::EventSystemError as Error;
use crate::spec::{AsyncEvent, Event, EventKind, EventResult, MetaAsyncEvent, MetaSyncEvent, SyncEvent, SyncResult};


/***** WORKER THREAD *****/
/// Defines one of the worker threads.
pub fn worker(
    syncs       : Arc<Mutex<HashMap<SyncEvent, Vec<Box<dyn FnOnce(SyncEvent) -> SyncResult>>>>>,
    meta_syncs  : Arc<Mutex<HashMap<MetaSyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaSyncEvent) -> SyncResult>>>>>,
    asyncs      : Arc<Mutex<HashMap<AsyncEvent, Vec<Box<dyn FnOnce(AsyncEvent) -> EventResult>>>>>,
    meta_asyncs : Arc<Mutex<HashMap<MetaAsyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaAsyncEvent) -> EventResult>>>>>,

    sync_queue : Arc<Mutex<Vec<SyncEvent>>>,
) -> Result<(), Error> {
    
}





/***** LIBRARY *****/
/// Defines the EventSystem, which is like the nerve center of the game engine.
pub struct EventSystem {
    /// The winit EventLoop which we use for Window events.
    event_loop : EventLoop<()>,

    /// Maps synchronous, non-meta events to their callbacks.
    syncs       : Arc<Mutex<HashMap<SyncEvent, Vec<Box<dyn FnOnce(SyncEvent) -> SyncResult>>>>>,
    /// Maps synchronous meta events to their callbacks.
    meta_syncs  : Arc<Mutex<HashMap<MetaSyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaSyncEvent) -> SyncResult>>>>>,
    /// Maps asynchronous, non-meta events to their callbacks.
    asyncs      : Arc<Mutex<HashMap<AsyncEvent, Vec<Box<dyn FnOnce(AsyncEvent) -> EventResult>>>>>,
    /// Maps asynchronous meta events to their callbacks.
    meta_asyncs : Arc<Mutex<HashMap<MetaAsyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaAsyncEvent) -> EventResult>>>>>,

    /// The worker threads for this system.
    /// Queue for synchronous, non-meta event calls.
    sync_queue : Arc<Mutex<Vec<SyncEvent>>>,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `event_loop`: The EventLoop who's events to pass to the EventSystem.
    /// - `n_threads`: The amount of worker threads for this system.
    pub fn new(event_loop: EventLoop<()>, n_threads: usize) -> Result<Arc<RwLock<Self>>, Error> {
        // Prepare the callback hashmaps
        let syncs       : Arc<Mutex<HashMap<SyncEvent, Vec<Box<dyn FnOnce(SyncEvent) -> SyncResult>>>>>                          = Arc::new(Mutex::new(HashMap::with_capacity(16)));
        let meta_syncs  : Arc<Mutex<HashMap<MetaSyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaSyncEvent) -> SyncResult>>>>>    = Arc::new(Mutex::new(HashMap::with_capacity(16)));
        let asyncs      : Arc<Mutex<HashMap<AsyncEvent, Vec<Box<dyn FnOnce(AsyncEvent) -> EventResult>>>>>                       = Arc::new(Mutex::new(HashMap::with_capacity(16)));
        let meta_asyncs : Arc<Mutex<HashMap<MetaAsyncEvent, Vec<Box<dyn FnOnce(&EventSystem, MetaAsyncEvent) -> EventResult>>>>> = Arc::new(Mutex::new(HashMap::with_capacity(16)));

        // Prepare the vectors for the queues
        let sync_queue: Arc<Mutex<Vec<SyncEvent>>> = Arc::new(Mutex::new(Vec::with_capacity(16)));

        // Spawn n_threads new worker threads
        let mut threads: Vec<thread::JoinHandle<Result<(), Error>>> = Vec::with_capacity(n_threads);
        for i in 0..n_threads {
            threads[i] = match thread::Builder::new()
                .name(format!("game-rust_event-system_{}", i))
                .spawn(|| {
                    worker(
                        syncs.clone(),
                        meta_syncs.clone(),
                        asyncs.clone(),
                        meta_asyncs.clone(),

                        sync_queue.clone(),
                    )
                })
            {
                Ok(thread) => thread,
                Err(err)   => { return Err(Error::ThreadSpawnError{ err }); }
            }
        }

        // Return the new instance (wrapped in locks and such)
        Ok(Arc::new(RwLock::new(Self {
            event_loop,

            syncs,
            meta_syncs,
            asyncs,
            meta_asyncs,

            sync_queue,
        })))
    }



    /// Registers a new callback for a synchronous, non-meta Event.
    /// 
    /// These callbacks _can_ prevent the Event from firing events that are fired after it, but _cannot_ register / deregister callbacks or fire events.
    /// 
    /// Because the event is synchronous, the order in which they are registered matters. It will only be called if none of the previous events blocked the event from propagation.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure.
    /// 
    /// # Arguments
    /// - `event`: The SyncEvent to fire on.
    /// - `callback`: The callback (as `fn callback(SyncEvent) -> SyncResult`) to call when the event is fired.
    pub fn register_sync<F>(this: &Arc<RwLock<Self>>, event: SyncEvent, callback: F)
    where
        F: 'static + FnOnce(SyncEvent) -> SyncResult,
    {
        // Get a write lock
        let mut this = this.write().expect("Could not get write lock on EventSystem");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match this.syncs.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                this.syncs.insert(event, Vec::with_capacity(4));

                // Return the new queue
                this.syncs.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));
    }

    /// Registers a new callback for a synchronous meta Event.
    /// 
    /// These callbacks _can_ prevent the Event from firing events that are fired after it, and _can_ register / deregister callbacks or fire events.
    /// 
    /// Because the event is synchronous, the order in which they are registered matters. It will only be called if none of the previous events blocked the event from propagation.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure.
    /// 
    /// # Arguments
    /// - `event`: The MetaSyncEvent to fire on.
    /// - `callback`: The callback (as `fn callback(&EventSystem, MetaSyncEvent) -> SyncResult`) to call when the event is fired.
    pub fn register_meta_sync<F>(this: &Arc<RwLock<Self>>, event: MetaSyncEvent, callback: F)
    where
        F: 'static + FnOnce(&EventSystem, MetaSyncEvent) -> SyncResult,
    {
        // Get a write lock
        let mut this = this.write().expect("Could not get write lock on EventSystem");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match this.meta_syncs.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                this.meta_syncs.insert(event, Vec::with_capacity(4));

                // Return the new queue
                this.meta_syncs.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));
    }

    /// Registers a new callback for an asynchronous, non-meta Event.
    /// 
    /// These callbacks _cannot_ prevent the Event from firing events that are fired after it, and _cannot_ register / deregister callbacks or fire events.
    /// 
    /// Because the event is asynchronous, the order in which they are registered does not matter.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure.
    /// 
    /// # Arguments
    /// - `event`: The AsyncEvent to fire on.
    /// - `callback`: The callback (as `fn callback(AsyncEvent) -> SyncResult`) to call when the event is fired.
    pub fn register_async<F>(this: &Arc<RwLock<Self>>, event: AsyncEvent, callback: F)
    where
        F: 'static + FnOnce(AsyncEvent) -> EventResult,
    {
        // Get a write lock
        let mut this = this.write().expect("Could not get write lock on EventSystem");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match this.asyncs.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                this.asyncs.insert(event, Vec::with_capacity(4));

                // Return the new queue
                this.asyncs.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));
    }

    /// Registers a new callback for an asynchronous meta Event.
    /// 
    /// These callbacks _cannot_ prevent the Event from firing events that are fired after it, but _can_ register / deregister callbacks or fire events.
    /// 
    /// Because the event is asynchronous, the order in which they are registered does not matter.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure.
    /// 
    /// # Arguments
    /// - `event`: The MetaAsyncEvent to fire on.
    /// - `callback`: The callback (as `fn callback(&EventSystem, MetaAsyncEvent) -> SyncResult`) to call when the event is fired.
    pub fn register_meta_async<F>(this: &Arc<RwLock<Self>>, event: MetaAsyncEvent, callback: F)
    where
        F: 'static + FnOnce(&EventSystem, MetaAsyncEvent) -> EventResult,
    {
        // Get a write lock
        let mut this = this.write().expect("Could not get write lock on EventSystem");

        // Make sure there is an event queue in the HashMap
        let queue: &mut Vec<_> = match this.meta_asyncs.get_mut(&event) {
            Some(queue) => queue,
            None        => {
                // Insert it
                this.meta_asyncs.insert(event, Vec::with_capacity(4));

                // Return the new queue
                this.meta_asyncs.get_mut(&event).unwrap()
            }
        };

        // Add the new callback to it
        queue.push(Box::new(callback));
    }



    /// Fires an event type that will definitely not register / deregister callbacks or schedule events itself.
    /// 
    /// This restriction means not all events may be called this way, but calling them is faster (since no single Write-lock is required).
    /// 
    /// # Generic types
    /// - `E`: The type of the Event to fire.
    /// 
    /// # Arguments
    /// - `event`: The Event to fire.
    /// 
    /// # Returns
    /// The EventResult of the last Event that was fired. This way, it will only be 'Continue' if no Events are called or if all Events were called successfully.
    pub fn fire_nonmeta<E: Debug + Event>(this: &Arc<RwLock<Self>>, event: E) -> EventResult {
        // Match on the kind of Event
        match event.kind() {
            EventKind::Sync => {
                // Get the write lock
                let mut this = this.write().expect("Could not get write lock on EventSystem");

                // Schedule the event on the synchronous queue
                this.sync_queue.push(event);
            },

            EventKind::Async => {

            },

            kind => { panic!("EventSystem::fire_nonmeta() may only called for non-Meta events; not {:?} (which has kind '{:?}')", event, kind); }
        }
    }

    // /// Fires an event, calling all the appropriate callbacks.
    // /// 
    // /// # Arguments
    // /// - `event` The Event to fire.
    // /// 
    // /// # Returns
    // /// The EventResult of the most recent callback. If no callbacks were fired, then returns `EventResult::Continue`.
    // /// 
    // /// # Errors
    // /// This function does not really error, although EventResult may encode errors from Events.
    // pub fn fire(&self, event: Event) -> EventResult {
    //     // Split the callbacks from self (reference-wise)
    //     let Self { ref callbacks, .. } = self;

    //     // If there are any, fire all events of this type
    //     let mut result = EventResult::Continue;
    //     if let Some(callbacks) = callbacks.get(&event) {
    //         // Iterate over the callbacks
    //         for callback in callbacks {
    //             // Quit if the EventResult is one that should let us quit
    //             match callback(self, event) {
    //                 // Stop iteration
    //                 EventResult::Block    |
    //                 EventResult::Exit     |
    //                 EventResult::Error(_) |
    //                 EventResult::Fatal(_) => { break; }

    //                 // Otherwise, keep iterating
    //                 _ => { continue; }
    //             }
    //         }
    //     }

    //     // Return the latest EventResult
    //     result
    // }



    /// Runs the main loop of the game. Whenever this function returns, the game is expected to stop.
    /// 
    /// # Errors
    /// This function may error if something went wrong during the main loop (which is basically everything).
    pub fn run(&mut self) -> Result<(), Error> {
        Ok(())
    }



    /// Returns the EventLoop that is stored within the EventSystem (for initializing new Windows and such).
    #[inline]
    pub fn event_loop(&self) -> &EventLoop<()> { &self.event_loop }
}
