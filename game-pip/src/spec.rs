//  SPEC.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2022, 15:39:32
//  Last edited:
//    11 Aug 2022, 15:40:38
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines public interfaces and structs for the `game-pip` crate.
// 

use game_utl::traits::AsAny;

pub use crate::errors::RenderPipelineError as Error;


/***** LIBRARY *****/
/// Defines a Render-capable pipeline.
pub trait RenderPipeline: 'static + AsAny {
    /// Renders a single frame to the given renderable target.
    /// 
    /// This function performs the actual rendering, and may be called by the RenderSystem to perform a render pass.
    /// 
    /// # Errors
    /// This function may error whenever it likes. If it does, it should return something that implements Error, at which point the program's execution is halted.
    fn render(&mut self) -> Result<(), Error>;



    /// Returns the name of the pipeline.
    fn name(&self) -> &'static str;
}
