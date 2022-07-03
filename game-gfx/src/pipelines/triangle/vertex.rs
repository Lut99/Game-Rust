/* VERTEX.rs
 *   by Lut99
 *
 * Created:
 *   03 Jul 2022, 11:21:05
 * Last edited:
 *   03 Jul 2022, 14:42:41
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the Vertex definition for the TrianglePipeline.
**/

use memoffset::offset_of;

use game_vk::auxillary::{AttributeLayout, VertexAttribute};


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
    /// Returns the descriptions that list the attributes (=fields) for this Vertex.
    /// 
    /// # Returns
    /// A list of VertexAttributeDescription that describes the attributes for this Vertex.
    #[inline]
    pub const fn vk_attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {
                binding  : 0,
                location : 0,
                layout   : AttributeLayout::Float2,
                offset   : offset_of!(Self, pos),
            },
            VertexAttribute {
                binding  : 0,
                location : 1,
                layout   : AttributeLayout::Float3,
                offset   : offset_of!(Self, pos),
            }
        ]
    }

    /// Returns the size (in bytes) of each Vertex.
    #[inline]
    pub const fn vk_size() -> usize { std::mem::size_of::<Self>() }
}
