/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:55:40
 * Last edited:
 *   11 Jul 2022, 19:15:57
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the part of the config that parses the command-line
 *   interface.
**/

use clap::Parser;
use log::LevelFilter;

use crate::spec::{Resolution, WindowMode};


/***** ARGUMENT STRUCTS *****/
/// Defines the command-line part of the Config struct.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    /// The verbosity of the logging (the CLI-part, at least)
    #[clap(short, long, help = "The verbosity of the terminal logging.")]
    pub(crate) verbosity : Option<LevelFilter>,

    /// If given, overrides the GPU to use
    #[clap(short, long, help = "The GPU to use during the rendering process.")]
    pub(crate) gpu         : Option<usize>,
    /// The resolution of the window.
    #[clap(short, long, help = "The resolution of the window. Should be in the form of '<width>x<height>', where '<width>' and '<height>' are unsigned integers.")]
    pub(crate) resolution  : Option<Resolution>,
    /// The window mode to open the window in.
    #[clap(short, long, help = "The window mode for the window. Can be 'windowed', 'windowed_fullscreen' or 'fullscreen'.")]
    pub(crate) window_mode : Option<WindowMode>,
}
