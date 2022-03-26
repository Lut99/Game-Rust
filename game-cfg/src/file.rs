/* FILE.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:04:45
 * Last edited:
 *   26 Mar 2022, 12:09:43
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the file-side of loading the game's configuration.
**/

use std::fmt::{Formatter, Result as FResult};
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};

pub use crate::errors::SettingsError as Error;


/***** CONSTANTS *****/
/// The standard logging path
pub const DEFAULT_LOG_PATH: &str = "./logs/%TODAY%_%NOW%.log";

/// The standard log level
pub const DEFAULT_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Error;





/***** HELPER STRUCTS *****/
/// Serde visitor for the LevelFilter.
struct LevelFilterVisitor;

impl<'de> Visitor<'de> for LevelFilterVisitor {
    type Value = LevelFilter;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FResult {
        write!(formatter, "'TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR' or 'OFF'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        match log::LevelFilter::from_str(value) {
            Ok(value) => Ok(LevelFilter(value)),
            Err(err)  => Err(E::custom(format!("{}", err))),
        }
    }
}



/// Allows the LevelFilter to be serialized and deserialized
#[derive(Debug, Clone)]
pub(crate) struct LevelFilter(pub(crate) log::LevelFilter);

impl Deref for LevelFilter {
    type Target = log::LevelFilter;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LevelFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for LevelFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl<'de> Deserialize<'de> for LevelFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LevelFilterVisitor)
    }
}





/***** SETTINGS STRUCT *****/
/// Defines the settings to load, and how to load them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// The output file path
    pub log_path  : PathBuf,
    /// The debug-level
    log_level     : LevelFilter,

    /// The GPU to use
    pub gpu : usize,
}

impl Settings {
    /// Almost-default constructor for the Settings.  
    /// Only options that can be configured during the 'install' subcommand need to be given.
    /// 
    /// **Generic types**
    ///  * `P`: The Path-like type of the logging file path.
    /// 
    /// **Arguments**
    ///  * `log_path`: Path to the logging file.
    ///  * `gpu`: The GPU to attempt to use.
    pub fn default<P: AsRef<Path>>(log_path: P, gpu: usize) -> Self {
        // Resolve anything *-like to *
        let log_path = log_path.as_ref();
        
        // Create the new Settings struct
        Self{
            log_path  : log_path.to_path_buf(),
            log_level : LevelFilter(log::LevelFilter::Error),
            gpu,
        }
    }

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



    /// Returns a muteable reference to the log's LevelFilter.
    #[inline]
    pub fn log_level(&self) -> &log::LevelFilter { &self.log_level.0 }
    
    /// Returns an immuteable reference to the log's LevelFilter.
    #[inline]
    pub fn log_level_mut(&mut self) -> &mut log::LevelFilter { &mut self.log_level.0 }
}
