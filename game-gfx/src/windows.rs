//  WINDOWS.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 11:40:30
//  Last edited:
//    31 Jul 2022, 12:09:22
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines functions that manage Window components within the ECS.
// 

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::{Fullscreen, WindowBuilder};

use game_cfg::spec::WindowMode;
use game_ecs::{Ecs, Entity};
use game_spc::spec::Event;
use game_vk::auxillary::enums::{ImageAspect, ImageViewKind};
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::swapchain::Swapchain;
use game_vk::surface::Surface;
use game_vk::image;

pub use crate::errors::WindowError as Error;
use crate::components::{RenderTarget, Window};


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// - `title`: The title of the window for which we create views. Only used for debugging purposes.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(device: &Rc<Device>, swapchain: &mut Rc<Swapchain>, title: &str) -> Result<Vec<Rc<image::View>>, Error> {
    // Get a muteable reference to the Swapchain
    let sc: &mut Swapchain = Rc::get_mut(swapchain).expect("Could not get muteable Swapchain");

    // Rebuild all of the image views
    debug!("Generating image views...");
    let mut views: Vec<Rc<image::View>> = Vec::with_capacity(sc.images().len());
    for swapchain_image in sc.images() {
        // Create the view around it
        let view = match image::View::new(device.clone(), swapchain_image.clone(), image::ViewInfo {
            kind    : ImageViewKind::TwoD,
            format  : sc.format().into(),
            swizzle : Default::default(),

            aspect     : ImageAspect::Colour,
            base_level : 0,
            mip_levels : 1,
        }) {
            Ok(view) => view,
            Err(err) => { return Err(Error::ViewsCreateError{ title: title.into(), err }); }
        };

        // Store it in the list
        views.push(view);
    }

    // Done, return
    Ok(views)
}





/***** LIBRARY *****/
/// Creates a new entity and initializes a Window component for it with the given properties.
/// 
/// # Generic types
/// - `S`: The String-like type of the `title`.
/// 
/// # Arguments
/// - `ecs`: The Entity Component System where we should create a new Window.
/// - `event_loop`: The EventLoop where the Window will register its events to.
/// - `title`: The title of the new Window.
/// - `window_mode`: The WindowMode of the new Window. This determines its monitor, location and size.
/// 
/// # Returns
/// The Entity ID of the new entity.
/// 
/// # Errors
/// This function may fail if we could not create a new Window or associated Vulkan structs.
pub fn create<S: Into<String>>(ecs: &Rc<RefCell<Ecs>>, event_loop: &EventLoop<Event>, device: Rc<Device>, title: S, window_mode: WindowMode) -> Result<Entity, Error> {
    // Convert String-like into String
    let title: String = title.into();

    // Start building the new window
    let mut wwindow = WindowBuilder::new()
        .with_title(title.clone());

    // Resolve the WindowMode and set the appropriate properties in the window.
    match window_mode {
        WindowMode::Windowed{ resolution } => {
            wwindow = wwindow.with_inner_size(Size::Physical(PhysicalSize{ width: resolution.0, height: resolution.1 }));
        },
        WindowMode::WindowedFullscreen{ monitor } => {
            // Attempt to find the target monitor & its resolution
            let monitor: MonitorHandle = match event_loop.available_monitors().nth(monitor) {
                Some(handle) => handle,
                None         => { return Err(Error::UnknownMonitor{ got: monitor, expected: event_loop.available_monitors().count() }); }
            };
            let resolution: PhysicalSize<u32> = monitor.size();

            // Pass that to the window
            wwindow = wwindow.with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
            wwindow = wwindow.with_inner_size(resolution);
        },
        WindowMode::Fullscreen{ monitor, resolution, refresh_rate } => {
            // Attempt to find the target monitor
            let monitor_i = monitor;
            let monitor: MonitorHandle = if monitor < usize::MAX {
                match event_loop.available_monitors().nth(monitor) {
                    Some(handle) => handle,
                    None         => { return Err(Error::UnknownMonitor{ got: monitor, expected: event_loop.available_monitors().count() }); }
                }
            } else {
                match event_loop.available_monitors().next() {
                    Some(handle) => handle,
                    None         => { return Err(Error::NoMonitors); }
                }
            };

            // Now find a videomode with matching stats
            let mut video_mode: Option<VideoMode> = None;
            for mode in monitor.video_modes() {
                if resolution.0 == mode.size().width && resolution.1 == mode.size().height && refresh_rate == mode.refresh_rate() && mode.bit_depth() == 32 {
                    video_mode = Some(mode);
                    break;
                }
            }
            let video_mode = match video_mode {
                Some(mode) => mode,
                None       => { return Err(Error::UnsupportedVideoMode{ monitor: monitor_i, resolution, refresh_rate, bit_depth: 32 }); }
            };

            // Put that in the Window
            wwindow = wwindow.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
        },
    };

    // Finish building the window
    let wwindow = match wwindow.build(event_loop) {
        Ok(wwindow) => wwindow,
        Err(err)    => { return Err(Error::WinitCreateError{ title, err }); }
    };

    // Build the surface around the window
    let surface = match Surface::new(device.instance().clone(), &wwindow) {
        Ok(surface) => surface,
        Err(err)    => { return Err(Error::SurfaceCreateError{ title, err }); }
    };

    // Build the swapchain around the GPU and surface
    let extent = wwindow.inner_size();
    let mut swapchain = match Swapchain::new(device.clone(), surface.clone(), extent.width, extent.height, 3) {
        Ok(swapchain) => swapchain,
        Err(err)      => { return Err(Error::SwapchainCreateError{ title, err }); }
    };
    let format = swapchain.format();

    // Generate the views
    let views: Vec<Rc<image::View>> = create_views(&device, &mut swapchain, &title)?;

    // Wrap it in a Window and associated Target component and store it in the ECS
    let ecs: Ref<Ecs> = ecs.borrow();
    let window = ecs.add_entity();
    ecs.add_component(window, Window {
        device,

        window : wwindow,
        surface,
        swapchain,

        title,
    });
    ecs.add_component(window, RenderTarget {
        views,

        format,
        extent : Extent2D::new(extent.width, extent.height),
    });

    // Done, return the entity ID
    Ok(window)
}
