//  MOD.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:15:11
//  Last edited:
//    31 Jul 2022, 12:21:36
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines various types of pipelines, each of them as a submodule.
// 

pub mod triangle;



/***** FUNCTIONS *****/
/// Registers the components of all the submodules.
pub fn register(ecs: &std::rc::Rc<std::cell::RefCell<game_ecs::Ecs>>) {
    // game_ecs::Ecs::register::<triangle::components::>(ecs);
}
