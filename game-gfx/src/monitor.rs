/* MONITOR.rs
 *   by Lut99
 *
 * Created:
 *   12 Jul 2022, 17:50:19
 * Last edited:
 *   12 Jul 2022, 18:09:17
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains code for interacting with various monitors.
**/

use winit::event_loop::EventLoop;
use winit::monitor::VideoMode as WinitVideoMode;

use game_vk::auxillary::structs::{Extent2D, Offset2D};


/***** LIBRARY *****/
/// The VideoMode represents a set of properties about the (exclusive) video modes of a monitor.
pub struct VideoMode {
    /// The resolution of the monitor in this mode (in pixels).
    pub resolution : Extent2D<u32>,
    /// The resolution of the monitor in this mode (in )
}



/// Represents a single Monitor, which may be used to create new Windows.
pub struct Monitor {
    /// Some name of the monitor as it's known in the system.
    pub name : String,

    /// The monitor's position (of its top-left corner) in the virtual screen area
    pub position   : Offset2D<u32>,
    /// The monitor's resolution
    pub resolution : Extent2D<u32>,
    /// The scale of the monitor (i.e., the mapping of pixels to screen coordinates)
    pub scaling    : f64,

    /// The video modes supported by this monitor.
    /// 
    /// # Layout
    /// - `0`: The width of the monitor in this mode (in pixels).
    /// - `1`: The height of the monitor in this mode (in pixels).
    /// - `2`: The refresh rate of the monitor in this mode.
    /// - `3`: The bit colour depth for the monitor in this mode.
    pub video_modes : Vec<(usize, usize, usize, usize)>,
}

impl Monitor {
    /// Factory method that creates a Monitor struct per monitor known to winit.
    /// 
    /// # Returns
    /// A Vec of Monitor instances, one per attached monitor. If no monitors are found, then the vector is simply empty.
    #[inline]
    pub fn get_monitors(event_loop: &EventLoop<()>) -> Vec<Self> {
        // Get the monitors
        event_loop.available_monitors().map(|m| {
            Self {
                name : m.name().unwrap_or(String::from("<unnamed monitor>")),
            }
        }).collect()
    }
}
