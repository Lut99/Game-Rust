//  SPEC.rs
//    by Lut99
// 
//  Created:
//    31 Jul 2022, 12:36:36
//  Last edited:
//    31 Jul 2022, 12:37:55
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines public interfaces and structs, such as the pipeline's notion
//!   of a Vertex.
// 

use memoffset::offset_of;

use game_vk::auxillary::enums::AttributeLayout;
use game_vk::auxillary::structs::VertexAttribute;


/***** LIBRARY *****/
/// The Vertex for the TrianglePipeline
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The coordinate of the vertex (in 2D space, for now)
    pub pos    : [f32; 2],
    /// The colour of the vertex (as a (normalized) RGB tuple)
    pub colour : [f32; 3],
}

impl Vertex {
    /// Returns the descriptions that list the attributes (=fields) for this Vertex.
    /// 
    /// # Returns
    /// A list of VertexAttributeDescription that describes the attributes for this Vertex.
    #[inline]
    pub fn vk_attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {
                binding  : 0,
                location : 0,
                layout   : AttributeLayout::Float2,
                offset   : offset_of!(Vertex, pos),
            },
            VertexAttribute {
                binding  : 0,
                location : 1,
                layout   : AttributeLayout::Float3,
                offset   : offset_of!(Vertex, colour),
            }
        ]
    }

    /// Returns the size (in bytes) of each Vertex.
    #[inline]
    pub const fn vk_size() -> usize { std::mem::size_of::<Self>() }
}
