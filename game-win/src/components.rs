/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   29 Jul 2022, 12:44:21
 * Last edited:
 *   29 Jul 2022, 12:45:40
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the ECS component specifications for this crate.
**/

use std::rc::Rc;

use winit::window::Window as WinitWindow;

use game_ecs::Component;
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;


/***** LIBRARY *****/
/// Defines a Window, which contains Window-only specific functions and stuff, like Swapchains.
pub struct Window {
    /// The Device that will render to this Window.
    pub device    : Rc<Device>,
    /// The actual winit Window we wrap.
    pub window    : WinitWindow,
    /// The Surface where we will render to.
    pub surface   : Rc<Surface>,
    /// The Swapchain which generates images and presents to the Window.
    pub swapchain : Rc<Swapchain>,

    /// The title of this Window.
    pub title : String,
}

impl Component for Window {}
