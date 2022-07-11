/* FILE.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:04:45
 * Last edited:
 *   11 Jul 2022, 19:12:49
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the file-side of loading the game's configuration.
**/

use std::fs::File;
use std::path::Path;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

pub use crate::errors::SettingsError as Error;
use crate::spec::{Resolution, WindowMode};


/***** SETTINGS STRUCT *****/
/// Defines the settings to load, and how to load them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// The debug-level
    pub verbosity : LevelFilter,

    /// The GPU to use
    pub gpu         : usize,
    /// The resolution of the Window.
    pub resolution  : Resolution,
    /// The WindowMode for the window.
    pub window_mode : WindowMode,
}

impl Settings {
    /// Tries to load the Settings file from disk. If no such file is found, auto-generates it with the default settings.
    /// 
    /// **Generic types**
    ///  * `P`: The Path-like type of the settings.json file path.
    /// 
    /// **Arguments**
    ///  * `path`: The Path to the settings.json file.
    /// 
    /// **Returns**
    /// A new Settings instance on success, or an Error on failure.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Convert the Path-like to a Path.
        let path = path.as_ref();

        // Try to open the path
        let handle = match File::open(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::OpenError{ path: path.to_path_buf(), err }); }
        };

        // Try to parse with serde
        let settings: Settings = match serde_json::from_reader(handle) {
            Ok(settings) => settings,
            Err(err)     => { return Err(Error::ParseError{ path: path.to_path_buf(), err }); }
        };

        // Success! We're done here
        Ok(settings)
    }



    /// Writes this Settings file to the given path.
    /// 
    /// **Generic types**
    ///  * `P`: The Path-like type of the settings.json file path.
    /// 
    /// **Arguments**
    ///  * `path`: The Path to write the settings.json file file.
    /// 
    /// **Returns**
    /// Nothing on success, or an Error on failure.
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        // Convert the Path-like to a Path.
        let path = path.as_ref();

        // Open a handle to the file location
        let handle = match File::create(path) {
            Ok(handle) => handle,
            Err(err)   => { return Err(Error::OpenError{ path: path.to_path_buf(), err }); }
        };

        // Use serde to write
        match serde_json::to_writer_pretty(handle, self) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::WriteError{ path: path.to_path_buf(), err }),
        }
    }
}
