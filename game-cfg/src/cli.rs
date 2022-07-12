/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:55:40
 * Last edited:
 *   12 Jul 2022, 18:44:46
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
    pub(crate) gpu          : Option<usize>,
    /// The monitor where the window will be spawned.
    #[clap(short, long, default_value = "-1", help = "The monitor where the window will be placed (as an index). If '-1', then uses the current monitor. Not relevant in 'windowed' window mode. See the 'game-list' executable to discover the options.")]
    pub(crate) monitor      : i64,
    /// The resolution of the window.
    #[clap(short, long, default_value = "800x600", help = "The resolution of the window. Should be in the form of '<width>x<height>', where '<width>' and '<height>' are unsigned integers. Not relevant in 'windowed_fullscreen' window mode. See the 'game-list' executable to discover the options.")]
    pub(crate) resolution   : Resolution,
    /// The refresh rate of the window.
    #[clap(short, long, default_value = "30", help = "The refresh rate of the window, in Hz. Only relevant in 'fullscreen' window mode. See the 'game-list' executable to discover the options.")]
    pub(crate) refresh_rate : u16,
    /// The refresh rate of the window.
    #[clap(short, long, help = "The bit depth to render in (in bits-per-pixel). Only relevant in 'fullscreen' window mode. See the 'game-list' executable to discover the options.")]
    pub(crate) bit_depth    : Option<usize>,
    /// The window mode to open the window in.
    #[clap(short, long, help = "The window mode for the window. Can be 'windowed', 'windowed_fullscreen' or 'fullscreen'.")]
    pub(crate) window_mode  : Option<WindowMode>,
}
