/* ENUMS.rs
 *   by Lut99
 *
 * Created:
 *   09 Jul 2022, 12:23:22
 * Last edited:
 *   09 Jul 2022, 12:49:57
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements various auxillary enums that are used throughout this crate
 *   and which wrap / stand-in for Vulkan structs.
**/

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FResult};
use std::str::FromStr;

use ash::vk;

use crate::errors::{AttributeLayoutError, ExtensionError};


/***** INSTANCE *****/
/// An enum that describes instance extensions used in the Game.
#[derive(Copy, Clone, Debug)]
pub enum InstanceExtension {
    /// A dummy extension as a temporary placeholder
    Dummy,
}

impl InstanceExtension {
    /// Constant function to get the string value of the InstanceExtension.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        use InstanceExtension::*;
        match self {
            Dummy => "dummy",
        }
    }
}

impl Display for InstanceExtension {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for InstanceExtension {
    type Err = ExtensionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "dummy" => Ok(InstanceExtension::Dummy),
            value   => Err(ExtensionError::UnknownInstanceExtension{ got: value.into() }),
        }
    }
}



/// An enum that describes instance layers used in the Game.
#[derive(Copy, Clone, Debug)]
pub enum InstanceLayer {
    /// The Khronos validation layer
    KhronosValidation,
}

impl InstanceLayer {
    /// Constant function to get the string value of the InstanceLayer.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        use InstanceLayer::*;
        match self {
            KhronosValidation => "VK_LAYER_KHRONOS_validation",
        }
    }
}

impl Display for InstanceLayer {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for InstanceLayer {
    type Err = ExtensionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "VK_LAYER_KHRONOS_validation" => Ok(Self::KhronosValidation),
            value                         => Err(ExtensionError::UnknownInstanceLayer{ got: value.into() }),
        }
    }
}





/***** DEVICES *****/
/// Enumerates the possible Device types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceKind {
    /// A discrete GPU. Is given the highest 'CPU disconnectedness' score.
    Discrete,
    /// An intergrated but dedicated GPU. Is given the second highest 'CPU disconnectedness' score.
    Integrated,
    /// A Virtual GPU, which is given the third-worst 'CPU disconnectedness' score.
    Virtual,
    /// No dedicated GPU at all, just the CPU doing GPU stuff. Is given the fourth-worst 'CPU disconnectedness' score.
    Cpu,
    /// A GPU type which we do not know, which we prefer the least (worst 'CPU disconnectedness' score).
    Other,
}

impl DeviceKind {
    /// Returns a so-ca,lled 'CPU disconnectedness' score, which we hope to equate to a device's power when comparing multiple.
    /// 
    /// We assume that devices with a higher score are more discrete, and thus more powerful.
    /// 
    /// # Returns
    /// The score as an unsigned integer.
    #[inline]
    pub fn score(&self) -> u32 {
        use DeviceKind::*;
        match self {
            Discrete   => 4,
            Integrated => 3,
            Virtual    => 2,
            Cpu        => 1,
            Other      => 0,
        }
    }
}

impl Ord for DeviceKind {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by CPU disconnectedness
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for DeviceKind {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for DeviceKind {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use DeviceKind::*;
        match self {
            Discrete   => write!(f, "Discrete GPU"),
            Integrated => write!(f, "Integrated GPU"),
            Virtual    => write!(f, "Virtual GPU"),
            Cpu        => write!(f, "CPU"),
            Other      => write!(f, "Other"),
        }
    }
}

impl From<vk::PhysicalDeviceType> for DeviceKind {
    #[inline]
    fn from(value: vk::PhysicalDeviceType) -> Self {
        match value {
            vk::PhysicalDeviceType::DISCRETE_GPU   => DeviceKind::Discrete,
            vk::PhysicalDeviceType::INTEGRATED_GPU => DeviceKind::Integrated,
            vk::PhysicalDeviceType::VIRTUAL_GPU    => DeviceKind::Virtual,
            vk::PhysicalDeviceType::CPU            => DeviceKind::Cpu,
            _                                      => DeviceKind::Other,
        }
    }
}

impl From<DeviceKind> for vk::PhysicalDeviceType {
    #[inline]
    fn from(value: DeviceKind) -> Self {
        match value {
            DeviceKind::Discrete   => vk::PhysicalDeviceType::DISCRETE_GPU,
            DeviceKind::Integrated => vk::PhysicalDeviceType::INTEGRATED_GPU,
            DeviceKind::Virtual    => vk::PhysicalDeviceType::VIRTUAL_GPU,
            DeviceKind::Cpu        => vk::PhysicalDeviceType::CPU,
            DeviceKind::Other      => vk::PhysicalDeviceType::OTHER,
        }
    }
}



/// An enum that describes device extensions used in the Game.
#[derive(Copy, Clone, Debug)]
pub enum DeviceExtension {
    /// The Swapchain device extension.
    Swapchain,
}

impl DeviceExtension {
    /// Constant function to get the string value of the DeviceExtension.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        use DeviceExtension::*;
        match self {
            Swapchain => "VK_KHR_swapchain",
        }
    }
}

impl Display for DeviceExtension {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for DeviceExtension {
    type Err = ExtensionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "VK_KHR_swapchain" => Ok(DeviceExtension::Swapchain),
            value              => Err(ExtensionError::UnknownDeviceExtension{ got: value.into() }),
        }
    }
}



/// An enum that describes device layers used in the Game.
#[derive(Copy, Clone, Debug)]
pub enum DeviceLayer {
    /// A dummy extension as a temporary placeholder
    Dummy,
}

impl DeviceLayer {
    /// Constant function to get the string value of the DeviceLayer.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        use DeviceLayer::*;
        match self {
            Dummy => "dummy",
        }
    }
}

impl Display for DeviceLayer {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for DeviceLayer {
    type Err = ExtensionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "dummy" => Ok(DeviceLayer::Dummy),
            value   => Err(ExtensionError::UnknownDeviceLayer{ got: value.into() }),
        }
    }
}





/***** QUEUES *****/
/// Enum that defines the types of queues that the Game has.
#[derive(Clone, Copy, Debug)]
pub enum QueueKind {
    /// The queue that is used for graphics operations (rendering & (technically) presenting)
    Graphics,
    /// The queue that is used for memory operations (transferring)
    Memory,
    /// The queue that is used for present operations (& technically rendering)
    Present,
    /// The queue that is used for compute operations
    Compute,
}





/***** DESCRIPTOR SETS / LAYOUTS *****/
/// Defines the possible Descriptor types.
#[derive(Clone, Copy, Debug)]
pub enum DescriptorKind {
    /// Describes a uniform buffer.
    UniformBuffer,
    /// Describes a storage buffer.
    StorageBuffer, 
    /// Describes a dynamic uniform buffer.
    UniformDynamicBuffer,
    /// Describes a dynamic storage buffer.
    StorageDynamicBuffer, 
    /// Describes a uniform texel buffer.
    UniformTexelBuffer,
    /// Describes a storage texel buffer.
    StorageTexelBuffer, 

    /// Describes an input attachment.
    InputAttachment,
    /// Describes a single storage image.
    StorageImage,
    /// Describes a single, sampled image.
    SampledImage,

    /// Describes a texture sampler.
    Sampler,
    /// Describes a combined image sampler.
    CombindImageSampler,
}

impl From<vk::DescriptorType> for DescriptorKind {
    #[inline]
    fn from(value: vk::DescriptorType) -> Self {
        match value {
            vk::DescriptorType::UNIFORM_BUFFER         => DescriptorKind::UniformBuffer,
            vk::DescriptorType::STORAGE_BUFFER         => DescriptorKind::StorageBuffer,
            vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC => DescriptorKind::UniformDynamicBuffer,
            vk::DescriptorType::STORAGE_BUFFER_DYNAMIC => DescriptorKind::StorageDynamicBuffer,
            vk::DescriptorType::UNIFORM_TEXEL_BUFFER   => DescriptorKind::UniformTexelBuffer,
            vk::DescriptorType::STORAGE_TEXEL_BUFFER   => DescriptorKind::StorageTexelBuffer,

            vk::DescriptorType::INPUT_ATTACHMENT => DescriptorKind::InputAttachment,
            vk::DescriptorType::STORAGE_IMAGE    => DescriptorKind::StorageImage,
            vk::DescriptorType::SAMPLED_IMAGE    => DescriptorKind::SampledImage,

            vk::DescriptorType::SAMPLER                => DescriptorKind::Sampler,
            vk::DescriptorType::COMBINED_IMAGE_SAMPLER => DescriptorKind::CombindImageSampler,

            value => { panic!("Encountered illegal VkDescriptorType value '{}'", value.as_raw()); }
        }
    }
}

impl From<DescriptorKind> for vk::DescriptorType {
    #[inline]
    fn from(value: DescriptorKind) -> Self {
        match value {
            DescriptorKind::UniformBuffer        => vk::DescriptorType::UNIFORM_BUFFER,
            DescriptorKind::StorageBuffer        => vk::DescriptorType::STORAGE_BUFFER,
            DescriptorKind::UniformDynamicBuffer => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            DescriptorKind::StorageDynamicBuffer => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            DescriptorKind::UniformTexelBuffer   => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            DescriptorKind::StorageTexelBuffer   => vk::DescriptorType::STORAGE_TEXEL_BUFFER,

            DescriptorKind::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
            DescriptorKind::StorageImage    => vk::DescriptorType::STORAGE_IMAGE,
            DescriptorKind::SampledImage    => vk::DescriptorType::SAMPLED_IMAGE,

            DescriptorKind::Sampler             => vk::DescriptorType::SAMPLER,
            DescriptorKind::CombindImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        }
    }
}





/***** RENDER PASSES *****/
/// Defines a load operation for attachments.
#[derive(Clone, Copy, Debug)]
pub enum AttachmentLoadOp {
    /// We don't care what the value of the attachment is (so they'll be undefined).
    /// 
    /// # Synchronization
    /// - For colour attachments, this uses the `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT` operation (???).
    /// - For depth / stencil attachments, this uses the `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` operation (???).
    DontCare,

    /// Clear the attachment upon loading. The clear value is specified in the RenderPass.
    /// 
    /// # Synchronization
    /// - For colour attachments, this uses the `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT` operation.
    /// - For depth / stencil attachments, this uses the `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT` operation.
    Clear,
    /// Loads whatever values where already in the attachment.
    /// 
    /// # Synchronization
    /// - For colour attachments, this uses the `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT` operation.
    /// - For depth / stencil attachments, this uses the `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` operation.
    Load,
}

impl From<vk::AttachmentLoadOp> for AttachmentLoadOp {
    #[inline]
    fn from(value: vk::AttachmentLoadOp) -> Self {
        match value {
            vk::AttachmentLoadOp::DONT_CARE => AttachmentLoadOp::DontCare,

            vk::AttachmentLoadOp::CLEAR => AttachmentLoadOp::Clear,
            vk::AttachmentLoadOp::LOAD  => AttachmentLoadOp::Load,

            value => { panic!("Encountered illegal VkAttachmentLoadOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<AttachmentLoadOp> for vk::AttachmentLoadOp {
    #[inline]
    fn from(value: AttachmentLoadOp) -> Self {
        match value {
            AttachmentLoadOp::DontCare => vk::AttachmentLoadOp::DONT_CARE,

            AttachmentLoadOp::Clear => vk::AttachmentLoadOp::CLEAR,
            AttachmentLoadOp::Load  => vk::AttachmentLoadOp::LOAD,
        }
    }
}



/// Defines a store operation for attachments.
#[derive(Clone, Copy, Debug)]
pub enum AttachmentStoreOp {
    /// We don't care what the value of the attachment will be (so they'll be undefined).
    /// 
    /// # Synchronization
    /// - For colour attachments, this uses the `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT` operation (???).
    /// - For depth / stencil attachments, this uses the `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` operation (???).
    DontCare,

    /// Stores the values of the attachment 'permanently' so they may be propagated to the next subpass / presentation.
    /// 
    /// # Synchronization
    /// - For colour attachments, this uses the `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT` operation.
    /// - For depth / stencil attachments, this uses the `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT` operation.
    Store,
}

impl From<vk::AttachmentStoreOp> for AttachmentStoreOp {
    #[inline]
    fn from(value: vk::AttachmentStoreOp) -> Self {
        match value {
            vk::AttachmentStoreOp::DONT_CARE => AttachmentStoreOp::DontCare,

            vk::AttachmentStoreOp::STORE => AttachmentStoreOp::Store,

            value => { panic!("Encountered illegal VkAttachmentStoreOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<AttachmentStoreOp> for vk::AttachmentStoreOp {
    #[inline]
    fn from(value: AttachmentStoreOp) -> Self {
        match value {
            AttachmentStoreOp::DontCare => vk::AttachmentStoreOp::DONT_CARE,

            AttachmentStoreOp::Store => vk::AttachmentStoreOp::STORE,
        }
    }
}

/// The point where a subpass will be attached to the pipeline.
#[derive(Clone, Copy, Debug)]
pub enum BindPoint {
    /// The subpass will be attached in the graphics-part of the pipeline.
    Graphics,
    /// The subpass will be attached in the compute-part of the pipeline.
    Compute,
}

impl From<vk::PipelineBindPoint> for BindPoint {
    #[inline]
    fn from(value: vk::PipelineBindPoint) -> Self {
        match value {
            vk::PipelineBindPoint::GRAPHICS => BindPoint::Graphics,
            vk::PipelineBindPoint::COMPUTE  => BindPoint::Compute,

            value => { panic!("Encountered illegal VkPipelineBindPoint value '{}'", value.as_raw()); }
        }
    }
}

impl From<BindPoint> for vk::PipelineBindPoint {
    #[inline]
    fn from(value: BindPoint) -> Self {
        match value {
            BindPoint::Graphics => vk::PipelineBindPoint::GRAPHICS,
            BindPoint::Compute  => vk::PipelineBindPoint::COMPUTE,
        }
    }
}





/***** PIPELINE *****/
/// Defines the possible layouts for an attribute
#[derive(Clone, Copy, Debug)]
pub enum AttributeLayout {
    /// A two-dimensional vector of 32-bit floating-point numbers
    Float2,
    /// A three-dimensional vector of 32-bit floating-point numbers
    Float3,
}

impl TryFrom<vk::Format> for AttributeLayout {
    type Error = AttributeLayoutError;

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        match value {
            vk::Format::R32G32_SFLOAT    => Ok(AttributeLayout::Float2),
            vk::Format::R32G32B32_SFLOAT => Ok(AttributeLayout::Float3),
            value                        => Err(AttributeLayoutError::IllegalFormatValue{ value }),
        }
    }
}

impl From<AttributeLayout> for vk::Format {
    fn from(value: AttributeLayout) -> Self {
        match value {
            AttributeLayout::Float2 => vk::Format::R32G32_SFLOAT,
            AttributeLayout::Float3 => vk::Format::R32G32B32_SFLOAT,
        }
    }
}



/// Defines how vertices will be read from the buffer (specifically, direct or instanced)
#[derive(Clone, Copy, Debug)]
pub enum VertexInputRate {
    /// Input the vertices as-is
    Vertex,
    /// Render instance-based
    Instance,
}

impl From<vk::VertexInputRate> for VertexInputRate {
    #[inline]
    fn from(value: vk::VertexInputRate) -> Self {
        match value {
            vk::VertexInputRate::VERTEX   => VertexInputRate::Vertex,
            vk::VertexInputRate::INSTANCE => VertexInputRate::Instance,
            value                         => { panic!("Encountered illegal VkVertexInputRate value '{}'", value.as_raw()); }
        }
    }
}

impl From<VertexInputRate> for vk::VertexInputRate {
    #[inline]
    fn from(value: VertexInputRate) -> Self {
        match value {
            VertexInputRate::Vertex   => vk::VertexInputRate::VERTEX,
            VertexInputRate::Instance => vk::VertexInputRate::INSTANCE,
        }
    }
}



/// Defines the possible topologies for input vertices.
#[derive(Clone, Copy, Debug)]
pub enum VertexTopology {
    /// The input vertices each define separate points
    PointList,

    /// The input vertices define a list of separate lines.
    /// 
    /// Concretely, every consecutive set of two vertices define a line.
    LineList,
    /// The input vertices define a list of consecutive lines.
    /// 
    /// Concretely, the first consecutive set of two vertices defines a line. Then, every consecutive new vertex defines a line with the previous vertex.
    LineStrip,
    /// The input vertices define a list of separate lines with adjacent points.
    /// 
    /// Concretely, every consecutive set of four vertices define a line, drawn between the second and third vertex. The other two are not drawn, but only accessible in the geometry shader.
    LineListAdjacency,
    /// The input vertices define a list of consecutive lines with adjacent points.
    /// 
    /// Concretely, the very first vertex is skipped. The subsequent consecutive set of two vertices defines a line. Then, every consecutive new vertex defines a line with the previous vertex, except for the last vertex. That and the first vertex are only accessible in the geometry shader.
    LineStripAdjacency,

    /// The input vertices define a list of separate triangles.
    /// 
    /// Concretely, every consecutive set of three vertices define a triangle.
    TriangleList,
    /// The input vertices define a list of triangles that share edges.
    /// 
    /// Concretely, the first consecutive set of three vertices defines a triangle. Then, every consecutive new vertex defines a triangle with the previous two vertices.
    TriangleStrip,
    /// The input vertices define a list of triangles that share a common origin vertex.
    /// 
    /// Concretely, the first consecutive set of three vertices defines a triangle. Then, every consecutive set of two vertices defines a triangle with the first vertex in the list.
    /// 
    /// Note that this mode might do funky with some sort of portability mode (see https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans)
    TriangleFan,
    /// The input vertices define a list of separate triangles with adjacent points.
    /// 
    /// Concretely, every consecutive set of six vertices define a triangle, drawn between the first, third and fifth vertex. The other three are not drawn, but only accessible in the geometry shader.
    TriangleListAdjacency,
    /// The input vertices define a list of triangles that share edges.
    /// 
    /// Concretely, the first consecutive set of five vertices defines a triangle, drawn between the first, third and fifth vertex. Then, every consecutive set of two vertices defines a triangle with the the second of those vertices and the previous two (drawn) vertices. The other vertices are not drawn, but only accessible in the geometry shader.
    TriangleStripAdjacency,

    /// The input vertices define no particular shape.
    /// 
    /// Concretely, the vertices are treated to belong to the same shape, and will not be send to vertex post-processing. Instead, they should be used in tessellation to generate renderable primitives.
    PatchList,
}

impl From<vk::PrimitiveTopology> for VertexTopology {
    #[inline]
    fn from(value: vk::PrimitiveTopology) -> Self {
        match value {
            vk::PrimitiveTopology::POINT_LIST => VertexTopology::PointList,

            vk::PrimitiveTopology::LINE_LIST                 => VertexTopology::LineList,
            vk::PrimitiveTopology::LINE_STRIP                => VertexTopology::LineStrip,
            vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY  => VertexTopology::LineListAdjacency,
            vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY => VertexTopology::LineStripAdjacency,

            vk::PrimitiveTopology::TRIANGLE_LIST                 => VertexTopology::TriangleList,
            vk::PrimitiveTopology::TRIANGLE_STRIP                => VertexTopology::TriangleStrip,
            vk::PrimitiveTopology::TRIANGLE_FAN                  => VertexTopology::TriangleFan,
            vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY  => VertexTopology::TriangleListAdjacency,
            vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY => VertexTopology::TriangleStripAdjacency,

            vk::PrimitiveTopology::PATCH_LIST => VertexTopology::PatchList,

            value => { panic!("Encountered illegal VkPrimitiveTopology value '{}'", value.as_raw()); }
        }
    }
}

impl From<VertexTopology> for vk::PrimitiveTopology {
    #[inline]
    fn from(value: VertexTopology) -> Self {
        match value {
            VertexTopology::PointList => vk::PrimitiveTopology::POINT_LIST,

            VertexTopology::LineList           => vk::PrimitiveTopology::LINE_LIST,
            VertexTopology::LineStrip          => vk::PrimitiveTopology::LINE_STRIP,
            VertexTopology::LineListAdjacency  => vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY,
            VertexTopology::LineStripAdjacency => vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY,

            VertexTopology::TriangleList           => vk::PrimitiveTopology::TRIANGLE_LIST,
            VertexTopology::TriangleStrip          => vk::PrimitiveTopology::TRIANGLE_STRIP,
            VertexTopology::TriangleFan            => vk::PrimitiveTopology::TRIANGLE_FAN,
            VertexTopology::TriangleListAdjacency  => vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY,
            VertexTopology::TriangleStripAdjacency => vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY,

            VertexTopology::PatchList => vk::PrimitiveTopology::PATCH_LIST,
        }
    }
}



/// Defines the possible culling modes (i.e., how to discard vertices based on their winding order).
#[derive(Clone, Copy, Debug)]
pub enum CullMode {
    /// Cull vertices that we see from both the front and the back (lol)
    FrontAndBack,
    /// Only cull vertices facing us
    Front,
    /// Only cull vertices facing away from us
    Back,
    /// Do not cull any vertices
    None,
}

impl From<vk::CullModeFlags> for CullMode {
    #[inline]
    fn from(value: vk::CullModeFlags) -> Self {
        match value {
            vk::CullModeFlags::FRONT_AND_BACK => CullMode::FrontAndBack,
            vk::CullModeFlags::FRONT          => CullMode::Front,
            vk::CullModeFlags::BACK           => CullMode::Back,
            vk::CullModeFlags::NONE           => CullMode::None,
            value                             => { panic!("Encountered illegal VkCullModeFlags value '{}'", value.as_raw()); }
        }
    }
}

impl From<CullMode> for vk::CullModeFlags {
    #[inline]
    fn from(value: CullMode) -> Self {
        match value {
            CullMode::FrontAndBack => vk::CullModeFlags::FRONT_AND_BACK,
            CullMode::Front        => vk::CullModeFlags::FRONT,
            CullMode::Back         => vk::CullModeFlags::BACK,
            CullMode::None         => vk::CullModeFlags::NONE,
        }
    }
}



/// Defines which winding direction we consider to be 'front'
#[derive(Clone, Copy, Debug)]
pub enum FrontFace {
    /// The clockwise-winded triangles are 'front'
    Clockwise,
    /// The counter-clockwise-winded triangles are 'front'
    CounterClockwise,
}

impl From<vk::FrontFace> for FrontFace {
    #[inline]
    fn from(value: vk::FrontFace) -> Self {
        match value {
            vk::FrontFace::CLOCKWISE         => FrontFace::Clockwise,
            vk::FrontFace::COUNTER_CLOCKWISE => FrontFace::CounterClockwise,
            value                            => { panic!("Encountered illegal VkFrontFace value '{}'", value.as_raw()); }
        }
    }
}

impl From<FrontFace> for vk::FrontFace {
    #[inline]
    fn from(value: FrontFace) -> Self {
        match value {
            FrontFace::Clockwise        => vk::FrontFace::CLOCKWISE,
            FrontFace::CounterClockwise => vk::FrontFace::COUNTER_CLOCKWISE,
        }
    }
}



/// Defines how to draw in-between the vertices
#[derive(Clone, Copy, Debug)]
pub enum DrawMode {
    /// Only draw the points of the primitive shape
    Point,
    /// Only draws the countours of the primitive shape
    Line,
    /// Fills the entire shape
    Fill,
}

impl From<vk::PolygonMode> for DrawMode {
    #[inline]
    fn from(value: vk::PolygonMode) -> Self {
        match value {
            vk::PolygonMode::POINT => DrawMode::Point,
            vk::PolygonMode::LINE  => DrawMode::Line,
            vk::PolygonMode::FILL  => DrawMode::Fill,
            value                  => { panic!("Encountered illegal VkPolygonMode value '{}'", value.as_raw()); }
        }
    }
}

impl From<DrawMode> for vk::PolygonMode {
    #[inline]
    fn from(value: DrawMode) -> vk::PolygonMode {
        match value {
            DrawMode::Point => vk::PolygonMode::POINT,
            DrawMode::Line  => vk::PolygonMode::LINE,
            DrawMode::Fill  => vk::PolygonMode::FILL,
        }
    }
}



/// Defines a possible number of samples.
#[derive(Clone, Copy, Debug)]
pub enum SampleCount {
    /// Only one sample
    One,
    /// Take two samples
    Two,
    /// Take four samples
    Four,
    /// Take eight samples
    Eight,
    /// Now we're getting somewhere: sixteen samples
    Sixteen,
    /// _Hardcore_: thirty-two samples!
    ThirtyTwo,
    /// What?! Sixty-four whole samples?! :0
    SixtyFour,
}

impl From<vk::SampleCountFlags> for SampleCount {
    #[inline]
    fn from(value: vk::SampleCountFlags) -> Self {
        match value {
            vk::SampleCountFlags::TYPE_1  => SampleCount::One,
            vk::SampleCountFlags::TYPE_2  => SampleCount::Two,
            vk::SampleCountFlags::TYPE_4  => SampleCount::Four,
            vk::SampleCountFlags::TYPE_8  => SampleCount::Eight,
            vk::SampleCountFlags::TYPE_16 => SampleCount::Sixteen,
            vk::SampleCountFlags::TYPE_32 => SampleCount::ThirtyTwo,
            vk::SampleCountFlags::TYPE_64 => SampleCount::SixtyFour,

            value => { panic!("Encountered illegal VkSampleCountFlags value '{}'", value.as_raw()); }
        }
    }
}

impl From<SampleCount> for vk::SampleCountFlags {
    #[inline]
    fn from(value: SampleCount) -> Self {
        match value {
            SampleCount::One       => vk::SampleCountFlags::TYPE_1,
            SampleCount::Two       => vk::SampleCountFlags::TYPE_2,
            SampleCount::Four      => vk::SampleCountFlags::TYPE_4,
            SampleCount::Eight     => vk::SampleCountFlags::TYPE_8,
            SampleCount::Sixteen   => vk::SampleCountFlags::TYPE_16,
            SampleCount::ThirtyTwo => vk::SampleCountFlags::TYPE_32,
            SampleCount::SixtyFour => vk::SampleCountFlags::TYPE_64,
        }
    }
}



/// Defines possible operations for stencils.
#[derive(Clone, Copy, Debug)]
pub enum StencilOp {
    /// Keeps the fragment (or something else)
    Keep,
    /// Sets its value to 0
    Zero,
    /// Replaces the fragment with another value
    Replace,
    /// Inverts the value of the fragment bitwise
    Invert,

    /// Increments the value and clamps it to the maximum representable value
    IncrementClamp,
    /// Decrements the value and clamps it to 0
    DecrementClamp,

    /// Increments the value and wraps it around the maximum representable value back to 0
    IncrementWrap,
    /// Decrements the value and wraps it around 0 back to the maximum representable value
    DecrementWrap,
}

impl From<vk::StencilOp> for StencilOp {
    #[inline]
    fn from(value: vk::StencilOp) -> Self {
        match value {
            vk::StencilOp::KEEP    => StencilOp::Keep,
            vk::StencilOp::ZERO    => StencilOp::Zero,
            vk::StencilOp::REPLACE => StencilOp::Replace,
            vk::StencilOp::INVERT  => StencilOp::Invert,

            vk::StencilOp::INCREMENT_AND_CLAMP => StencilOp::IncrementClamp,
            vk::StencilOp::DECREMENT_AND_CLAMP => StencilOp::DecrementClamp,

            vk::StencilOp::INCREMENT_AND_WRAP => StencilOp::IncrementWrap,
            vk::StencilOp::DECREMENT_AND_WRAP => StencilOp::DecrementWrap,

            value => { panic!("Encountered illegal VkStencilOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<StencilOp> for vk::StencilOp {
    #[inline]
    fn from(value: StencilOp) -> Self {
        match value {
            StencilOp::Keep    => vk::StencilOp::KEEP,
            StencilOp::Zero    => vk::StencilOp::ZERO,
            StencilOp::Replace => vk::StencilOp::REPLACE,
            StencilOp::Invert  => vk::StencilOp::INVERT,

            StencilOp::IncrementClamp => vk::StencilOp::INCREMENT_AND_CLAMP,
            StencilOp::DecrementClamp => vk::StencilOp::DECREMENT_AND_CLAMP,

            StencilOp::IncrementWrap => vk::StencilOp::INCREMENT_AND_WRAP,
            StencilOp::DecrementWrap => vk::StencilOp::DECREMENT_AND_WRAP,
        }
    }
}



/// Defines possible comparison operations.
#[derive(Clone, Copy, Debug)]
pub enum CompareOp {
    /// The comparison always succeeds
    Always,
    /// The comparison never succeeds (always fails)
    Never,

    /// The comparison succeeds iff A < B
    Less,
    /// The comparison succeeds iff A <= B
    LessEq,
    /// The comparison succeeds iff A > B
    Greater,
    /// The comparison succeeds iff A >= B
    GreaterEq,
    /// The comparison succeeds iff A == B
    Equal,
    /// The comparison succeeds iff A != B
    NotEqual,
}

impl From<vk::CompareOp> for CompareOp {
    #[inline]
    fn from(value: vk::CompareOp) -> Self {
        match value {
            vk::CompareOp::ALWAYS => CompareOp::Always,
            vk::CompareOp::NEVER  => CompareOp::Never,

            vk::CompareOp::LESS             => CompareOp::Less,
            vk::CompareOp::LESS_OR_EQUAL    => CompareOp::LessEq,
            vk::CompareOp::GREATER          => CompareOp::Greater,
            vk::CompareOp::GREATER_OR_EQUAL => CompareOp::GreaterEq,
            vk::CompareOp::EQUAL            => CompareOp::Equal,
            vk::CompareOp::NOT_EQUAL        => CompareOp::NotEqual,

            value => { panic!("Encountered illegal VkCompareOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<CompareOp> for vk::CompareOp {
    #[inline]
    fn from(value: CompareOp) -> Self {
        match value {
            CompareOp::Always => vk::CompareOp::ALWAYS,
            CompareOp::Never  => vk::CompareOp::NEVER,

            CompareOp::Less      => vk::CompareOp::LESS,
            CompareOp::LessEq    => vk::CompareOp::LESS_OR_EQUAL,
            CompareOp::Greater   => vk::CompareOp::GREATER,
            CompareOp::GreaterEq => vk::CompareOp::GREATER_OR_EQUAL,
            CompareOp::Equal     => vk::CompareOp::EQUAL,
            CompareOp::NotEqual  => vk::CompareOp::NOT_EQUAL,
        }
    }
}



/// Defines logic operations to perform.
#[derive(Clone, Copy, Debug)]
pub enum LogicOp {
    /// Leaves the destination as-is (`d = d`)
    NoOp,
    /// Set the bits of the destination to 0 (`d = 0`)
    Clear,
    /// Set the bits of the destination to 1 (`d = ~0`)
    Set,
    /// Copies the bits of the source to the destination (`d = s`)
    Copy,
    /// Copies the bits of the source after negating them (`d = ~s`)
    CopyInv,

    /// Negates the destination (`d = ~d`)
    Not,

    /// Performs a bitwise-AND (`d = s & d`)
    And,
    /// Performs a bitwise-AND, negating the source (`d = ~s & d`)
    AndInv,
    /// Performs a bitwise-AND, negating the destination (`d = s & ~d`)
    AndRev,
    /// Performs a negated bitwise-AND (`d = ~(s & d)`)
    NAnd,

    /// Performs a bitwise-XOR (`d = s ^ d`)
    Xor,
    /// Performs a negated bitwise-XOR (`d = ~(s ^ d)`)
    NXor,

    /// Performs a bitwise-OR (`d = s | d`)
    Or,
    /// Performs a bitwise-OR, negating the source (`d = ~s | d`)
    OrInv,
    /// Performs a bitwise-OR, negating the destination (`d = s | ~d`)
    OrRev,
    /// Performs a negated bitwise-OR (`d = ~(s | d)`)
    NOr,
}

impl From<vk::LogicOp> for LogicOp {
    #[inline]
    fn from(value: vk::LogicOp) -> Self {
        match value {
            vk::LogicOp::NO_OP         => LogicOp::NoOp,
            vk::LogicOp::CLEAR         => LogicOp::Clear,
            vk::LogicOp::SET           => LogicOp::Set,
            vk::LogicOp::COPY          => LogicOp::Copy,
            vk::LogicOp::COPY_INVERTED => LogicOp::CopyInv,

            vk::LogicOp::INVERT => LogicOp::Not,

            vk::LogicOp::AND          => LogicOp::And,
            vk::LogicOp::AND_INVERTED => LogicOp::AndInv,
            vk::LogicOp::AND_REVERSE  => LogicOp::AndRev,
            vk::LogicOp::NAND         => LogicOp::NAnd,

            vk::LogicOp::XOR        => LogicOp::Xor,
            vk::LogicOp::EQUIVALENT => LogicOp::NXor,

            vk::LogicOp::OR          => LogicOp::Or,
            vk::LogicOp::OR_INVERTED => LogicOp::OrInv,
            vk::LogicOp::OR_REVERSE  => LogicOp::OrRev,
            vk::LogicOp::NOR         => LogicOp::NOr,

            _ => { panic!("Encountered illegal VkLogicOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<LogicOp> for vk::LogicOp {
    #[inline]
    fn from(value: LogicOp) -> Self {
        match value {
            LogicOp::NoOp    => vk::LogicOp::NO_OP,
            LogicOp::Clear   => vk::LogicOp::CLEAR,
            LogicOp::Set     => vk::LogicOp::SET,
            LogicOp::Copy    => vk::LogicOp::COPY,
            LogicOp::CopyInv => vk::LogicOp::COPY_INVERTED,

            LogicOp::Not => vk::LogicOp::INVERT,

            LogicOp::And    => vk::LogicOp::AND,
            LogicOp::AndInv => vk::LogicOp::AND_INVERTED,
            LogicOp::AndRev => vk::LogicOp::AND_REVERSE,
            LogicOp::NAnd   => vk::LogicOp::NAND,

            LogicOp::Xor  => vk::LogicOp::XOR,
            LogicOp::NXor => vk::LogicOp::EQUIVALENT,

            LogicOp::Or    => vk::LogicOp::OR,
            LogicOp::OrInv => vk::LogicOp::OR_INVERTED,
            LogicOp::OrRev => vk::LogicOp::OR_REVERSE,
            LogicOp::NOr   => vk::LogicOp::NOR,
        }
    }
}



/// Defines the factor of some value to take in a blending operation.
#[derive(Clone, Copy, Debug)]
pub enum BlendFactor {
    /// Use none of the colour (`(0.0, 0.0, 0.0, 0.0)`)
    Zero,
    /// Use all of the colour (`(1.0, 1.0, 1.0, 1.0)`)
    One,

    /// Use the source colour as the factor in blending (`(Rs, Gs, Bs, As)`)
    SrcColour,
    /// Use one minus the source colour as the factor in blending (`(1.0 - Rs, 1.0 - Gs, 1.0 - Bs, 1.0 - As)`)
    OneMinusSrcColour,
    /// Use the destination colour as the factor in blending (`(Rd, Gd, Bd, Ad)`)
    DstColour,
    /// Use one minus the destination colour as the factor in blending (`(1.0 - Rd, 1.0 - Gd, 1.0 - Bd, 1.0 - Ad)`)
    OneMinusDstColour,

    /// Use the source alpha as the factor in blending (`(As, As, As, As)`)
    SrcAlpha,
    /// Use one minus the source alpha as the factor in blending (`(1.0 - As, 1.0 - As, 1.0 - As, 1.0 - As)`)
    OneMinusSrcAlpha,
    /// Use the destination alpha as the factor in blending (`(Ad, Ad, Ad, Ad)`)
    DstAlpha,
    /// Use one minus the destination alpha as the factor in blending (`(1.0 - Ad, 1.0 - Ad, 1.0 - Ad, 1.0 - Ad)`)
    OneMinusDstAlpha,

    /// Use the constant factors given in the ColourBlendState as the factors (`(Fr, Fg, Fb, Fa)`)
    ConstColour,
    /// Use one minus the constant factors given in the ColourBlendState as the factors (`(1.0 - Fr, 1.0 - Fg, 1.0 - Fb, 1.0 - Fa)`)
    OneMinusConstColour,
    /// Use the constant alpha factor given in the ColourBlendState as the factors (`(Fa, Fa, Fa, Fa)`)
    ConstAlpha,
    /// Use one minus the constant alpha factor given in the ColourBlendState as the factors (`(1.0 - Fa, 1.0 - Fa, 1.0 - Fa, 1.0 - Fa)`)
    OneMinusConstAlpha,

    /// When using double source channels, use the colour of the second channel (`(Rs2, Gs2, Bs2, As2)`).
    SrcColour2,
    /// When using double source channels, use one minus the colour of the second channel (`(1.0 - Rs2, 1.0 - Gs2, 1.0 - Bs2, 1.0 - As2)`).
    OneMinusSrcColour2,
    /// When using double source channels, use the alpha of the second channel (`(As2, As2, As2, As2)`).
    SrcAlpha2,
    /// When using double source channels, use one minus the alpha of the second channel (`(1.0 - As2, 1.0 - As2, 1.0 - As2, 1.0 - As2)`).
    OneMinusSrcAlpha2,

    /// Saturates the colour according to the alpha channel (`(min(As, 1.0 - Ad), min(As, 1.0 - Ad), min(As, 1.0 - Ad), 1.0)`)
    SrcAlphaSaturate,
}

impl From<vk::BlendFactor> for BlendFactor {
    #[inline]
    fn from(value: vk::BlendFactor) -> Self {
        match value {
            vk::BlendFactor::ZERO => BlendFactor::Zero,
            vk::BlendFactor::ONE  => BlendFactor::One,

            vk::BlendFactor::SRC_COLOR           => BlendFactor::SrcColour,
            vk::BlendFactor::ONE_MINUS_SRC_COLOR => BlendFactor::OneMinusSrcColour,
            vk::BlendFactor::DST_COLOR           => BlendFactor::DstColour,
            vk::BlendFactor::ONE_MINUS_DST_COLOR => BlendFactor::OneMinusDstColour,

            vk::BlendFactor::SRC_ALPHA           => BlendFactor::SrcAlpha,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA => BlendFactor::OneMinusSrcAlpha,
            vk::BlendFactor::DST_ALPHA           => BlendFactor::DstAlpha,
            vk::BlendFactor::ONE_MINUS_DST_ALPHA => BlendFactor::OneMinusDstAlpha,

            vk::BlendFactor::CONSTANT_COLOR           => BlendFactor::ConstColour,
            vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR => BlendFactor::OneMinusConstColour,
            vk::BlendFactor::CONSTANT_ALPHA           => BlendFactor::ConstAlpha,
            vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA => BlendFactor::OneMinusConstAlpha,

            vk::BlendFactor::SRC1_COLOR           => BlendFactor::SrcColour2,
            vk::BlendFactor::ONE_MINUS_SRC1_COLOR => BlendFactor::OneMinusSrcColour2,
            vk::BlendFactor::SRC1_ALPHA           => BlendFactor::SrcAlpha2,
            vk::BlendFactor::ONE_MINUS_SRC1_ALPHA => BlendFactor::OneMinusSrcAlpha2,

            vk::BlendFactor::SRC_ALPHA_SATURATE => BlendFactor::SrcAlphaSaturate,

            value => { panic!("Encountered illegal VkBlendFactor value '{}'", value.as_raw()); }
        }
    }
}

impl From<BlendFactor> for vk::BlendFactor {
    #[inline]
    fn from(value: BlendFactor) -> Self {
        match value {
            BlendFactor::Zero => vk::BlendFactor::ZERO,
            BlendFactor::One  => vk::BlendFactor::ONE,

            BlendFactor::SrcColour         => vk::BlendFactor::SRC_COLOR,
            BlendFactor::OneMinusSrcColour => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
            BlendFactor::DstColour         => vk::BlendFactor::DST_COLOR,
            BlendFactor::OneMinusDstColour => vk::BlendFactor::ONE_MINUS_DST_COLOR,

            BlendFactor::SrcAlpha         => vk::BlendFactor::SRC_ALPHA,
            BlendFactor::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            BlendFactor::DstAlpha         => vk::BlendFactor::DST_ALPHA,
            BlendFactor::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,

            BlendFactor::ConstColour         => vk::BlendFactor::CONSTANT_COLOR,
            BlendFactor::OneMinusConstColour => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstAlpha          => vk::BlendFactor::CONSTANT_ALPHA,
            BlendFactor::OneMinusConstAlpha  => vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,

            BlendFactor::SrcColour2         => vk::BlendFactor::SRC1_COLOR,
            BlendFactor::OneMinusSrcColour2 => vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
            BlendFactor::SrcAlpha2          => vk::BlendFactor::SRC1_ALPHA,
            BlendFactor::OneMinusSrcAlpha2  => vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,

            BlendFactor::SrcAlphaSaturate => vk::BlendFactor::SRC_ALPHA_SATURATE,
        }
    }
}



/// Defines blend operations to perform.
#[derive(Clone, Copy, Debug)]
pub enum BlendOp {
    /// Add the proper fractions of the colours together:
    /// ```math
    /// Rd = Rs * FCs + Rd * FCd
    /// Gd = Gs * FCs + Gd * FCd
    /// Bd = Bs * FCs + Bd * FCd
    /// Ad = As * FAs + Ad * FAd
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    Add,
    /// Subtract the proper fractions of the colours from each other:
    /// ```math
    /// Rd = Rs * FCs - Rd * FCd
    /// Gd = Gs * FCs - Gd * FCd
    /// Bd = Bs * FCs - Bd * FCd
    /// Ad = As * FAs - Ad * FAd
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    Sub,
    /// Subtract the proper fractions of the colours from each other:
    /// ```math
    /// Rd = Rd * FCd - Rs * FCs
    /// Gd = Gd * FCd - Gs * FCs
    /// Bd = Bd * FCd - Bs * FCs
    /// Ad = Ad * FAd - As * FAs
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    SubRev,

    /// Take the minimal value of the colours (ignoring fractions):
    /// ```math
    /// Rd = min(Rs, Rd)
    /// Gd = min(Gs, Gd)
    /// Bd = min(Bs, Bd)
    /// Ad = min(As, Ad)
    /// ```
    /// (`Xs` is the source channel and `Xd` is the destination channel)
    Min,
    /// Take the maximum value of the colours (ignoring fractions):
    /// ```math
    /// Rd = max(Rs, Rd)
    /// Gd = max(Gs, Gd)
    /// Bd = max(Bs, Bd)
    /// Ad = max(As, Ad)
    /// ```
    /// (`Xs` is the source channel and `Xd` is the destination channel)
    Max,
}

impl From<vk::BlendOp> for BlendOp {
    #[inline]
    fn from(value: vk::BlendOp) -> Self {
        match value {
            vk::BlendOp::ADD              => BlendOp::Add,
            vk::BlendOp::SUBTRACT         => BlendOp::Sub,
            vk::BlendOp::REVERSE_SUBTRACT => BlendOp::SubRev,

            vk::BlendOp::MIN => BlendOp::Min,
            vk::BlendOp::MAX => BlendOp::Max,

            value => { panic!("Encountered illegal VkBlendOp value '{}'", value.as_raw()); }
        }
    }
}

impl From<BlendOp> for vk::BlendOp {
    #[inline]
    fn from(value: BlendOp) -> Self {
        match value {
            BlendOp::Add    => vk::BlendOp::ADD,
            BlendOp::Sub    => vk::BlendOp::SUBTRACT,
            BlendOp::SubRev => vk::BlendOp::REVERSE_SUBTRACT,

            BlendOp::Min => vk::BlendOp::MIN,
            BlendOp::Max => vk::BlendOp::MAX,
        }
    }
}
