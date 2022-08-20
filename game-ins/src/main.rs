//  MAIN.rs
//    by Lut99
// 
//  Created:
//    16 Apr 2022, 13:13:03
//  Last edited:
//    20 Aug 2022, 16:12:53
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the tool that handles installing and/or deinstalling
//!   the
// 

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

use clap::Parser;
use console::style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest::StatusCode;
use reqwest::blocking as req;
use semver::Version;


/***** CONSTANTS *****/
/// The default Game version to install.
const DEFAULT_VERSION: Version = Version::new(0, 1, 0);

// Lazy stuff
lazy_static!{
    /// The location of program files that the user probably wants saved.
    static ref DATA_DIR: PathBuf = dirs::data_local_dir().expect("Could not get data directory; specify it manually using '--data-dir'").join("Game-Rust");

    /// The location of configuration files that the user probably wants saved.
    static ref CONFIG_DIR: PathBuf = dirs::document_dir().expect("Could not get document directory; specify it manually using '--config-dir'").join("Game-Rust");
}





/***** MACROS *****/
/// Sets the global debug state.
macro_rules! set_debug {
    ($value:expr) => {
        unsafe{ PRINT_DEBUG = $value; }
    };
}



/// Prints an error, then quits
macro_rules! fatal {
    ($($arg:tt)+) => {
        {
            println!("{}{}{} {}\n", style("[").bold(), style("ERROR").red().bold(), style("]").bold(), format!($($arg)+));
            std::process::exit(1);
        }
    };
}

/// Prints an error
macro_rules! debug {
    ($($arg:tt)+) => {
        if unsafe{ PRINT_DEBUG } { println!("{}{}{} {}", style("[").bold(), style("DEBUG").cyan().bold(), style("]").bold(), format!($($arg)+)); }
    };
}





/***** GLOBALS *****/
/// Keeps track of the debug state.
static mut PRINT_DEBUG: bool = false;





/***** ARGUMENTS *****/
/// Defines the arguments for the setup tool.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help = "If given, includes additional debug statements detailling what the installer does.")]
    debug : bool,

    #[clap(short, long, help = "The directory where all of the game's system files will be stored. These are typically files that you can re-download or rebuild when moving to a new computer. If omitted, this will be queried during installation.")]
    program_dir : Option<PathBuf>,
    #[clap(short, long, help = "The directory where all of the game's config files will be stored. These are typically files you want to save when moving to a new computer. If omitted, this will be queried during installation.")]
    config_dir  : Option<PathBuf>,

    #[clap(short, long, help = "The Game version to install. If omitted, this will be queried during installation.")]
    version : Option<Version>,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let args: Args = Args::parse();
    set_debug!(args.debug);

    // Print a header thingy
    println!();
    println!("### GAME-RUST INSTALLER v{} ###", env!("CARGO_PKG_VERSION"));
    println!();

    // Let the user choose a version
    let version

    // Start asking questions
    
}
