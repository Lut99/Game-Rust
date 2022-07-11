/* CONFIG.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:48:52
 * Last edited:
 *   11 Jul 2022, 19:16:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the code that merges the settings file input with the
 *   CLI-overrides.
**/

use clap::Parser;
use log::LevelFilter;

use crate::errors::ConfigError as Error;
use crate::spec::{DirConfig, FileConfig, Resolution, WindowMode};
use crate::cli::Arguments;
use crate::file::Settings;


/***** LIBRARY *****/
/// The Config struct, which contains the configuration as loaded from both disk and CLI.
#[derive(Debug)]
pub struct Config {
    /// The locations of the various directories. Is generated at runtime to resolve relative to the executable.
    pub dirs  : DirConfig,
    /// The locations of the various files. Is generated at runtime to resolve relative to the executable.
    pub files : FileConfig,

    /// The verbosity of the logging (the CLI-part, at least)
    pub verbosity : LevelFilter,

    /// The gpu to use during rendering
    pub gpu         : usize,
    /// The resolution
    pub resolution  : Resolution,
    /// The window mode
    pub window_mode : WindowMode,
}

impl Config {
    /// Constructor for the Config, that initializes it with configuration from both the CLI and disk.
    /// 
    /// **Returns**  
    /// A new Config on success, or else an Error.
    pub fn new() -> Result<Self, Error> {
        // Generate the default paths
        let dir_config  = DirConfig::new()?;
        let file_config = FileConfig::new(&dir_config)?;

        // Load the CLI
        let args: Arguments = Arguments::parse();
        // Load the settings file
        let settings = match Settings::from_path(&file_config.settings) {
            Ok(settings) => settings,
            Err(err)     => { return Err(Error::SettingsLoadError{ err }); }  
        };

        // Overwrite stuff if necessary
        let verbosity   = args.verbosity.unwrap_or(settings.verbosity);
        let gpu         = args.gpu.unwrap_or(settings.gpu);
        let resolution  = args.resolution.unwrap_or(settings.resolution);
        let window_mode = args.window_mode.unwrap_or(settings.window_mode);

        // Done, return
        Ok(Self {
            dirs  : dir_config,
            files : file_config,
            
            verbosity,

            gpu,
            resolution,
            window_mode,
        })
    }
}
