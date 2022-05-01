/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 14:09:20
 * Last edited:
 *   01 May 2022, 17:26:23
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to our own wrapper around Vulkan.
**/

/// The module for the the component lists.
pub mod errors;
/// The module for extra traits and other interfaces.
pub mod spec;
/// The module for wrapper structs around Vulkan structs
pub mod auxillary;
/// The module for the instance
pub mod instance;
/// The module for the device
pub mod device;
/// The module for the surface
pub mod surface;
/// The module for the swapchain
pub mod swapchain;
/// The module for the shaders
pub mod shader;
/// The module for descriptor layouts and sets
pub mod descriptors;
/// The module for the pipeline layout
pub mod layout;
/// The module for the render pass(es)
pub mod render_pass;
/// The module for the pipeline
pub mod pipeline;
/// The module for the various pools
pub mod pools;
/// The module for the images & image views
pub mod image;
/// The module that contains synchronization primitives
pub mod sync;

// Bring some components into the general package namespace
