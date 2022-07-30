/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   30 Jul 2022, 18:18:45
 * Last edited:
 *   30 Jul 2022, 18:23:25
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines (public) interfaces and structs for the `game-gfx` crate.
**/

use semver::Version;


/***** LIBRARY *****/
/// Defines a struct that carries some application info.
pub struct AppInfo {
    /// The name of the application.
    pub name    : String,
    /// The version of the application.
    pub version : Version,

    /// The name of the engine that backs the application.
    pub engine_name    : String,
    /// The version of the engine that backs the application.
    pub engine_version : Version,
}

impl AppInfo {
    /// Shorthand constructor that defines the engine as default.
    /// 
    /// # Arguments
    /// - `name`: The name of the current application.
    /// - `version`: The version of the current application.
    /// 
    /// # Returns
    /// A new AppInfo struct.
    #[inline]
    pub fn with_default_engine(name: String, version: Version) -> Self {
        Self {
            name,
            version,

            engine_name    : "game-gfx".into(),
            engine_version : Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or_else(|err| panic!("Failed to parse Cargo package version '{}': {}", env!("CARGO_PKG_VERSION"), err)),
        }
    }
}
