/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:31:02
 * Last edited:
 *   26 Mar 2022, 10:52:28
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint for the ECS package of the game. This package implements
 *   the base Entity Component System and related interfaces.
**/

/// The module that contains common specifications.
pub mod spec;
/// The module for the the component lists.
pub mod list;
/// The module for the base system itself.
pub mod system;

// Bring some components into the general package namespace
pub use spec::Entity;
pub use list::ComponentList;
pub use system::Ecs;


/***** MACROS *****/
/// Downcasts a generic ComponentListBase to a ComponentList<T>
#[macro_export]
macro_rules! to_component_list {
    ($list:expr,$ctype:tt) => {
        {
            let id = $list.id();
            $list.as_any().downcast_ref::<ComponentList<$ctype>>().expect(&format!("Could not downcast ComponentList<{:?}> to ComponentList<{:?}>", id, ComponentList::<$ctype>::id()))
        }
    };
}

/// Downcasts a generic ComponentListBase to a ComponentList<T>
#[macro_export]
macro_rules! to_component_list_mut {
    ($list:expr,$ctype:tt) => {
        {
            let id = $list.id();
            $list.as_any_mut().downcast_mut::<ComponentList<$ctype>>().expect(&format!("Could not downcast ComponentList<{:?}> to ComponentList<{:?}>", id, ComponentList::<$ctype>::id()))
        }
    };
}
