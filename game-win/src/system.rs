/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   24 Jul 2022, 15:51:36
 * Last edited:
 *   24 Jul 2022, 16:08:40
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The WindowSystem itself, which does most of the heavy lifting.
**/

use std::rc::Rc;
use std::sync::{Arc, RwLock};

use log::debug;
use winit::dpi::{PhysicalSize, Size};
use winit::event_loop::EventLoop;
use winit::monitor::{MonitorHandle, VideoMode};
use winit::window::{Fullscreen, WindowBuilder};

use game_cfg::spec::WindowMode;
use game_ecs::{Ecs, Entity};
use game_vk::auxillary::enums::{ImageAspect, ImageViewKind};
use game_vk::device::Device;
use game_vk::image;
use game_vk::surface::Surface;
use game_vk::swapchain::Swapchain;

pub use crate::errors::WindowError as Error;
use crate::components::Window;


/***** HELPER FUNCTIONS *****/
/// Given a Swapchain, generates new ImageViews around its images.
/// 
/// # Arguments
/// - `device`: The Device where the Swapchain lives.
/// - `swapchain`: The Swapchain to create ImageViews for.
/// 
/// # Errors
/// This function errors if we could not create the new views.
fn create_views(device: &Rc<Device>, swapchain: &Arc<RwLock<Swapchain>>) -> Result<Vec<Rc<image::View>>, Error> {
    // Get a read lock for the rest
    let sc = swapchain.read().expect("Could not get read lock on Swapchain");

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
/// The WindowSystem manages windows, which are stored in the ECS.
pub struct WindowSystem {
    
}

impl WindowSystem {
    /// Creates a new WindowSystem.
    /// 
    /// # Arguments
    /// - `ecs`: The ECS to register components to.
    /// 
    /// # Returns
    /// A new WindowSystem.
    pub fn new(ecs: &mut Ecs) -> Self {
        // Register new components
        ecs.register::<Window>();

        // Return ourselves
        Self {}
    }



    /// Creates a new Window in the given ECS with the given properties.
    /// 
    /// # Generic types
    /// - `S`: The String-like type of the title.
    /// 
    /// # Arguments
    /// - `ecs`: The Entity Component System where the Window will live.
    /// - `event_loop`: The EventLoop where the events of the new Window will be processed on.
    /// - `device`: The Device that will render to the given Window.
    /// - `title`: The title of the Window (as a String-like).
    /// 
    /// # Returns
    /// The Entity ID of the new Window.
    /// 
    /// # Errors
    /// This function typically errors if we failed to create a new Window.
    pub fn create<S: AsRef<str>>(&self, ecs: &mut Ecs, event_loop: &EventLoop<()>, device: Rc<Device>, title: S, window_mode: WindowMode) -> Result<Entity, Error> {
        // Convert str-like to str
        let title: &str = title.as_ref();

        // Start building the new window
        let mut wwindow = WindowBuilder::new()
            .with_title(title);

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
        let swapchain = match Swapchain::new(device.clone(), surface.clone(), extent.width, extent.height, 3) {
            Ok(swapchain) => swapchain,
            Err(err)      => { return Err(Error::SwapchainCreateError{ err }); }
        };

        // Generate the views
        let views: Vec<Rc<image::View>> = create_views(&device, &swapchain)?;

        // Done! Return the window
        debug!("Initialized new window '{}'", title);
        let window = ecs.add_entity();
        ecs.add_component(window, Window {
            
        });
        Ok(window)
    }
}