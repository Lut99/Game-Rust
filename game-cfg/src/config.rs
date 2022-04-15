/* CONFIG.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:48:52
 * Last edited:
 *   15 Apr 2022, 12:48:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the code that merges the settings file input with the
 *   CLI-overrides.
**/

use std::env;
use std::path::{Path, PathBuf};

use chrono::Local;
use clap::Parser;
use log::LevelFilter;

use crate::errors::ConfigError as Error;
use crate::cli::Arguments;
use crate::file::Settings;


/***** HELPER FUNCTIONS *****/
/// Tries to resolve the given path relative to the executable if it's a relative path.  
/// Returns the path unmodified if it's absolute.
/// 
/// **Generic types**
///  * `P`: The Path-like type of the path we were given.
/// 
/// **Arguments**
///  * `path`: The path to re-relative.
/// 
/// **Returns**  
/// The path as a PathBuf, or a UtilError otherwise.
pub fn reresolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    // Convert path-like to a Path
    let path: &Path = path.as_ref();

    // If the path is not relative, ez pz
    if !path.is_relative() { return Ok(path.to_path_buf()); }

    // Otherwise, get the directory where the executable is
    let exec_path = match env::current_exe() {
        Ok(path) => path,
        Err(err) => { return Err(Error::ExecutablePathError{ err }); }
    };
    let exec_dir = match exec_path.parent() {
        Some(path) => path,
        None       => { return Err(Error::PathParentError{ path: exec_path }); }
    };

    // Add the settings file path to it
    let settings_path = exec_dir.join(path);
    let ssettings_path = match settings_path.to_str() {
        Some(path) => path,
        None       => { return Err(Error::PathToStringError{ path: settings_path }); }
    };

    // Make sure we did not escape the executable directory
    let path = PathBuf::from(path_clean::clean(ssettings_path));
    if !path.starts_with(&exec_dir) {
        return Err(Error::RelativeEscape{ base: exec_dir.to_path_buf(), path });
    }

    // We didn't, so the path makes sense
    Ok(path)
}





/***** LIBRARY *****/
/// Contains the runtime-generated locations of important directories
#[derive(Debug)]
pub struct DirConfig {
    /// The location of the log files
    pub logs : PathBuf,
}

impl DirConfig {
    /// Constructor for the DirConfig, which will generate the locations of directories relative to the executable.
    /// 
    /// # Returns
    /// A new DirConfig instance with generated paths on success, or else an Error.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            logs : reresolve_path(PathBuf::from("./logs"))?,
        })
    }
}



/// Contains the runtime-generated locations of important files
#[derive(Debug)]
pub struct FileConfig {
    /// The location of the settings.json file
    pub settings : PathBuf,
    /// The location of the log file for this session
    pub log      : PathBuf,
}

impl FileConfig {
    /// Constructor for the FileConfig, which will generate the locations of files relative to the executable.
    /// 
    /// # Arguments
    /// The newly generated DirConfig to derive nested paths from.
    /// 
    /// # Returns
    /// A new FileConfig instance with generated paths on success, or else an Error.
    pub fn new(dir_config: &DirConfig) -> Result<Self, Error> {
        // Generate today's time and date
        let now = Local::now().format("%Y-%m-%d_%H-%M-%s.log").to_string();

        // Use that to populate (and return) the struct
        Ok(Self {
            settings : reresolve_path(PathBuf::from("./settings.json"))?,
            log      : dir_config.logs.join(now),
        })
    }
}



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
    pub gpu : usize,
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
        let verbosity = args.verbosity.unwrap_or(settings.verbosity);
        let gpu       = args.gpu.unwrap_or(settings.gpu);

        // Done, return
        Ok(Self {
            dirs  : dir_config,
            files : file_config,
            
            verbosity,

            gpu,
        })
    }
}
