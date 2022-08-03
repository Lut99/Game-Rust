//  SYSTEM.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:27:38
//  Last edited:
//    03 Aug 2022, 18:37:26
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the EventSystem itself, which manages all events within the
//!   Game.
// 

use std::cell::RefCell;
use std::rc::Rc;

use log::error;
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

use game_ecs::Ecs;
use game_gfx::RenderSystem;

pub use crate::errors::EventError as Error;
use crate::spec::Event;


/***** LIBRARY *****/
/// Implements the EventSystem.
pub struct EventSystem {
    /// The ECS that the EventSystem may use for Events.
    ecs : Rc<Ecs>,

    /// The EventLoop around which this EventSystem wraps.
    event_loop    : EventLoop<Event>,
    /// The RenderSystem that will process any render-related events.
    render_system : RenderSystem,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The EntityComponentSystem where to register new components.
    /// - `render_system': The RenderSystem that will process any render-related events.
    /// 
    /// # Returns
    /// A new instance of an EventSystem.
    #[inline]
    pub fn new(ecs: Rc<RefCell<Ecs>>, render_system: RenderSystem) -> Self {
        // Return a new instance with that ECS, done
        Ok(Self {
            ecs,

            event_loop : EventLoop::with_user_defined(),
            render_system,
        })
    }



    /// Function that handles the given Event.
    /// 
    /// # Arguments
    /// - `event`: The Event that occurred.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function errors whenever any of its callbacks error.
    #[inline]
    pub fn handle(&self, event: Event) -> Result<(), Error> {
        // Match on the given Event
        match event {
            
        }
    }



    /// Function that handles the redraw-event for a particular Window.
    /// 
    /// # Arguments
    /// - `window_id`: The ID of the window for which the event was fired.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function errors whenever any of the RenderSystem's callbacks error.
    /// 
    /// # Panics
    /// This function panics if the window ID is not known to the RenderSystem.
    pub fn handle_window_draw(window_id: WindowId) -> Result<(), Error> {
        
    }



    /// Function that handles the GameLoopComplete-event.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function errors whenever any of the callbacks error.
    pub fn handle_game_loop_complete(&self) -> Result<(), Error> {
        // Trigger the RenderSystem to trigger redraws in all of its Windows.
        
    }

    /// Function that handles the Exit-event.
    /// 
    /// # Arguments
    /// - `error`: If this Event was fired due to an Error, this should be `Some()` with that Error.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function does not explicitly return errors. Instead, it logs them (using `error!()`), and fires the remaining close events as if the exit was called with an Error (overwriting any Error already set).
    pub fn handle_exit(&self, error: Option<Error>) {
        info!("Triggered Exit event");
        if let Some(err) = error.as_ref() { debug!("Exit was triggered due to an error: {}", err); }

        // Nothing to callback yet
    }



    /// Initiates the EventSystem's loop, taking over the EventLoop of winit (for rendering).
    /// 
    /// # Arguments
    /// - `render_system`: The RenderSystem that processes any render-related events.
    /// 
    /// # Returns
    /// This function never returns, effectively 'hijacking' the current thread.
    /// 
    /// # Errors
    /// Any error that occurs is printed to stderr using `log`'s `error!()` macro.
    pub fn game_loop(self, render_system: RenderSystem) -> ! {
        // Start the EventLoop
        event_loop.run(move |wevent, _, control_flow| {
            // Switch on the Event that happened
            match wevent {
                WinitEvent::WindowEvent{ window_id: _window_id, event } => {
                    // Match the event again
                    match event {
                        WinitWindowEvent::CloseRequested => {
                            // We close the flow in principle
                            *control_flow = ControlFlow::Exit;

                            // Fire close events (it acts as a sink for errors)
                            self.handle_exit(None);

                            // Done
                        },
    
                        // Ignore the others
                        _ => {}
                    }
                },

                WinitEvent::MainEventsCleared => {
                    // Trigger the associated events
                    if let Err(err) = self.handle_game_loop_complete() {
                        // Print it, then quit the game
                        error!("{}", &err);
                        self.handle_exit(Some(err));
                        *control_flow = ControlFlow::Exit;
                    }
                },

                WinitEvent::RedrawRequested(window_id) => {
                    // Trigger the associated events
                    if let Err(err) = self.handle_window_draw(window_id) {
                        // Print it, then quit the game
                        error!("{}", &err);
                        self.handle_exit(Some(err));
                        *control_flow = ControlFlow::Exit;
                    }
                }

                // Skip the rest (for now)
                _ => {},
            }
        })
    }
}
