//  VERTEX.rs
//    by Lut99
// 
//  Created:
//    11 Aug 2022, 15:56:59
//  Last edited:
//    13 Aug 2022, 13:00:04
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the Vertex interface for the SquarePipeline.
// 

use memoffset::offset_of;

use rust_vk::auxillary::enums::AttributeLayout;
use rust_vk::auxillary::structs::VertexAttribute;
use rust_vk::pools::memory::spec::Vertex;


/***** LIBRARY *****/
/// The Vertex for the TrianglePipeline
#[repr(C)]
#[derive(Clone, Debug)]
pub struct SquareVertex {
    /// The coordinate of the vertex (in 2D space, for now)
    pub pos    : [f32; 2],
    /// The colour of the vertex (as a (normalized) RGB tuple)
    pub colour : [f32; 3],
}

impl Vertex for SquareVertex {
    /// Returns the descriptions that list the attributes (=fields) for this Vertex.
    /// 
    /// # Returns
    /// A list of VertexAttributeDescription that describes the attributes for this Vertex.
    #[inline]
    fn vk_attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {
                binding  : 0,
                location : 0,
                layout   : AttributeLayout::Float2,
                offset   : offset_of!(SquareVertex, pos),
            },
            VertexAttribute {
                binding  : 0,
                location : 1,
                layout   : AttributeLayout::Float3,
                offset   : offset_of!(SquareVertex, colour),
            }
        ]
    }

    /// Returns the size (in bytes) of each Vertex.
    #[inline]
    fn vk_size() -> usize { std::mem::size_of::<Self>() }
}
