/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jul 2022, 23:21:16
 * Last edited:
 *   25 Jul 2022, 23:24:04
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the ECS components used by the RenderSystem.
**/

use game_ecs::spec::Component;


/***** LIBRARY *****/
/// Defines a renderable Target (i.e., a specific kind of image).
pub struct Target {
    
}

impl Component for Target {}



/// Defines a Window. This lives in the ECS mostly because the event system has to be able to trigger redraw events.
pub struct Window {
    
}

impl Component for Window {}
