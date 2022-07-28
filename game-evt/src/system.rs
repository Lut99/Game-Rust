/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   18 Jul 2022, 18:27:38
 * Last edited:
 *   28 Jul 2022, 17:59:56
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The Event system is in charge of triggering events, which trigger
 *   computations, updates or render passes.
**/

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use log::error;
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use game_ecs::{Ecs, Entity};
use game_ecs::list::ComponentListBase;

pub use crate::errors::EventError as Error;
use crate::spec::Event;
use crate::components::{DrawCallback, ExitCallback, GameLoopCompleteCallback, TickCallback};


/***** LIBRARY *****/
/// Implements the EventSystem.
pub struct EventSystem {
    /// The entity component system around which the EventSystem builds.
    ecs : Rc<RefCell<Ecs>>,
}

impl EventSystem {
    /// Constructor for the EventSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The EntityComponentSystem where to register new components.
    /// 
    /// # Returns
    /// A new instance of an EventSystem, already wrapped in a reference-counting pointer.
    /// 
    /// # Errors
    /// This function only errors if we failed to register new components.
    pub fn new(ecs: Rc<RefCell<Ecs>>) -> Result<Rc<Self>, Error> {
        // Register the components
        Ecs::register::<DrawCallback>(&ecs);
        Ecs::register::<TickCallback>(&ecs);
        Ecs::register::<ExitCallback>(&ecs);

        // Return a new instance, done
        Ok(Rc::new(Self {
            ecs,
        }))
    }



    /// Initiates the EventSystem's loop, taking over the EventLoop of winit (for rendering).
    /// 
    /// # Arguments
    /// - `event_loop`: The winit EventLoop that is used to handle Window events.
    /// 
    /// # Returns
    /// This function never returns, effectively 'hijacking' the current thread.
    /// 
    /// # Errors
    /// Any error that occurs is printed to stderr using `log`'s `error!()` macro.
    pub fn game_loop(self, event_loop: EventLoop<Event>) -> ! {
        // Start the EventLoop
        event_loop.run(move |wevent, _, control_flow| {
            // Switch on the Event that happened
            let mut exit: Option<Event> = None;
            match wevent {
                WinitEvent::WindowEvent{ window_id: _window_id, event } => {
                    // Match the event again
                    match event {
                        WinitWindowEvent::CloseRequested => {
                            // We tell it that we'd like to stop
                            exit = Some(Event::Exit(None));
                        },
    
                        // Ignore the others
                        _ => {}
                    }
                },

                WinitEvent::MainEventsCleared => {
                    // Trigger the 'MainEventsCleared' event
                    let ecs: Ref<Ecs> = self.ecs.borrow();
                    let mut callbacks = ecs.list_component_mut::<GameLoopCompleteCallback>();
                    for c in callbacks.iter_mut() {
                        // Perform the call
                        if let Err(err) = (*c.loop_complete_callback)(Event::GameLoopComplete, c.this) {
                            // Make sure the error loop begins
                            exit = Some(Event::Exit(Some(err)));
                            break;
                        }
                    }
                },

                WinitEvent::RedrawRequested(window) => {
                    // Trigger the 'Draw' event for this target
                    let ecs: Ref<Ecs> = self.ecs.borrow();
                    let mut callbacks = ecs.list_component_mut::<DrawCallback>();
                    for c in callbacks.iter_mut() {
                        // Perform the call
                        if let Err(err) = (*c.draw_callback)(Event::Draw, c.this) {
                            // Make sure the error loop begins
                            exit = Some(Event::Exit(Some(err)));
                            break;
                        }
                    }
                }

                // Skip the rest (for now)
                _ => {},
            }

            // If the ControlFlow is exiting, call the callbacks
            if let Some(Event::Exit(mut error)) = exit {
                // Get the exit callbacks
                let ecs: Ref<Ecs> = self.ecs.borrow();
                let mut exit_callbacks = ecs.list_component_mut::<ExitCallback>();

                // If Exit is an Error, print that first
                if let Some(err) = error.as_ref() {
                    error!("{}", err);
                }

                // Iterate over them to call them
                *control_flow = ControlFlow::Exit;
                for c in exit_callbacks.iter_mut() {
                    // The function *might* decide to cancel the quit
                    match (*c.exit_callback)(Event::Exit(None), c.this) {
                        // If told to stop quitting, then see if we need to stop
                        Ok(should_close) => if !should_close {
                            // Only stop if no error occurred (otherwise, we still quit but forego to call the other callbacks)
                            if let None = error { *control_flow = ControlFlow::default() }
                            break;
                        },
                        Err(err) => {
                            // The exit function failed; from now on, go on with this error
                            error!("{}", err);
                            error = Some(err);
                        }
                    }
                }
            }

            // The result of the exit callbacks now determine control_flow
        })
    }
}
