//  SPEC.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 13:01:17
//  Last edited:
//    07 Aug 2022, 18:30:47
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains interfaces and other structs for the GFX crate.
// 

use std::fmt::{Display, Debug, Formatter, Result as FResult};
use std::str::FromStr;

use semver::Version;

use game_utl::traits::AsAny;

use crate::errors::PipelineError;


/***** AUXILLARY NEWTYPES *****/
/// Defines an ID to reference specific windows.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum WindowId {
    /// The main Window to which the RenderSystem renders.
    Main,
}

impl Display for WindowId {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use WindowId::*;
        match self {
            Main => write!(f, "Main"),
        }
    }
}





/***** RENDER PIPELINE TRAIT *****/
/// Defines a Render-capable pipeline.
pub trait RenderPipeline: 'static + AsAny {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderSystem to perform a render pass.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), PipelineError>;



    /// Returns the name of the pipeline.
    fn name(&self) -> &'static str;
}





/***** ARGUMENT STRUCTS *****/
/// The AppInfo struct defines information about the application itself.
#[derive(Clone, Debug)]
pub struct AppInfo {
    /// The name of the application.
    pub name    : String,
    /// The version of the application.
    pub version : Version,

    /// The name of the application's engine.
    pub engine_name    : String,
    /// The version of the application's engine.
    pub engine_version : Version,
}

impl AppInfo {
    /// Convenience constructor that does some implicit type convertion.
    /// 
    /// # Generic types
    /// - `S1`: The String-like type of the `name`.
    /// - `V1`: The &str-like type of the `version` that we will parse to a Version.
    /// - `S2`: The String-like type of the `engine_name`.
    /// - `V2`: The &str-like type of the `engine_version` that we will parse to a Version.
    /// 
    /// # Arguments
    /// - `name`: The name of the application.
    /// - `version`: The version of the application.
    /// - `engine_name`: The name of the application's engine.
    /// - `engine_version`: The version of the application's engine.
    #[inline]
    pub fn new<S1: Into<String>, V1: AsRef<str>, S2: Into<String>, V2: AsRef<str>>(name: S1, version: V1, engine_name: S2, engine_version: V2) -> Self {
        Self {
            name    : name.into(),
            version : Version::from_str(version.as_ref()).unwrap_or_else(|err| panic!("Failed to parse application Version from '{}': {}", version.as_ref(), err)),

            engine_name    : engine_name.into(),
            engine_version : Version::from_str(engine_version.as_ref()).unwrap_or_else(|err| panic!("Failed to parse engine Version from '{}': {}", engine_version.as_ref(), err)),
        }
    }
}



/// The VulkanInfo-struct defines information that is destined for the Vulkan backend.
#[derive(Clone, Debug)]
pub struct VulkanInfo {
    /// The index of the GPU which we will use for rendering.
    pub gpu   : usize,
    /// If true, then we enable Vulkan debug layers.
    pub debug : bool,
}
