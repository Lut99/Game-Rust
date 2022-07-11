/* ERRORS.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:12:24
 * Last edited:
 *   11 Jul 2022, 19:11:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Collects all errors in the crate.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;


/***** ERRORS *****/
/// Lists errors that occur with the Settings struct.
#[derive(Debug)]
pub enum SettingsError {
    /// The given resolution string did not have an 'x'.
    MissingX{ raw: String },
    /// The given number is not an unsigned integer.
    IllegalUnsignedInteger{ raw: String, err: std::num::ParseIntError },

    /// Could not parse a WindowMode.
    UnknownWindowMode{ raw: String },

    /// Could not open the Settings file.
    OpenError{ path: PathBuf, err: std::io::Error },
    /// Could not parse the Settings file.
    ParseError{ path: PathBuf, err: serde_json::Error },

    /// Could not create the new Settings file.
    CreateError{ path: PathBuf, err: std::io::Error },
    /// Could not write the Settings file to the given location.
    WriteError{ path: PathBuf, err: serde_json::Error },
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use SettingsError::*;
        match self {
            MissingX{ raw }                    => write!(f, "Resolution string '{}' does not have an 'x' to separate width and height", raw),
            IllegalUnsignedInteger{ raw, err } => write!(f, "Could not parse '{}' as an unsigned int: {}", raw, err),
            

            UnknownWindowMode{ raw } => write!(f, "Unknown window mode '{}'", raw),

            OpenError{ path, err }  => write!(f, "Could not open settings file '{}': {}", path.display(), err),
            ParseError{ path, err } => write!(f, "Could not parse settings file '{}': {}", path.display(), err),

            CreateError{ path, err } => write!(f, "Could not create new settings file '{}': {}", path.display(), err),
            WriteError{ path, err }  => write!(f, "Could not write settings file to '{}': {}", path.display(), err),
        }
    }
}

impl Error for SettingsError {}



/// Lists errors that occur in the Config struct.
#[derive(Debug)]
pub enum ConfigError {
    /// Could not get the path of the executable
    ExecutablePathError{ err: std::io::Error },
    /// Could not get the parent from a path
    PathParentError{ path: PathBuf },
    /// Could not convert the given path to string
    PathToStringError{ path: PathBuf },
    /// The given relative path tried to escape the parent path
    RelativeEscape{ base: PathBuf, path: PathBuf },

    /// Could not load the settings file.
    SettingsLoadError{ err: SettingsError },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ConfigError::*;
        match self {
            ExecutablePathError{ err }   => write!(f, "Could not get path of executable: {}", err),
            PathParentError{ path }      => write!(f, "Could not get parent folder of '{}'", path.display()),
            PathToStringError{ path }    => write!(f, "Could not convert '{}' to a string", path.display()),
            RelativeEscape{ base, path } => write!(f, "Given path '{}' tries to escape base path '{}': use absolute paths instead", path.display(), base.display()),

            SettingsLoadError{ err } => write!(f, "Could not load the settings file: {}", err),
        }
    }
}

impl Error for ConfigError {}
