/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jul 2022, 23:21:16
 * Last edited:
 *   27 Jul 2022, 13:58:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the ECS components used by the RenderSystem.
**/

use std::rc::Rc;

use winit::window::Window as WinitWindow;

use game_ecs::spec::Component;
use game_vk::auxillary::enums::ImageFormat;
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;


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



/// Defines a Target, which is an entity which may be rendered to.
pub struct Target {
    /// The ImageViews of this RenderTarget.
    pub views  : Vec<Rc<image::View>>,
    /// The format for these views.
    pub format : ImageFormat,
    /// The size (extent) of these views.
    pub extent : Extent2D<u32>,
}

impl Component for Target {}
