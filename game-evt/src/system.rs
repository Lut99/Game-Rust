/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   18 Jul 2022, 18:27:38
 * Last edited:
 *   27 Jul 2022, 14:37:05
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The Event system is in charge of triggering events, which trigger
 *   computations, updates or render passes.
**/

use std::rc::Rc;

use log::error;
use winit::event::{Event as WinitEvent, WindowEvent as WinitWindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use game_ecs::Ecs;

pub use crate::errors::EventError as Error;
use crate::spec::Event;
use crate::components::{DrawCallback, ExitCallback, TickCallback};


/***** LIBRARY *****/
/// Implements the EventSystem.
pub struct EventSystem {
    /// The entity component system around which the EventSystem builds.
    ecs : Rc<Ecs>,
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
    pub fn new(ecs: Rc<Ecs>) -> Result<Rc<Self>, Error> {
        let mut ecs = ecs;

        // Register the components
        Ecs::register::<DrawCallback>(&mut ecs);
        Ecs::register::<TickCallback>(&mut ecs);
        Ecs::register::<ExitCallback>(&mut ecs);

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
            match wevent {
                WinitEvent::WindowEvent{ window_id: _window_id, event } => {
                    // Match the event again
                    match event {
                        WinitWindowEvent::CloseRequested => {
                            // We close the flow in principle
                            *control_flow = ControlFlow::Exit;

                            // Handle close events
                            let mut exit_callbacks = self.ecs.list_component_mut::<ExitCallback>();
                            for c in exit_callbacks.iter_mut() {
                                // The function *might* decide to cancel the quit
                                match (*c.exit_callback)() {
                                    Ok(should_close) => if !should_close { *control_flow = ControlFlow::default(); break; },
                                    Err(err)         => {
                                        error!("{}", err);
                                        *control_flow = ControlFlow::Exit;
                                    }
                                }
                            }

                            // Done
                        },
    
                        // Ignore the others
                        _ => {}
                    }
                },

                WinitEvent::MainEventsCleared => {
                    // // Trigger the 'redraw' winit event
                    // let windows = ecs.list_component_mut::<game_gfx::Window>();
                    // for w in windows {
                    //     // The function *might* decide to cancel the quit
                    //     if let Err(err) = (*c.draw_callback)() {
                    //         error!("{}", err);
                    //         *control_flow = ControlFlow::Exit;
                    //     }
                    // }
                    todo!();
                },

                // Skip the rest (for now)
                _ => {},
            }
        })
    }
}
