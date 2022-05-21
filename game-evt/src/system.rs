/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:54:47
 * Last edited:
 *   21 May 2022, 11:27:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the actual EventSystem itself.
**/

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use log::debug;
use winit::event_loop::EventLoop;
use queues::{IsQueue, Queue};

pub use crate::errors::EventSystemError as Error;
use crate::spec::{Event, EventResult, InputEvent, TickEvent, WindowEvent};


/***** WORKER THREADS *****/
/// The code of the timer thread (which generates tick events)
/// 
/// # Arguments
/// - `running`: Reference to a boolean indiciating whether this thread should be stopped or not. The thread will always run at least once.
/// - `interval`: The interval (in milliseconds) between two Tick events.
/// - `event_queue`: The event queue (as a Queue) to fire new Tick events on.
/// 
/// # Errors
/// This function does not error (yet).
fn timer_thread(running: Arc<bool>, interval: u64, event_queue: Arc<RwLock<Queue<TickEvent>>>) -> Result<(), Error> {
    debug!("Spawned timer thread");

    // Loop while we're running
    loop {
        // Wait until the given amount of time has passed
        thread::sleep(Duration::from_millis(interval));

        // Fire the event
        {
            // Get the lock
            let queue = event_queue.write().expect("Could not get a write lock for the tick event queue");
            queue.add(TickEvent::Tick);
        }

        // Check if we should stop
        if !*running {
            break;
        }
    }

    // Done
    Ok(())
}



/// A thread that handles input events specifically.
/// 
/// # Arguments
/// - `running`: Reference to a boolean indiciating whether this thread should be stopped or not. The thread will always run at least once.
/// - `event_queue`: The queue with new events in it.
/// - `callbacks`: The callbacks that we need to fire based on the events we see.
fn input_thread(running: Arc<bool>, event_queue: Arc<RwLock<Queue<InputEvent>>>, callbacks: Arc<Mutex<HashMap<InputEvent, Vec<Box<dyn Send + Sync + FnMut(InputEvent) -> EventResult>>>>>) -> Result<(), Error> {
    debug!("Spawned input worker thread");

    // Loop while we're running
    loop {
        // Wait until we're awoken by the signal

        // We're the only one, so lock the thread

        // Check if we should stop
        if !*running {
            break;
        }
    }

    // Done
    Ok(())
}



/// A thread that handles tick events specifically.
/// 
/// # Arguments
/// - `running`: Reference to a boolean indiciating whether this thread should be stopped or not. The thread will always run at least once.
/// - `index`: The index of this thread.
/// - `event_queue`: The queue with new events in it.
/// - `callbacks`: The callbacks that we need to fire based on the events we see.
fn tick_thread(running: Arc<bool>, index: usize, event_queue: Arc<RwLock<Queue<TickEvent>>>, callbacks: Arc<RwLock<HashMap<TickEvent, Vec<Box<dyn Send + Sync + FnMut(TickEvent) -> EventResult>>>>>) -> Result<(), Error> {
    debug!("Spawned tick worker thread {}", index);
    
    // Loop while we're running
    loop {
        // Check if we should stop
        if !*running {
            break;
        }
    }

    // Done
    Ok(())
}





/***** LIBRARY *****/
/// Defines the EventSystem, which is like the nerve center of the game engine.
pub struct EventSystem {
    /// The winit EventLoop which we use for Window events.
    event_loop : EventLoop<()>,

    /// Maps Winit events to their callbacks.
    window_callbacks : RefCell<HashMap<WindowEvent, Vec<Box<dyn FnMut(WindowEvent) -> EventResult>>>>,
    /// Maps input events to their callbacks.
    input_callbacks  : Arc<RwLock<HashMap<InputEvent, Vec<Box<dyn Send + Sync + FnMut(InputEvent) -> EventResult>>>>>,
    /// Maps other (tick) events to their callbacks.
    tick_callbacks   : Arc<RwLock<HashMap<TickEvent, Vec<Box<dyn Send + Sync + FnMut(TickEvent) -> EventResult>>>>>,

    /// The queue that writes input events that have been fired
    input_queue   : Arc<RwLock<Queue<InputEvent>>>,
    /// The queue that writes tick events that are fired
    tick_queue    : Arc<RwLock<Queue<TickEvent>>>,

    /// The boolean indicating whether to run the threads
    enabled      : Arc<bool>,
    /// The timer thread
    timer_thread : JoinHandle<Result<(), Error>>,
    /// The input thread
    input_thread : JoinHandle<Result<(), Error>>,
    /// The tick threads
    tick_threads : Vec<JoinHandle<Result<(), Error>>>,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `event_loop`: The EventLoop who's events to pass to the EventSystem.
    /// - `interval`: The interval (in milliseconds) between two Tick events.
    /// - `n_threads`: The number of auxillary threads to launch that handle tick events. In addition to this, the EventLoop will hijack the main thread (for Window events) and a timer thread (to spawn ticks).
    pub fn new(event_loop: EventLoop<()>, interval: u64, n_threads: usize) -> Result<Self, Error> {
        // Prepare the callback hashmaps
        let window_callbacks : RefCell<HashMap<WindowEvent, Vec<Box<dyn FnMut(WindowEvent) -> EventResult>>>>                 = RefCell::new(HashMap::with_capacity(16));
        let input_callbacks  : Arc<RwLock<HashMap<InputEvent, Vec<Box<dyn Send + Sync + FnMut(InputEvent) -> EventResult>>>>> = Arc::new(RwLock::new(HashMap::with_capacity(16)));
        let tick_callbacks   : Arc<RwLock<HashMap<TickEvent, Vec<Box<dyn Send + Sync + FnMut(TickEvent) -> EventResult>>>>>   = Arc::new(RwLock::new(HashMap::with_capacity(16)));

        // Prepare the queues
        let input_queue : Arc<RwLock<Queue<InputEvent>>> = Arc::new(RwLock::new(Queue::new()));
        let tick_queue : Arc<RwLock<Queue<TickEvent>>>   = Arc::new(RwLock::new(Queue::new()));

        // Spawn the timer thread
        let enabled: Arc<bool> = Arc::new(true);
        let timer_thread: JoinHandle<Result<(), Error>> = match thread::Builder::new()
            .name("game-rust_event-system_timer".to_string())
            .spawn(|| {
                timer_thread(enabled.clone(), interval, tick_queue.clone())
            })
        {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::ThreadSpawnError{ err }); }
        };

        // Spawn the input thread
        let input_thread: JoinHandle<Result<(), Error>> = match thread::Builder::new()
            .name("game-rust_event-system_input".to_string())
            .spawn(|| {
                input_thread(enabled.clone(), input_queue.clone(), input_callbacks.clone())
            })
        {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::ThreadSpawnError{ err }); }
        };

        // We will use the main thread for window / render events. Other thread(s) will handle timed events or other input sources.
        let tick_threads: Vec<JoinHandle<Result<(), Error>>> = Vec::with_capacity(n_threads);
        for i in 0..n_threads {
            tick_threads.push(
                match thread::Builder::new()
                    .name(format!("game-rust_event-system_tick-{}", i))
                    .spawn(|| {
                        tick_thread(enabled.clone(), i, tick_queue.clone(), tick_callbacks.clone())
                    })
                {
                    Ok(handle) => handle,
                    Err(err)   => { return Err(Error::ThreadSpawnError{ err }); }
                }
            );
        }

        // Return the new instance (wrapped in locks and such)
        Ok(Self {
            event_loop,

            window_callbacks,
            input_callbacks,
            tick_callbacks,

            input_queue,
            tick_queue,

            enabled,
            timer_thread,
            input_thread,
            tick_threads,
        })
    }



    /// Registers a new callback for the given (Tick)Event.
    /// 
    /// # Generic types
    /// - `F`: The type of the closure.
    /// 
    /// # Arguments
    /// - `event`: The Event to fire on.
    /// - `callback`: The callback (as `fn callback(&EventSystem, Event) -> EventResult`) to call when the event is fired.
    pub fn register<F>(&self, event: Event, callback: F)
    where
        F: 'static + Fn(&EventSystem, Event) -> EventResult,
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
    }



    /// Fires an event type that will definitely not register / deregister callbacks or schedule events itself.
    /// 
    /// This restriction means not all events may be called this way, but calling them is faster (since no single Write-lock is required).
    /// 
    /// # Arguments
    /// - `event`: The Event to fire.
    /// 
    /// # Returns
    /// The EventResult of the last Event. This way, if the event cancels the chain, then it will be visible through this value.
    pub fn fire(&self, event: Event) -> EventResult {
        // Borrow the map
        let map: Ref<HashMap<_, _>> = self.callbacks.borrow();

        // Fire all callbacks for this event
        if let Some(queue) = map.get(&event) {
            // Handle all of the registered callbacks
            for callback in queue.iter() {
                match callback(self, event) {
                    // Continue fireing the events in this queue.
                    EventResult::Continue => { continue; }

                    // In all other cases, just return as-is
                    result => { return result; }
                }
            }
        }

        // Everything went OK
        EventResult::Continue
    }



    /// Runs the main loop of the game. Whenever this function returns, the game is expected to stop.
    /// 
    /// # Errors
    /// This function may error if something went wrong during the main loop (which is basically everything).
    pub fn run(&mut self) -> Result<(), Error> {
        // Simply run the EventLoop
        
    }



    /// Returns the EventLoop that is stored within the EventSystem (for initializing new Windows and such).
    #[inline]
    pub fn event_loop(&self) -> &EventLoop<()> { &self.event_loop }
}
