/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   30 Jul 2022, 18:11:44
 * Last edited:
 *   30 Jul 2022, 18:13:02
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the various ECS components that the RenderSystem uses /
 *   exports.
**/

use game_ecs::Component;


/***** LIBRARY *****/
/// Defines a Window, which may be rendered to (i.e., an entity with the Window entity is guaranteed to implement the RenderTarget entity).
pub struct Window {
    
}

impl Component for Window {}



/// Defines some renderable Target.
pub struct RenderTarget {
    
}

impl Component for RenderTarget {}
