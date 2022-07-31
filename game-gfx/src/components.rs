//  COMPONENTS.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 18:11:44
//  Last edited:
//    31 Jul 2022, 12:11:12
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the various ECS components that the RenderSystem uses /
// 

use std::rc::Rc;

use winit::window::Window as WinitWindow;

use game_ecs::Component;
use game_vk::auxillary::enums::ImageFormat;
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::swapchain::Swapchain;
use game_vk::surface::Surface;
use game_vk::image;


/***** LIBRARY *****/
/// Defines a Window, which may be rendered to (i.e., an entity with the Window entity is guaranteed to implement the RenderTarget entity).
pub struct Window {
    /// The Device that will render to this Window.
    pub device : Rc<Device>,

    /// The winit Window itself.
    pub window    : WinitWindow,
    /// The surface of the window to render to.
    pub surface   : Rc<Surface>,
    /// The swapchain of the window to render to.
    pub swapchain : Rc<Swapchain>,

    /// The title of the window.
    pub title  : String,
}

impl Component for Window {}



/// Defines some renderable Target.
pub struct RenderTarget {
    /// The list of image views that wrap the swapchain images to render to.
    pub views     : Vec<Rc<image::View>>,

    /// The format of the window.
    pub format : ImageFormat,
    /// The size of the window.
    pub extent : Extent2D<u32>,
}

impl Component for RenderTarget {}



/// Defines a general RenderPipeline component that may be used by the RenderSystem to render a pipeline.
pub struct RenderPipeline {
    
}

impl Component for RenderPipeline {}
