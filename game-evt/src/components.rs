/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   18 Jul 2022, 18:25:39
 * Last edited:
 *   28 Jul 2022, 17:58:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the components in the ECS for the event system. This thus
 *   mostly encompasses (general) events.
**/

use std::error::Error;

use game_ecs::{Component, Entity};

use crate::spec::Event;


/***** LIBRARY *****/
/// Defines a Draw callback, which is called whenever the window needs redrawing.
pub struct DrawCallback {
    /// The Entity ID of this callback.
    pub this : Entity,

    /// The callback to call when a target needs to be redrawn.
    /// 
    /// # Arguments
    /// - `event`: The Event type that was called (is always `Event::Draw` for this callback). It contains the target to which should be drawn.
    /// - `this`: The ID of the entity for which the callback was called.
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub draw_callback: Box<dyn FnMut(Event, Entity) -> Result<(), Box<dyn Error>>>,
}

impl Component for DrawCallback {}



/// Defines a Tick callback, which means that the given closure will be fired when a game tick happens.
pub struct TickCallback {
    /// The Entity ID of this callback.
    pub this : Entity,

    /// The callback to call when a game tick has occurred.
    /// 
    /// # Arguments
    /// - `event`: The Event type that was called (is always `Event::Tick` for this callback).
    /// - `this`: The ID of the entity for which the callback was called.
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub tick_callback: Box<dyn FnMut(Event, Entity) -> Result<(), Box<dyn Error>>>,
}

impl Component for TickCallback {}

/// Defines a GameLoopComplete callback, which is called when the main events in the loop have been cleared. It basically signals the end of a game loop iteration.
pub struct GameLoopCompleteCallback {
    /// The Entity ID of this callback.
    pub this : Entity,

    /// The callback to call when a game loop has been completed.
    /// 
    /// # Arguments
    /// - `event`: The Event type that was called (is always `Event::GameLoopComplete` for this callback).
    /// - `this`: The ID of the entity for which the callback was called.
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub loop_complete_callback: Box<dyn FnMut(Event, Entity) -> Result<(), Box<dyn Error>>>,
}

impl Component for GameLoopCompleteCallback {}



/// The ExitCallback component is used to mark entities that need to handle stuff on program exit.
pub struct ExitCallback {
    /// The Entity ID of this callback.
    pub this : Entity,

    /// The callback to call when the game is closing down.
    /// 
    /// # Arguments
    /// - `event`: The Event type that was called (is always `Event::Exit` for this callback).
    /// - `this`: The ID of the entity for which the callback was called.
    /// 
    /// # Returns
    /// Whether or not the exiting should continue (true) or not (false).
    /// 
    /// # Errors
    /// The callback may actually error what and whenever it likes.
    pub exit_callback: Box<dyn FnMut(Event, Entity) -> Result<bool, Box<dyn Error>>>,
}

impl Component for ExitCallback {}
