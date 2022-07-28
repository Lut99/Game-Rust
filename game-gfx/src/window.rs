/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   27 Jul 2022, 12:54:07
 * Last edited:
 *   28 Jul 2022, 17:05:22
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains functions to manage the Window entity.
**/

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::{Fullscreen, WindowBuilder};

use game_cfg::spec::WindowMode;
use game_ecs::{Ecs, Entity};
use game_vk::auxillary::enums::{ImageAspect, ImageViewKind};
use game_vk::auxillary::structs::Extent2D;
use game_vk::device::Device;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;
use game_vk::image;
use game_vk::sync::Semaphore;

pub use crate::errors::WindowError as Error;
use crate::components;


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(device: &Rc<Device>, swapchain: &mut Rc<Swapchain>) -> Result<Vec<Rc<image::View>>, Error> {
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
            Err(err) => { return Err(Error::ImagesCreateError{ err }); }
        };

        // Store it in the list
        views.push(view);
    }

    // Done, return
    Ok(views)
}





/***** LIBRARY *****/
/// Creates a new entity and generates a Window and Target component for it.
/// 
/// # Generic types
/// - `S`: The String-like type for the title.
/// 
/// # Arguments
/// - `ecs`: The Entity Component System which will store the new entity.
/// - `device`: The Device which will render to the new Window (i.e., will be used to create Vulkan resources).
/// - `event_loop`: The Winit EventLoop which will receive the Window's events.
/// - `title`: The title of the new Window.
/// - `window_mode`: The WindowMode that is used to determine the location, mode and size of the Window.
/// 
/// # Returns
/// The created entity's ID.
/// 
/// # Errors
/// This function may error if we failed to create a new Window, or associated Vulkan resources (such as swapchains etc).
pub fn create<S: Into<String>>(ecs: &Rc<RefCell<Ecs>>, device: Rc<Device>, event_loop: &EventLoop<()>, title: S, window_mode: WindowMode) -> Result<Entity, Error> {
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
                None       => { return Err(Error::UnknownVideoMode{ monitor: monitor_i, resolution, refresh_rate, bit_depth: 32 }); }
            };

            // Put that in the Window
            wwindow = wwindow.with_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
        },
    };

    // Finish building the window
    let wwindow = match wwindow.build(event_loop) {
        Ok(wwindow) => wwindow,
        Err(err)    => { return Err(Error::WinitCreateError{ err }); }
    };

    // Build the surface around the window
    let surface = match Surface::new(device.instance().clone(), &wwindow) {
        Ok(surface) => surface,
        Err(err)    => { return Err(Error::SurfaceCreateError{ err }); }
    };

    // Build the swapchain around the GPU and surface
    let extent = wwindow.inner_size();
    let mut swapchain = match Swapchain::new(device.clone(), surface.clone(), extent.width, extent.height, 3) {
        Ok(swapchain) => swapchain,
        Err(err)      => { return Err(Error::SwapchainCreateError{ err }); }
    };
    let format = swapchain.format();

    // Generate the views
    let views: Vec<Rc<image::View>> = create_views(&device, &mut swapchain)?;

    // Wrap it in a Window and associated Target component and store it in the ECS
    let ecs: Ref<Ecs> = ecs.borrow();
    let window = ecs.add_entity();
    ecs.add_component(window, components::Window {
        device,
        window : wwindow,
        surface,
        swapchain,

        title,
    });
    ecs.add_component(window, components::Target {
        views,
        format,
        extent : Extent2D::new(extent.width, extent.height),
    });

    // Done, return the entity ID
    Ok(window)
}



/// Get the next swapchain image index in the Window.
/// 
/// # Arguments
/// - `window`: The Window component which contains the swapchain to get the next image from.
/// - `done_semaphore`: An optional Semaphore to call when acquiring the next image is done. It is not safe to use the returned index until then.
/// 
/// # Returns
/// The new swapchain index as a number. However, if the swapchain is outdated or invalid, then 'None' is returned, indicating a rebuild is necessary.
/// 
/// # Errors
/// This function may error if we could not get the next image in the swapchain.
#[inline]
pub fn next_image(window: &components::Window, done_semaphore: Option<&Rc<Semaphore>>) -> Result<Option<usize>, Error> {
    // Try to get an image from the swapchain
    match window.swapchain.next_image(done_semaphore, None, None) {
        Ok(index) => Ok(index),
        Err(err)  => Err(Error::SwapchainNextImageError{ err }),
    }
}



/// Presents the current swapchain image to the window.
/// 
/// # Arguments
/// - `window`: The Window component which contains the swapchain to get the next image from.
/// - `index`: The index of the image to present.
/// - `wait_semaphores`: The semaphores to wait for before presenting.
/// 
/// # Returns
/// Upon success, returns whether the swapchain still needs to be recreated or not.
/// 
/// # Errors
/// This function may error if presentation failed.
#[inline]
pub fn present(window: &components::Window, index: usize, wait_semaphores: &[&Rc<Semaphore>]) -> Result<bool, Error> {
    // Call with the swapchain's function
    match window.swapchain.present(index as u32, wait_semaphores) {
        Ok(redo) => Ok(redo),
        Err(err) => Err(Error::SwapchainPresentError{ err }),
    }
}



/// Rebuilds the Window's resources to its new size.
/// 
/// # Arguments
/// - `window`: The Window component which contains the swapchain to get the next image from.
/// 
/// # Errors
/// This function may error if we failed to rebuild any of the resources.
pub fn rebuild(target: &mut components::Target, window: &mut components::Window) -> Result<(), Error> {
    // Get the new size
    let new_size = window.window.inner_size();

    // Rebuild the swapchain
    if let Err(err) = Rc::get_mut(&mut window.swapchain).expect("Could not get muteable Swapchain for rebuild").rebuild(new_size.width, new_size.height) {
        return Err(Error::SwapchainRebuildError{ err });
    }

    // Generate new views again
    target.views = create_views(&window.device, &mut window.swapchain)?;

    // Update the extent in the target
    target.extent = Extent2D::new(new_size.width, new_size.height);

    // Done
    Ok(())
}
