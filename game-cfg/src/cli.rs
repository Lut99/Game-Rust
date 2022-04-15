/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:55:40
 * Last edited:
 *   15 Apr 2022, 12:48:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the part of the config that parses the command-line
 *   interface.
**/

use clap::Parser;
use log::LevelFilter;


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
    pub(crate) gpu : Option<usize>,
}
