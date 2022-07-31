//  SPEC.rs
//    by Lut99
// 
//  Created:
//    30 Jul 2022, 18:18:45
//  Last edited:
//    31 Jul 2022, 12:01:55
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs for the `game-gfx` crate.
// 

use semver::Version;

use game_cfg::spec::WindowMode;


/***** LIBRARY *****/
/// Defines a struct that carries some application info.
#[derive(Clone, Debug)]
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



/// Defines a struct that carries Vulkan info that configures the backend.
#[derive(Clone, Debug)]
pub struct VulkanInfo {
    /// The index (as found by Vulkan) for the GPU to use as renderer.
    pub gpu : usize,
}



/// Defines a struct that carries information about the Window.
#[derive(Clone, Debug)]
pub struct WindowInfo {
    /// The title of the Window.
    pub title : String,
    /// The window mode of the Window.
    pub mode  : WindowMode,
}
