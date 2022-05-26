/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   15 May 2022, 11:54:47
 * Last edited:
 *   26 May 2022, 16:37:00
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the actual EventSystem itself.
**/

use std::time::Duration;

use log::error;
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub use crate::errors::EventSystemError as Error;
use crate::spec::{Callback, ControlEvent, ControlEventResult, EventResult, InputEvent, LocalEventResult, ThreadedEventResult, TickEvent, WindowEvent};
use crate::handler::{EventHandler, LocalEventDelegate, LocalEventHandler, ThreadedEventDelegate, ThreadedEventHandler};


/***** LIBRARY *****/
/// Defines the EventSystem, which is like the nerve center of the game engine.
pub struct EventSystem {
    /// The winit EventLoop which we use for Window events.
    event_loop : EventLoop<()>,

    /// Maps some events (like Exit) to callbacks.
    control_handler : LocalEventHandler<ControlEvent, ControlEventResult>,
    /// Maps Winit events to their callbacks.
    window_handler  : LocalEventHandler<WindowEvent, LocalEventResult>,
    /// Maps input events to their callbacks.
    input_handler   : ThreadedEventHandler<InputEvent>,
    /// Maps other (tick) events to their callbacks.
    tick_handler    : ThreadedEventHandler<TickEvent>,
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
        let control_handler = LocalEventHandler::new();
        let window_handler  = LocalEventHandler::new();
        let input_handler   = ThreadedEventHandler::new();
        let tick_handler    = ThreadedEventHandler::new();

        // Start the tick callback
        tick_handler.register(TickEvent::Tick, |delegate, _| {
            // Wait the interval
            tokio::time::sleep(Duration::from_millis(tick_interval))
        });

        // Return the new instance
        Ok(Self {
            event_loop,

            control_handler,
            window_handler,
            input_handler,
            tick_handler,
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
    pub fn register_control(&self, event: ControlEvent, callback: impl Callback<LocalEventDelegate<ControlEvent, ControlEventResult>, ControlEvent, ControlEventResult>) -> Result<(), Error> {
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
    pub fn register_window(&self, event: WindowEvent, callback: impl Callback<WindowEvent, LocalEventHandler<WindowEvent, LocalEventResult>, LocalEventResult>) -> Result<(), Error> {
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
    pub fn register_input(&self, event: InputEvent, callback: impl Callback<InputEvent, ThreadedEventDelegate<InputEvent>, ThreadedEventResult>) -> Result<(), Error> {
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
    pub fn register_tick(&self, event: TickEvent, callback: impl Callback<TickEvent, ThreadedEventDelegate<TickEvent>, ThreadedEventResult>) -> Result<(), Error> {
        // Register with the appropriate handler
        self.tick_handler.register(event, callback).map_err(|err| Error::RegisterError{ err })
    }



    /// Fires a control event.
    /// 
    /// # Arguments
    /// - `event`: The ControlEvent to fire.
    /// 
    /// # Returns
    /// ControlEventResult::Continue if all callbacks fired successfully, or else the exit state if it crashed or wanted to exit.
    #[inline]
    pub async fn fire_control(&self, event: ControlEvent) -> ControlEventResult {
        // Match the kind
        self.control_handler.fire(event).await
    }

    /// Fires a window event.
    /// 
    /// # Arguments
    /// - `event`: The WindowEvent to fire.
    /// 
    /// # Returns
    /// LocalEventResult::Continue if all callbacks fired successfully, or else the exit state if it crashed or wanted to block.
    #[inline]
    pub async fn fire_window(&self, event: WindowEvent) -> LocalEventResult {
        // Match the kind
        self.window_handler.fire(event).await
    }

    /// Fires an input event.
    /// 
    /// # Arguments
    /// - `event`: The InputEvent to fire.
    /// 
    /// # Returns
    /// ThreadedEventResult::Continue if all callbacks fired successfully, or else the exit state if it crashed.
    #[inline]
    pub async fn fire_input(&self, event: InputEvent) -> ThreadedEventResult {
        // Match the kind
        self.input_handler.fire(event).await
    }

    /// Fires a tick event.
    /// 
    /// # Arguments
    /// - `event`: The TickEvent to fire.
    /// 
    /// # Returns
    /// ThreadedEventResult::Continue if all callbacks fired successfully, or else the exit state if it crashed.
    #[inline]
    pub async fn fire_tick(&self, event: TickEvent) -> ThreadedEventResult {
        // Match the kind
        self.tick_handler.fire(event).await
    }



    /// Runs the main loop of the game. Whenever this function returns, the game is expected to stop.
    /// 
    /// # Errors
    /// This function may error if something went wrong during the main loop (which is basically everything).
    pub async fn run(mut self) -> Result<(), Error> {
        // Split self
        let Self { event_loop, mut control_handler, mut window_handler, mut input_handler, mut tick_handler, .. } = self;

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
                    match self.window_handler.fire(WindowEvent::RequestRedraw).await {
                        LocalEventResult::Continue |
                        LocalEventResult::Block    => {}

                        LocalEventResult::Error(err) => { error!("Error while requesting Window redraw: {}", err); }
                        LocalEventResult::Fatal(err) => { error!("Error while requesting Window redraw: {}", err); *control_flow = ControlFlow::Exit; }

                        _ => {}
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
                if let Err(err) = control_handler.fire(ControlEvent::Closing) { error!("Failed to run closing events: {}", err); };

                // Tell all handlers to stop
                control_handler.stop();
                window_handler.stop();
                input_handler.stop();
                tick_handler.stop();
            }
        });
    }



    /// Returns the EventLoop that is stored within the EventSystem (for initializing new Windows and such).
    #[inline]
    pub fn event_loop(&self) -> &EventLoop<()> { &self.event_loop }
}
