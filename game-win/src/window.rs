/* WINDOW.rs
 *   by Lut99
 *
 * Created:
 *   01 Apr 2022, 17:15:38
 * Last edited:
 *   02 Apr 2022, 14:55:35
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Window struct, which represents and (OOP-like) manages
 *   a Window instance.
**/

use ash::{Entry, Instance};
use log::debug;
use winit::event_loop::EventLoop;
use winit::window::{Window as WWindow, WindowBuilder};

use game_evt::{EventType, EventHandler};

pub use crate::errors::WindowError as Error;
use crate::surface::Surface;


/***** WINDOW *****/
/// Manages a single Window, who's lifetime is tied to this object.
pub struct Window {
    /// The title of this Window.
    title : String,

    /// The WinitWindow that we wrap.
    window  : WWindow,
    /// The Vulkan Surface that we create from this Window.
    surface : Surface,
}

impl Window {
    /// Constructor for the Window.
    /// 
    /// This function tries to create a new Window in the given mode. The events for this window will be scheduled on the given event loop.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors whenever the winit OR Vulkan backend does.
    pub fn new<S: Into<String>>(event_loop: &EventLoop<()>, entry: &Entry, instance: &Instance, title: S) -> Result<Self, Error> {
        // Convert the string-like into a string
        let title = title.into();

        // Build the new Winit window
        let wwindow = match WindowBuilder::new()
            .with_title(&title)
            .build(event_loop)
        {
            Ok(wwindow) => wwindow,
            Err(err)    => { return Err(Error::WinitBuildError{ err }); }
        };

        // Build the surface around the window
        let surface = match Surface::new(entry, instance, &wwindow) {
            Ok(surface) => surface,
            Err(err)    => { return Err(Error::SurfaceBuildError{ err }); }
        };

        // Done! Return the window
        debug!("Initialized new window '{}'", &title);
        Ok(Self {
            title,

            window : wwindow,
            surface,
        })
    }



    /// Updates the title in the internal window.
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// ```
    pub fn set_title<S: Into<String>>(&mut self, new_title: S) {
        // Convert the String-like into a String
        let new_title: String = new_title.into();

        // Set the title
        self.window.set_title(&new_title);

        // Update the title internally too
        self.title = new_title;
    }

    

    /// Returns the title of the window.
    #[inline]
    pub fn title(&self) -> &str { &self.title }

    /// Returns the internal window object.
    #[inline]
    pub fn window(&self) -> &WWindow { &self.window }

    /// Returns the internal Vulkan surface object.
    #[inline]
    pub fn surface(&self) -> &Surface { &self.surface }
}

impl EventHandler for Window {
    /// This is a callback for the EventLoop to call when and event is ready to be processed.
    /// 
    /// All types of events are passed. Any unneeded events can safely be ignored.
    /// 
    /// In case the program should stop gracefully, return 'true'. Return 'false' if the program should continue, or any error if it should stop ungracefully.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TODO
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function can error anytime it likes, as long as it returns a valid error type.
    fn handle(&mut self, event: &EventType) -> Result<bool, Box<dyn std::error::Error>> {
        // Switch on the event kind
        match event {
            | EventType::WindowCloseRequested => {
                return Ok(true);
            },

            | EventType::WindowMainEventsCleared => {
                // Request redraw of the internal window
                self.window.request_redraw();
            }

            // Ignore the rest
            _ => {}
        };

        // Done! Keep going
        Ok(false)
    }
}
