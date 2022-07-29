/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   29 Jul 2022, 12:46:37
 * Last edited:
 *   29 Jul 2022, 13:03:46
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the library that contains crate-surpassing definitions
 *   and ECS components.
**/

/// Contains general ECS components.
pub mod components;



/// Registers the global components in the crate.
/// 
/// # Arguments
/// - `ecs`: The Entity Component System to register the new components to.
/// 
/// # Returns
/// Nothing, but does the registration in the Ecs.
pub fn register_components(ecs: &std::rc::Rc<std::cell::RefCell<game_ecs::Ecs>>) {
    // Register 'em
    game_ecs::Ecs::register::<components::Target>(ecs);
}
