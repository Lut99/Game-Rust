/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   24 Jul 2022, 15:53:54
 * Last edited:
 *   25 Jul 2022, 23:29:20
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains definitions of components in the ECS.
**/

use std::rc::Rc;
use std::sync::{Arc, RwLock};

use winit::window::Window as WinitWindow;

use game_ecs::spec::Component;
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;


/***** LIBRARY *****/
/// Defines a single Window component.
pub struct Window {
    /// The device that we used to build this Window in.
    pub device : Rc<Device>,

    /// The WinitWindow that we wrap.
    pub window    : WinitWindow,
    /// The Vulkan Surface that we create from this Window.
    pub surface   : Rc<Surface>,
    /// The Vulkan swapchain that we create from this Window.
    pub swapchain : Arc<RwLock<Swapchain>>,
    /// The list of Vulkan swapchain images that we create from this Window.
    pub views     : Vec<Rc<image::View>>,

    /// The title of this Window.
    pub title  : String,
    /// The size of the window (as an extent)
    pub extent : Extent2D<u32>,
}

impl Component for Window {}
