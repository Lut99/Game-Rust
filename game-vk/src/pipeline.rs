/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   23 Apr 2022, 17:26:39
 * Last edited:
 *   23 Apr 2022, 21:55:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements a wrapper around the Vulkan pipeline.
**/

use std::sync::Arc;

use ash::vk;
use log::{debug, warn};

pub use crate::errors::PipelineError as Error;
pub use crate::auxillary::{VertexAssembly, VertexInput, VertexTopology, Viewport};


/***** LIBRARY *****/
/// May speed up pipeline construction by caching the results and re-using that when possible.
pub struct PipelineCache {
    
}



/// Extended constructor for the Pipeline that may be used to configure it.
pub struct PipelineBuilder {
    /// Collects errors until build() gets called.
    errors : Vec<Error>,
    
    // Default stuff
    /// Describes how we treat the input vertices.
    vertex_assembly : VertexAssembly,

    // Non-default stuff
    /// Describes how the input vertices look like.
    vertex_input : Option<VertexInput>,
    /// Describes the output images dimensions, cutoff and depth.
    viewport     : Option<Viewport>,
}

impl PipelineBuilder {
    /// Constructor for the PipelineBuilder.
    /// 
    /// Spawns a new PipelineBuilder with, where possible, default settings.
    /// 
    /// Before calling build(), be sure to first call:
    /// - vertex_input()
    /// 
    /// Also note that any errors that will occur during building will be postponed until the build() call.
    #[inline]
    pub fn new() -> Self {
        debug!("Starting Pipeline construction");
        Self {
            errors : Vec::new(),

            vertex_assembly : VertexAssembly {
                topology          : VertexTopology::TriangleList,
                restart_primitive : false,
            },

            vertex_input : None,
            viewport     : None,
        }
    }

    /// Constructor for the PipelineBuilder that already initializes itself to use the given pipeline as a base.
    /// 
    /// Convenience function for calling new() and then set_pipeline().
    /// 
    /// This means that no functions are mandatory to call, but can instead be used to update the builder.
    /// 
    /// Note that any errors that will occur during building will be postponed until the build() call.
    /// 
    /// # Arguments
    /// - `pipeline`: The Pipeline to base this new one off.
    #[inline]
    pub fn from_pipeline(pipeline: Arc<Pipeline>) -> Self {
        // Construct a new version of ourselves and set the pipeline
        Self::new().set_pipeline(pipeline)
    }



    /// Uses the given PipelineCache as a pool to create new pipelines with.
    /// 
    /// # Arguments
    /// - `cache`: The PipelineCache to cache new pipelines in, and to possibly speedup building pipelines we build before.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the build() call.
    pub fn set_cache(mut self, cache: Arc<PipelineCache>) -> Self {
        warn!("PipelineBuilder::set_cache() is not yet implemented");
        self
    }

    /// Uses the given pipeline as a base for constructing the new one.
    /// 
    /// # Arguments
    /// - `pipeline`: The Pipeline to base this new one off.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the build() call.
    pub fn set_pipeline(mut self, pipeline: Arc<Pipeline>) -> Self {
        warn!("PipelineBuilder::set_pipeline() is not yet implemented");
        self
    }



    /// Define a VertexInputState for this Pipeline.
    /// 
    /// This is one of the non-default functions that must always be called to define the input (unless from_pipeline() is used as constructor or set_pipeline() is called).
    /// 
    /// # Arguments
    /// - `info`: The new VertexInputState struct that describes how the input vertices look like.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the build() call.
    pub fn vertex_input(mut self, info: VertexInput) -> Self {
        // Set the state
        self.vertex_input = Some(info);

        // Done, return us again
        debug!("Defined vertex input state");
        self
    }

    /// Define a non-default VertexAssemblyState for this Pipeline.
    /// 
    /// By default, the Pipeline uses a VertexAssemblyState with:
    /// - A triangle topology.
    /// - No primitive restart enabled (i.e., special value that allows mid-triangle to reset).
    /// 
    /// # Arguments
    /// - `info`: The new VertexAssemblyState struct that describes what to do with the input vertices.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the build() call.
    pub fn vertex_assembly(mut self, info: VertexAssembly) -> Self {
        // Set the state
        self.vertex_assembly = info;

        // Done, return us again
        debug!("Defined non-default vertex assembly state");
        self
    }

    /// Defines how the viewport looks like, i.e., the size of the output frame.
    /// 
    /// This is one of the non-default functions that must always be called to define the input (unless from_pipeline() is used as constructor or set_pipeline() is called).
    /// 
    /// # Arguments
    /// - `info`: The new Viewport struct that describes how the output frame looks like.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the build() call.
    pub fn viewport(mut self, info: Viewport) -> Self {
        // Set the state
        self.viewport = Some(info);

        // Done, return us again
        debug!("Defined non-default viewport state");
        self
    }
}



/// Wraps around a Vulkan Pipeline, which describes the process of rendering some vertices to an image.
pub struct Pipeline {
    
}

impl Pipeline {
    /// Private constructor for the Pipeline.
    /// 
    /// Should only be called from the Builder.
    /// 
    /// # Arguments
    /// - 
    #[inline]
    fn new() -> Self {
        Self {
            
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        
    }
}
