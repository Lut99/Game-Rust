//  SYSTEM.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:27:38
//  Last edited:
//    07 Aug 2022, 18:56:15
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the EventSystem itself, which manages all events within the
//!   Game.
// 

use std::cell::RefCell;
use std::rc::Rc;

use log::{debug, info, error};
use rust_ecs::Ecs;
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowId;

use game_gfx::RenderSystem;

pub use crate::errors::EventError as Error;
use crate::spec::Event;


/***** LIBRARY *****/
/// Implements the EventSystem.
pub struct EventSystem {
    /// The ECS that the EventSystem may use for Events.
    ecs : Rc<RefCell<Ecs>>,

    /// The EventLoop around which this EventSystem wraps.
    event_loop    : EventLoop<Event>,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The EntityComponentSystem where to register new components.
    /// 
    /// # Returns
    /// A new instance of an EventSystem.
    #[inline]
    pub fn new(ecs: Rc<RefCell<Ecs>>) -> Self {
        // Return a new instance with that ECS, done
        Self {
            ecs,

            event_loop : EventLoop::with_user_event(),
        }
    }



    /// Function that handles the given Event.
    /// 
    /// # Arguments
    /// - `event`: The Event that occurred.
    /// - `render_system`: The RenderSystem that handles draw callbacks.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function errors whenever any of its callbacks error.
    #[inline]
    pub fn handle(event: Event, render_system: &mut RenderSystem) -> Result<(), Error> {
        // Match on the given Event
        match event {
            Event::WindowDraw(id) => Self::handle_window_draw(render_system, id),

            Event::GameLoopComplete => Self::handle_game_loop_complete(render_system),
            Event::Exit(err)        => { Self::handle_exit(err); Ok(()) },
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
    #[inline]
    pub fn handle_window_draw(render_system: &mut RenderSystem, window_id: WindowId) -> Result<(), Error> {
        // Relay to the render system's function
        match render_system.render_window(window_id) {
            Ok(_)    => Ok(()),
            Err(err) => { return Err(Error::RenderError{ id: window_id, err }); }
        }
    }



    /// Function that handles the GameLoopComplete-event.
    /// 
    /// # Returns
    /// Nothing, but does trigger the appropriate callbacks.
    /// 
    /// # Errors
    /// This function errors whenever any of the callbacks error.
    pub fn handle_game_loop_complete(render_system: &RenderSystem) -> Result<(), Error> {
        // Trigger the RenderSystem to trigger redraws in all of its Windows.
        render_system.game_loop_complete();
        Ok(())
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
    pub fn handle_exit(error: Option<Error>) {
        info!("Triggered Exit event");
        if let Some(err) = error.as_ref() { debug!("Exit was triggered due to an error: {}", err); }
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
        // Split self
        let Self{ ecs: _ecs, event_loop } = self;
        let mut render_system = render_system;

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
                            Self::handle_exit(None);

                            // Done
                        },
    
                        // Ignore the others
                        _ => {}
                    }
                },

                WinitEvent::MainEventsCleared => {
                    // Trigger the associated events
                    if let Err(err) = Self::handle_game_loop_complete(&render_system) {
                        // Print it, then quit the game
                        error!("{}", &err);
                        Self::handle_exit(Some(err));
                        *control_flow = ControlFlow::Exit;
                    }
                },

                WinitEvent::RedrawRequested(window_id) => {
                    // Trigger the associated events
                    if let Err(err) = Self::handle_window_draw(&mut render_system, window_id) {
                        // Print it, then quit the game
                        error!("{}", &err);
                        Self::handle_exit(Some(err));
                        *control_flow = ControlFlow::Exit;
                    }
                }

                // Skip the rest (for now)
                _ => {},
            }
        })
    }



    /// Returns the name of the EventSystem, for use in Vulkan's AppInfo.
    #[inline]
    pub fn name() -> &'static str { "Game-Rust EventSystem" }

    /// Returns the version of the EventSystem, for use in Vulkan's AppInfo.
    #[inline]
    pub fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

    /// Returns the internal EventLoop.
    #[inline]
    pub fn event_loop(&self) -> &EventLoop<Event> { &self.event_loop }
}
