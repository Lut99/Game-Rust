/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:54:47
 * Last edited:
 *   22 May 2022, 14:26:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the actual EventSystem itself.
**/

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use log::{debug, error};
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub use crate::errors::EventSystemError as Error;
use crate::spec::{Callback, ControlEvent, Event, EventHandler, EventKind, EventResult, InputEvent, TickEvent, WindowEvent};
use crate::handler::{LocalEventHandler, ThreadedEventHandler};


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
fn timer_thread(running: Arc<bool>, interval: u64, tick_handler: Arc<ThreadedEventHandler<TickEvent>>) -> Result<(), Error> {
    debug!("Spawned timer thread");

    // Loop while we're running
    loop {
        // Wait until the given amount of time has passed
        thread::sleep(Duration::from_millis(interval));

        // Fire the event
        if let Err(err) = tick_handler.fire(TickEvent::Tick) { return Err(Error::TickFireError{ err }); };

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

    /// Maps some events (like Exit) to callbacks.
    control_handler : LocalEventHandler<ControlEvent>,
    /// Maps Winit events to their callbacks.
    window_handler  : LocalEventHandler<WindowEvent>,
    /// Maps input events to their callbacks.
    input_handler   : Arc<ThreadedEventHandler<InputEvent>>,
    /// Maps other (tick) events to their callbacks.
    tick_handler    : Arc<ThreadedEventHandler<TickEvent>>,

    /// The boolean indicating whether to run the timer thread
    enabled      : Arc<bool>,
    /// The timer thread
    timer_thread : JoinHandle<Result<(), String>>,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `event_loop`: The EventLoop who's events to pass to the EventSystem.
    /// - `tick_interval`: The interval (in milliseconds) between two Tick events.
    /// - `input_threads`: The number of auxillary threads to launch that handle input events. In addition to this, the EventLoop will hijack the main thread (for Window events) and a timer thread (to spawn ticks).
    /// - `tick_threads`: The number of auxillary threads to launch that handle tick events. In addition to this, the EventLoop will hijack the main thread (for Window events) and a timer thread (to spawn ticks).
    pub fn new(event_loop: EventLoop<()>, tick_interval: u64, input_threads: usize, tick_threads: usize) -> Result<Self, Error> {
        // Prepare the handlers
        let control_handler       = LocalEventHandler::new();
        let window_handler        = LocalEventHandler::new();
        let input_handler: Arc<_> = match ThreadedEventHandler::new(input_threads) {
            Ok(handler) => handler,
            Err(err)    => { return Err(Error::HandlerCreateError{ what: "input", err: Box::new(err) }); }
        };
        let tick_handler: Arc<_>  = match ThreadedEventHandler::new(tick_threads) {
            Ok(handler) => handler,
            Err(err)    => { return Err(Error::HandlerCreateError{ what: "tick", err: Box::new(err) }); }
        };

        // Spawn the timer thread
        let enabled: Arc<bool> = Arc::new(true);
        let timer_thread: JoinHandle<Result<(), String>> = match thread::Builder::new()
            .name("game_rust-event_system-timer".to_string())
            .spawn(|| {
                timer_thread(enabled.clone(), tick_interval, tick_handler.clone())
                    .map_err(|err| format!("{}", err))
            })
        {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::TimerThreadSpawnError{ err }); }
        };

        // Return the new instance (wrapped in locks and such)
        Ok(Self {
            event_loop,

            control_handler,
            window_handler,
            input_handler,
            tick_handler,

            enabled,
            timer_thread,
        })
    }



    /// Registers a new callback for the given ControlEvent.
    /// 
    /// # Arguments
    /// - `event`: The ControlEvent to fire on.
    /// - `callback`: The callback to call when the event is fired.
    /// 
    /// # Errors
    /// This function might error if the handler where the ControlEvent is scheduled does.
    #[inline]
    pub fn register_control<F>(&self, event: ControlEvent, callback: F) -> Result<(), Error>
    where
        F: 'static + Callback<LocalEventHandler<ControlEvent>, ControlEvent>,
    {
        // Register with the appropriate handler
        self.control_handler.register(event, callback).map_err(|err| Error::RegisterError{ err })
    }

    /// Registers a new callback for the given WindowEvent.
    /// 
    /// # Arguments
    /// - `event`: The WindowEvent to fire on.
    /// - `callback`: The callback to call when the event is fired.
    /// 
    /// # Errors
    /// This function might error if the handler where the WindowEvent is scheduled does.
    #[inline]
    pub fn register_window<F>(&self, event: WindowEvent, callback: F) -> Result<(), Error>
    where
        F: 'static + Callback<LocalEventHandler<WindowEvent>, WindowEvent>,
    {
        // Register with the appropriate handler
        self.window_handler.register(event, callback).map_err(|err| Error::RegisterError{ err })
    }

    /// Registers a new callback for the given InputEvent.
    /// 
    /// # Arguments
    /// - `event`: The InputEvent to fire on.
    /// - `callback`: The callback to call when the event is fired.
    /// 
    /// # Errors
    /// This function might error if the handler where the InputEvent is scheduled does.
    #[inline]
    pub fn register_input<F>(&self, event: InputEvent, callback: F) -> Result<(), Error>
    where
        F: 'static + Callback<ThreadedEventHandler<InputEvent>, InputEvent>,
    {
        // Register with the appropriate handler
        self.input_handler.register(event, callback).map_err(|err| Error::RegisterError{ err })
    }

    /// Registers a new callback for the given TickEvent.
    /// 
    /// # Arguments
    /// - `event`: The TickEvent to fire on.
    /// - `callback`: The callback to call when the event is fired.
    /// 
    /// # Errors
    /// This function might error if the handler where the TickEvent is scheduled does.
    #[inline]
    pub fn register_tick<F>(&self, event: TickEvent, callback: F) -> Result<(), Error>
    where
        F: 'static + Callback<ThreadedEventHandler<TickEvent>, TickEvent>,
    {
        // Register with the appropriate handler
        self.tick_handler.register(event, callback).map_err(|err| Error::RegisterError{ err })
    }



    /// Fires an event.
    /// 
    /// # Generic types
    /// - `E`: The type of the Event to fire. This determines the backend handler to call.
    /// 
    /// # Arguments
    /// - `event`: The Event to fire.
    /// 
    /// # Returns
    /// The EventResult of the last Event (if running on a LocalEventHandler). This way, if the event cancels the chain, then it will be visible through this value.
    pub fn fire<E>(&self, event: E) -> Result<EventResult, Error>
    where
        E: Event,
    {
        // Match the kind
        match event.kind() {
            EventKind::ControlEvent => self.control_handler.fire(*event.as_t::<ControlEvent>()).map_err(|err| Error::FireError{ event: format!("ControlEvent::{:?}", event), err }),
            EventKind::WindowEvent  => self.window_handler.fire(*event.as_t::<WindowEvent>()).map_err(|err| Error::FireError{ event: format!("WindowEvent::{:?}", event), err }),
            EventKind::InputEvent   => self.input_handler.fire(*event.as_t::<InputEvent>()).map_err(|err| Error::FireError{ event: format!("InputEvent::{:?}", event), err }),
            EventKind::TickEvent    => self.tick_handler.fire(*event.as_t::<TickEvent>()).map_err(|err| Error::FireError{ event: format!("TickEvent::{:?}", event), err }),
        }
    }



    /// Runs the main loop of the game. Whenever this function returns, the game is expected to stop.
    /// 
    /// # Errors
    /// This function may error if something went wrong during the main loop (which is basically everything).
    pub fn run(mut self) -> Result<(), Error> {
        // Split self
        let Self { event_loop, control_handler, .. } = self;

        // Simply run the EventLoop
        event_loop.run(move |event, _, control_flow| {
            // Switch on the event type
            match event {
                | WinitEvent::WindowEvent{ window_id: _window_id, event } => {
                    // Match the event again
                    match event {
                        | WinitWindowEvent::CloseRequested => {
                            // For now, we close on _any_ window close, but this should obviously be marginally more clever
                            *control_flow = ControlFlow::Exit;
                        },
    
                        // Ignore the others
                        _ => {}
                    }
                },
    
                | WinitEvent::MainEventsCleared => {
                    // Ask all windows to redraw
                    match self.window_handler.fire(WindowEvent::RequestRedraw) {
                        Ok(res) => match res {
                            EventResult::Continue => {}

                            EventResult::Exit => { *control_flow = ControlFlow::Exit; }

                            EventResult::Error(err) => { error!("Error while requesting Window redraw: {}", err); }
                            EventResult::Fatal(err) => { error!("Error while requesting Window redraw: {}", err); *control_flow = ControlFlow::Exit; }

                            _ => {}
                        },
                        Err(err) => { error!("{}", Error::FireError{ event: format!("WindowEvent::{:?}", WindowEvent::RequestRedraw), err }); }
                    };
                },
    
                | WinitEvent::RedrawRequested(window_id) => {
                    // Redraw a specific window
                    match self.window_handler.fire(WindowEvent::Redraw(window_id)) {
                        Ok(res) => match res {
                            EventResult::Continue => {}

                            EventResult::Exit => { *control_flow = ControlFlow::Exit; }

                            EventResult::Error(err) => { error!("Error while redrawing Window {:?}: {}", window_id, err); }
                            EventResult::Fatal(err) => { error!("Error while redrawing Window {:?}: {}", window_id, err); *control_flow = ControlFlow::Exit; }

                            _ => {}
                        },
                        Err(err) => { error!("{}", Error::FireError{ event: format!("WindowEvent::{:?}", WindowEvent::Redraw(window_id)), err }); }
                    };
                },
    
                // We do nothing for all other events
                _ => {}
            }

            // Call closing events if necessary
            if let ControlFlow::Exit = *control_flow {
                // Fire the ControlHandler
                if let Err(err) = self.control_handler.fire(ControlEvent::Closing) { error!("Failed to run closing events: {}", err); };

                // Tell all handlers to stop
                self.control_handler.stop();
                self.window_handler.stop();
                self.input_handler.stop();
                self.tick_handler.stop();
            }
        });
    }



    /// Returns the EventLoop that is stored within the EventSystem (for initializing new Windows and such).
    #[inline]
    pub fn event_loop(&self) -> &EventLoop<()> { &self.event_loop }
}

impl Drop for EventSystem {
    fn drop(&mut self) {
        // Stop the worker threads and join them
        *self.enabled = false;
        match self.timer_thread.join() {
            Ok(res)  => if let Err(err) = res {
                error!("EventSystem timer thread failed: {}", err);   
            },
            Err(err) => { panic::resume_unwind(err); }   
        }
    }
}
