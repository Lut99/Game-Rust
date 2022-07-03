/* VERTEX.rs
 *   by Lut99
 *
 * Created:
 *   03 Jul 2022, 11:21:05
 * Last edited:
 *   03 Jul 2022, 11:22:40
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Vertex definition for the TrianglePipeline.
**/


/***** LIBRARY *****/
/// The Vertex for the TrianglePipeline
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The coordinate of the vertex (in 2D space, for now)
    pos    : [f32; 2],
    /// The colour of the vertex (as a (normalized) RGB tuple)
    colour : [f32; 3],
}

impl Vertex {
    
}
