//  SPEC.rs
//    by Lut99
// 
//  Created:
//    11 Jul 2022, 18:52:17
//  Last edited:
//    06 Aug 2022, 17:46:18
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains enums and structs that are given a value in the config but
// 

use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::Local;
use serde::{Deserialize, Serialize};

pub use crate::errors::{ConfigError, SettingsError};


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
pub fn reresolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConfigError> {
    // Convert path-like to a Path
    let path: &Path = path.as_ref();

    // If the path is not relative, ez pz
    if !path.is_relative() { return Ok(path.to_path_buf()); }

    // Otherwise, get the directory where the executable is
    let exec_path = match env::current_exe() {
        Ok(path) => path,
        Err(err) => { return Err(ConfigError::ExecutablePathError{ err }); }
    };
    let exec_dir = match exec_path.parent() {
        Some(path) => path,
        None       => { return Err(ConfigError::PathParentError{ path: exec_path }); }
    };

    // Add the settings file path to it
    let settings_path = exec_dir.join(path);
    let ssettings_path = match settings_path.to_str() {
        Some(path) => path,
        None       => { return Err(ConfigError::PathToStringError{ path: settings_path }); }
    };

    // Make sure we did not escape the executable directory
    let path = PathBuf::from(path_clean::clean(ssettings_path));
    if !path.starts_with(&exec_dir) {
        return Err(ConfigError::RelativeEscape{ base: exec_dir.to_path_buf(), path });
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
    pub fn new() -> Result<Self, ConfigError> {
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
    pub fn new(dir_config: &DirConfig) -> Result<Self, ConfigError> {
        // Generate today's time and date
        let now = Local::now().format("%Y-%m-%d_%H-%M-%s.log").to_string();

        // Use that to populate (and return) the struct
        Ok(Self {
            settings : reresolve_path(PathBuf::from("./settings.json"))?,
            log      : dir_config.logs.join(now),
        })
    }
}



/// The resolution of the window.
/// 
/// # Contents
/// - `0`: The width of the window.
/// - `1`: The height of the window.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resolution(usize, usize);

impl From<Resolution> for (u32, u32) {
    #[inline]
    fn from(value: Resolution) -> Self {
        Self::from(&value)
    }
}

impl From<&Resolution> for (u32, u32) {
    #[inline]
    fn from(value: &Resolution) -> Self {
        (value.0 as u32, value.1 as u32)
    }
}

impl FromStr for Resolution {
    type Err = SettingsError;


    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Attempt to split
        let pos = match value.find('x') {
            Some(pos) => pos,
            None      => { return Err(SettingsError::MissingX{ raw: value.into() }); }
        };

        // Parse the halves as numbers
        let width: usize = match usize::from_str(&value[..pos]) {
            Ok(width) => width,
            Err(err)  => { return Err(SettingsError::IllegalUnsignedInteger{ raw: value.into(), err }); }
        };
        let height: usize = match usize::from_str(&value[pos + 1..]) {
            Ok(width) => width,
            Err(err)  => { return Err(SettingsError::IllegalUnsignedInteger{ raw: value.into(), err }); }
        };

        // Done
        Ok(Self(width, height))
    }
}
