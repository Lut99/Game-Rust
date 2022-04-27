/* PIPELINE.rs
 *   by Lut99
 *
 * Created:
 *   23 Apr 2022, 17:26:39
 * Last edited:
 *   27 Apr 2022, 11:57:17
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
pub use crate::auxillary::{AttachmentBlendState, BlendFactor, BlendOp, ColourBlendState, ColourMask, CompareOp, DepthTestingState, DynamicState, LogicOp, MultisampleState, RasterizerState, StencilOp, StencilOpState, VertexAssemblyState, VertexInputState, VertexTopology, ViewportState};
pub use crate::layout::{Error as PipelineLayoutError, PipelineLayout};


/***** LIBRARY *****/
/// May speed up pipeline construction by caching the results and re-using that when possible.
pub struct PipelineCache {
    
}



/// Extended constructor for the Pipeline that may be used to configure it.
pub struct PipelineBuilder {
    /// Collects errors until build() gets called.
    error : Option<Error>,
    
    // Default stuff
    /// Describes how we treat the input vertices.
    vertex_assembly : VertexAssemblyState,
    /// Describes the multisample stage
    _multisampling  : MultisampleState,
    /// Describes if and how depth testing is done
    depth_testing   : DepthTestingState,
    /// Describes how to write colours to the output frame
    colour_blending : ColourBlendState,
    /// Defines any dynamic part of the pipeline
    dynamic         : Vec<DynamicState>,

    // Non-default stuff
    /// Describes how the input vertices look like.
    vertex_input  : Option<VertexInputState>,
    /// Describes the output images dimensions, cutoff and depth.
    viewport      : Option<ViewportState>,
    /// Describes the rasterization stage
    rasterization : Option<RasterizerState>,
    /// Sets the layout for this pipeline
    layout        : Option<Arc<PipelineLayout>>,
}

impl PipelineBuilder {
    /// Constructor for the PipelineBuilder.
    /// 
    /// Spawns a new PipelineBuilder with, where possible, default settings.
    /// 
    /// Before calling build(), be sure to first call:
    /// - PipelineBuilder::vertex_input()
    /// - PipelineBuilder::viewport()
    /// - PipelineBuilder::rasterization()
    /// - PipelineBuilder::layout()
    /// 
    /// Also note that any errors that will occur during building will be postponed until the PipelineBuilder::build() call.
    #[inline]
    pub fn new() -> Self {
        debug!("Starting Pipeline construction");
        Self {
            error : None,

            vertex_assembly : VertexAssemblyState {
                topology          : VertexTopology::TriangleList,
                restart_primitive : false,
            },
            _multisampling  : MultisampleState {},
            depth_testing   : DepthTestingState {
                enable_depth   : false,
                enable_write   : false,
                enable_stencil : false,
                enable_bounds  : false,

                compare_op : CompareOp::LessEq,
                
                pre_stencil_test : StencilOpState {
                    on_stencil_fail : StencilOp::Keep,
                    on_depth_fail   : StencilOp::Keep,
                    on_success      : StencilOp::Keep,

                    compare_op   : CompareOp::Always,
                    compare_mask : 0,
                    write_mask   : 0,
                    reference    : 0,  
                },
                post_stencil_test : StencilOpState {
                    on_stencil_fail : StencilOp::Keep,
                    on_depth_fail   : StencilOp::Keep,
                    on_success      : StencilOp::Keep,

                    compare_op   : CompareOp::Always,
                    compare_mask : 0,
                    write_mask   : 0,
                    reference    : 0,  
                },

                min_bound : 1.0,
                max_bound : 0.0,
            },
            colour_blending : ColourBlendState {
                enable_logic : false,
                logic_op     : LogicOp::Copy,

                attachment_states : vec![AttachmentBlendState {
                    enable_blend : false,
                    
                    src_colour : BlendFactor::One,
                    dst_colour : BlendFactor::Zero,
                    colour_op  : BlendOp::Add,

                    src_alpha : BlendFactor::One,
                    dst_alpha : BlendFactor::Zero,
                    alpha_op  : BlendOp::Add,

                    write_mask : ColourMask::ALL,
                }],
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            },
            dynamic : vec![],

            vertex_input  : None,
            viewport      : None,
            rasterization : None,
            layout        : None,
        }
    }

    /// Constructor for the PipelineBuilder that already initializes itself to use the given pipeline as a base.
    /// 
    /// Convenience function for calling PipelineBuilder::new() and then PipelineBuilder::set_pipeline().
    /// 
    /// This means that no functions are mandatory to call, but can instead be used to update the builder.
    /// 
    /// Note that any errors that will occur during building will be postponed until the PipelineBuilder::build() call.
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
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn set_cache(mut self, cache: Arc<PipelineCache>) -> Self {
        if self.error.is_some() { return self; }
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
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn set_pipeline(mut self, pipeline: Arc<Pipeline>) -> Self {
        if self.error.is_some() { return self; }
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
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn vertex_input(mut self, info: VertexInputState) -> Self {
        if self.error.is_some() { return self; }

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
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn vertex_assembly(mut self, info: VertexAssemblyState) -> Self {
        if self.error.is_some() { return self; }

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
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn viewport(mut self, info: ViewportState) -> Self {
        if self.error.is_some() { return self; }

        // Set the state
        self.viewport = Some(info);

        // Done, return us again
        debug!("Defined viewport state");
        self
    }

    /// Defines the configuration of the rasterization stage.
    /// 
    /// This is one of the non-default functions that must always be called to define the input (unless from_pipeline() is used as constructor or set_pipeline() is called).
    /// 
    /// # Arguments
    /// - `info`: The new Rasterization struct that describes the config.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn rasterization(mut self, info: RasterizerState) -> Self {
        if self.error.is_some() { return self; }

        // Set the state
        self.rasterization = Some(info);

        // Done, return us again
        debug!("Defined rasterization state");
        self
    }

    /// Define a non-default configuration of how to multisample.
    /// 
    /// By default, no multisampling is used.
    /// 
    /// # Arguments
    /// - `info`: The new Multisampling struct that describes the config.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn multisampling(self, _info: MultisampleState) -> Self {
        if self.error.is_some() { return self; }

        warn!("Called useless PipelineBuilder::multisampling() function");
        self
    }

    /// Define a non-default configuration of if and how to perform depth testing.
    /// 
    /// By default, no depth testing is used.
    /// 
    /// # Arguments
    /// - `info`: The new DepthTestingState struct that describes the config.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn depth_testing(mut self, info: DepthTestingState) -> Self {
        if self.error.is_some() { return self; }

        // Set the state
        self.depth_testing = info;

        // Done, return us again
        debug!("Defined non-default depth testing state");
        self
    }

    /// Define a non-default configuration of how to multisample.
    /// 
    /// By default, the source colour fragments are always copied over the destination ones already present in the frame.
    /// 
    /// # Arguments
    /// - `info`: The new ColourBlendState struct that describes the config.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn colour_blending(mut self, info: ColourBlendState) -> Self {
        if self.error.is_some() { return self; }

        // Set the state
        self.colour_blending = info;

        // Done, return us again
        debug!("Defined non-default colour blending state");
        self
    }

    /// Defines the parts of the Pipeline that will be dynamic.
    /// 
    /// By default, no dynamic state is defined.
    /// 
    /// # Arguments
    /// - `info`: A list of Pipeline parts (as DynamicStates) to make dynamic.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn dynamic_state(mut self, states: Vec<DynamicState>) -> Self {
        if self.error.is_some() { return self; }

        // Set the state
        self.dynamic = states;

        // Done, return us again
        debug!("Defined non-default dynamic state");
        self
    }

    /// Defines the layout (in terms of resources) for this Pipeline.
    /// 
    /// This is one of the non-default functions that must always be called to define the input (unless from_pipeline() is used as constructor or set_pipeline() is called).  
    /// That said, one may also call try_layout().
    /// 
    /// # Arguments
    /// - `layout`: The new PipelineLayout struct that describes the layout.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn layout(mut self, layout: Arc<PipelineLayout>) -> Self {
        if self.error.is_some() { return self; }

        // Set the layout
        self.layout = Some(layout);

        // Done, return us again
        debug!("Defined pipeline layout");
        self
    }

    /// Defines the layout (in terms of resources) for this Pipeline.
    /// 
    /// This is one of the non-default functions that must always be called to define the input (unless from_pipeline() is used as constructor or set_pipeline() is called).  
    /// That said, one may also call layout().
    /// 
    /// # Arguments
    /// - `layout`: The result of a new PipelineLayout constructor that may contain the pipeline layout.
    /// 
    /// # Returns
    /// Because this function is consuming, returns the same instance of self as passed to it.
    /// 
    /// # Errors
    /// This function doesn't error directly, but may pass any incoming errors to the PipelineBuilder::build() call.
    pub fn try_layout(mut self, layout: Result<Arc<PipelineLayout>, PipelineLayoutError>) -> Self {
        if self.error.is_some() { return self; }

        // Try to unwrap it
        let layout = match layout {
            Ok(layout) => layout,
            Err(err)   => {
                // Set the error internally and immediately continue
                self.error = Some(Error::LayoutCreateError{ err });
                return self;
            }
        };

        // Otherwise, set the layout
        self.layout = Some(layout);

        // Done, return us again
        debug!("Defined pipeline layout");
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
