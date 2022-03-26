/* CONFIG.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 11:48:52
 * Last edited:
 *   26 Mar 2022, 12:52:52
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the code that merges the settings file input with the
 *   CLI-overrides.
**/

use std::env;
use std::path::{Path, PathBuf};

use clap::Parser;
use log::LevelFilter;
use time::OffsetDateTime;
use time::format_description;

use crate::errors::ConfigError as Error;
use crate::cli::{Arguments, ArgumentSubcommand};
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

/// Given a log path, resolves the special tokens in there with up-to-date information.
/// 
/// **Generic types**
///  * `P`: The Path-like type of the logging path we were given.
/// 
/// **Arguments**
///  * `log_path`: The logging path to fill in.
/// 
/// **Returns**  
/// The fixed path as a PathBuf on success, or else an Error.
pub fn fill_log_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    // Get the current time
    let now: OffsetDateTime = match OffsetDateTime::now_local() {
        Ok(now)  => now,
        Err(err) => { return Err(Error::LocalTimeError{ err }); }
    };

    // Render the current date
    let formatter = format_description::parse("[year]-[month]-[day]").unwrap_or_else(|err| panic!("Could not format date formatter: {}", err));
    let date = now.format(&formatter).unwrap_or_else(|err| panic!("Failed to format date: {}", err));
    
    // Render the current time
    let formatter = format_description::parse("[hour]-[minute]-[second]").unwrap_or_else(|err| panic!("Could not format time formatter: {}", err));
    let time = now.format(&formatter).unwrap_or_else(|err| panic!("Failed to format time: {}", err));

    // Convert the path to a string
    let path: &Path = path.as_ref();
    let spath: &str = match path.to_str() {
        Some(spath) => spath,
        None        => { return Err(Error::PathToStringError{ path: path.to_path_buf() }); }
    };

    // Replace the relevant parts
    let spath = spath.replace("%TODAY%", &date);
    let spath = spath.replace("%NOW%", &time);

    // Done!
    Ok(PathBuf::from(spath))
}





/***** LIBRARY *****/
/// The Config struct, which contains the configuration as loaded from both disk and CLI.
#[derive(Debug)]
pub struct Config {
    /// The location of the settings file
    pub settings_path    : PathBuf,
    /// The location of the logging file (in general)
    pub log_path         : PathBuf,
    /// The location of the logging file (this run)
    pub session_log_path : PathBuf,
    /// The verbosity of the logging (the CLI-part, at least)
    pub log_level        : LevelFilter,

    /// Decides what the executable should do next
    pub action : Action,
}

impl Config {
    /// Constructor for the Config, that initializes it with configuration from both the CLI and disk.
    /// 
    /// **Returns**  
    /// A new Config on success, or else an Error.
    pub fn new() -> Result<Self, Error> {
        // Load the CLI
        let args: Arguments = Arguments::parse();

        // Switch on the action taken (since not everything requires the settings file)
        let config: Config = match args.subcommand {
            ArgumentSubcommand::Install{ config_dir, log_dir } => {
                // Derive the settings and logs paths from the given arguments
                let settings_path = config_dir.join("settings.json");
                let log_path      = log_dir.join("%TODAY%_%NOW%.json");
                let log_level     = args.log_level.unwrap_or(LevelFilter::Error);

                // Resolve the paths relative to the executable
                let settings_path    = reresolve_path(settings_path)?;
                let log_path         = reresolve_path(log_path)?;
                let session_log_path = fill_log_path(&log_path)?;

                // Wrap it in an action
                let action = Action::Install{
                    config_dir : reresolve_path(config_dir)?,
                    log_dir    : reresolve_path(log_dir)?,
                };

                // Wrap it in a config and done!
                Config {
                    settings_path,
                    log_path,
                    session_log_path,
                    log_level,

                    action,
                }
            },

            ArgumentSubcommand::List{} => {
                // Load the settings file, after having resolved the location
                let settings_path = reresolve_path(args.settings_path)?;
                let settings: Settings = match Settings::from_path(&settings_path) {
                    Ok(settings) => settings,
                    Err(err)     => { return Err(Error::SettingsLoadError{ err }); }
                };

                // Resolve the toplevel fields
                let log_path         = reresolve_path(args.log_path.unwrap_or(settings.log_path.clone()))?;
                let session_log_path = fill_log_path(&log_path)?;
                let log_level        = args.log_level.unwrap_or(settings.log_level().clone());

                // Create the action
                let action = Action::List{};

                // Wrap it in a config and done!
                Config {
                    settings_path,
                    log_path,
                    session_log_path,
                    log_level,

                    action,
                }
            },

            ArgumentSubcommand::Run{ gpu } => {
                // Load the settings file, after having resolved the location
                let settings_path = reresolve_path(args.settings_path)?;
                let settings: Settings = match Settings::from_path(&settings_path) {
                    Ok(settings) => settings,
                    Err(err)     => { return Err(Error::SettingsLoadError{ err }); }
                };

                // Resolve the toplevel fields
                let log_path         = reresolve_path(args.log_path.unwrap_or(settings.log_path.clone()))?;
                let session_log_path = fill_log_path(&log_path)?;
                let log_level        = args.log_level.unwrap_or(settings.log_level().clone());

                // Create the action
                let action = Action::Run{
                    gpu : gpu.unwrap_or(settings.gpu),
                };

                // Wrap it in a config and done!
                Config {
                    settings_path,
                    log_path,
                    session_log_path,
                    log_level,

                    action,
                }
            },
        };

        // Done, return
        Ok(config)
    }
}



/// Defines the action that the executable should take.
#[derive(Debug)]
pub enum Action {
    /// The executable will generate the required files and directory structures.
    Install {
        /// The location of the game's config files
        config_dir : PathBuf,
        /// The location of the game's log files
        log_dir    : PathBuf,  
    },

    ///  The executable should list all GPUs it can find.
    List {},

    /// The exeuctable should run the game.
    Run {
        /// Overrides the GPU specified in the settings file.
        gpu : usize,
    },
}
