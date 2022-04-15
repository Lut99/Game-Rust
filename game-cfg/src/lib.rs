/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:53:41
 * Last edited:
 *   15 Apr 2022, 12:44:31
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the crate that concerns itself with loading
 *   configuration from both disk and CLI.
**/


/// The module that contains this crate's errors.
pub mod errors;
/// The module that handles the CLI-part of this crate.
pub mod cli;
/// The module that handles the file-part of this crate.
pub mod file;
/// The module that merges the file and the CLI.
pub mod config;


// Bring some stuff into the global scope
pub use errors::ConfigError as Error;
pub use config::Config;
