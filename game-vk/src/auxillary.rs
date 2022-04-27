/* AUXILLARY.rs
 *   by Lut99
 *
 * Created:
 *   18 Apr 2022, 12:27:51
 * Last edited:
 *   27 Apr 2022, 12:28:02
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains auxillary wrapped structs around Vulkan structs, to not
 *   expose any ash to the outside world.
**/

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FResult};
use std::ops::{BitOr, BitOrAssign, Range};
use std::ptr;
use std::slice;
use std::sync::Arc;

use ash::vk;

pub use crate::errors::{AttributeLayoutError, QueueError};
use crate::instance::Instance;


/***** GEOMETRY *****/
/// Defines a 2-dimensional offset with data type T.
#[derive(Clone, Debug)]
pub struct Offset2D<T> {
    /// The X-coordinate of the offset.
    pub x : T,
    /// The Y-coordinate of the offset.
    pub y : T,
}

impl<T> Offset2D<T> {
    /// Constructor for the Offset2D.
    /// 
    /// # Generic arguments
    /// - `T`: The data type of the coordinates.
    /// 
    /// # Arguments
    /// - `x`: The X-coordinate of the offset.
    /// - `y`: The Y-coordinate of the offset.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<vk::Offset2D> for Offset2D<T>
where
    T: From<i32>
{
    #[inline]
    fn from(value: vk::Offset2D) -> Self {
        Self {
            x : T::from(value.x),
            y : T::from(value.y),
        }
    }
}

impl<T> From<Offset2D<T>> for vk::Offset2D
where
    T: Into<i32>
{
    #[inline]
    fn from(value: Offset2D<T>) -> Self {
        Self {
            x : value.x.into(),
            y : value.y.into(),
        }
    }
}



/// Defines a 2-dimensional extent with data type T.
#[derive(Clone, Debug)]
pub struct Extent2D<T> {
    /// The width of the extent.
    pub w : T,
    /// The height of the extent.
    pub h : T,
}

impl<T> Extent2D<T> {
    /// Constructor for the Extent2D.
    /// 
    /// # Generic arguments
    /// - `T`: The data type of the dimensions.
    /// 
    /// # Arguments
    /// - `w`: The width of the extent.
    /// - `h`: The height of the extent.
    #[inline]
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}

impl<T> From<vk::Extent2D> for Extent2D<T>
where
    T: From<u32>
{
    #[inline]
    fn from(value: vk::Extent2D) -> Self {
        Self {
            w : T::from(value.width),
            h : T::from(value.height),
        }
    }
}

impl<T> From<Extent2D<T>> for vk::Extent2D
where
    T: Into<u32>
{
    #[inline]
    fn from(value: Extent2D<T>) -> Self {
        Self {
            width  : value.w.into(),
            height : value.h.into(),
        }
    }
}



/// Defines a 2-dimensional rectangle with an offset (of datatype T) and an extent (of datatype U).
#[derive(Clone, Debug)]
pub struct Rect2D<T, U = T> {
    /// The offset of the top-left corner of the rectangle.
    pub offset : Offset2D<T>,
    /// The extent of rectangle.
    pub extent : Extent2D<U>,
}

impl<T, U> Rect2D<T, U> {
    /// Constructor for the Rect2D.
    /// 
    /// # Generic arguments
    /// - `T`: The data type of the offset.
    /// - `U`: The data type of the extent.
    /// 
    /// # Arguments
    /// - `x`: The X-coordinate of the offset.
    /// - `y`: The Y-coordinate of the offset.
    /// - `w`: The width of the extent.
    /// - `h`: The height of the extent.
    #[inline]
    pub fn new(x: T, y: T, w: U, h: U) -> Self {
        Self {
            offset : Offset2D::new(x, y),
            extent : Extent2D::new(w, h),
        }
    }

    /// Constructor for the Rect2D that takes a separate offset and extend.
    /// 
    /// # Generic arguments
    /// - `T`: The data type of the offset.
    /// - `U`: The data type of the extent.
    /// 
    /// # Arguments
    /// - `offset`: The offset of the rectangle.
    /// - `extent`: The extent of the rectangle.
    #[inline]
    pub fn from_raw(offset: Offset2D<T>, extent: Extent2D<U>) -> Self {
        Self {
            offset,
            extent,
        }
    }



    /// Returns the X-coordinate of the rectangle's offset.
    #[inline]
    pub fn x(&self) -> T where T: Copy { self.offset.x }

    /// Returns the Y-coordinate of the rectangle's offset.
    #[inline]
    pub fn y(&self) -> T where T: Copy { self.offset.y }

    /// Returns the width of the rectangle's extent.
    #[inline]
    pub fn w(&self) -> U where U: Copy { self.extent.w }

    /// Returns the height of the rectangle's extent.
    #[inline]
    pub fn h(&self) -> U where U: Copy { self.extent.h }
}

impl<T, U> From<vk::Rect2D> for Rect2D<T, U>
where
    T: From<i32>,
    U: From<u32>,
{
    #[inline]
    fn from(value: vk::Rect2D) -> Self {
        Self {
            offset : value.offset.into(),
            extent : value.extent.into(),
        }
    }
}

impl<T, U> From<Rect2D<T, U>> for vk::Rect2D
where
    T: Into<i32>,
    U: Into<u32>,
{
    #[inline]
    fn from(value: Rect2D<T, U>) -> Self {
        Self {
            offset : value.offset.into(),
            extent : value.extent.into(),
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





/***** QUEUES *****/
/// Contains information about the queue families for an instantiated GPU.
#[derive(Debug)]
pub struct QueueFamilyInfo {
    /// The index of the queue we're going to use for graphics operations
    pub graphics : u32,
    /// The index of the queue we're going to use for memory operations
    pub memory   : u32,
    /// The index of the queue we're going to use for compute operations
    pub compute  : u32,
}

impl QueueFamilyInfo {
    /// Constructor for the QueueFamilyInfo.
    /// 
    /// Maps the queue families of the given PhysicalDevice to their usage. Will try to use as many different queue families as possible.
    /// 
    /// # Arguments
    /// - `instance`: A reference to an Instance pointer used to query the properties of a physical device.
    /// - `physical_device_index`: The index of the physical device we are trying to get info from. Only used for debugging purposes.
    /// - `physical_device_name`: The name of the physical device we are trying to get info from. Only used for debugging purposes.
    /// 
    /// # Returns
    /// The new QueueFamilyInfo struct on success, or else a QueueError::OperationNotSupported error if the given device does not support all required queue family types.
    pub(crate) fn new(instance: &Arc<Instance>, physical_device: vk::PhysicalDevice, physical_device_index: usize, physical_device_name: &str) -> Result<Self, QueueError> {
        // Prepare placeholders for the different queues
        let mut graphics : Option<(u32, usize)> = None;
        let mut memory : Option<(u32, usize)>   = None;
        let mut compute : Option<(u32, usize)>  = None;

        // Iterate over the queue families
        let families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for (i, family) in families.iter().enumerate() {
            // We need at least one queue in each family, obviously
            if family.queue_count == 0 { continue; }

            // Count the number of operations this queue can do
            let mut n_operations = 0;
            let supports_graphics = if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) { n_operations += 1; true } else { false };
            let supports_memory   = if family.queue_flags.contains(vk::QueueFlags::TRANSFER) { n_operations += 1; true } else { false };
            let supports_compute  = if family.queue_flags.contains(vk::QueueFlags::COMPUTE) { n_operations += 1; true } else { false };
            
            // Note the queue on every slot it supports, except we already have a more specialized one
            if supports_graphics && (graphics.is_none() || n_operations < graphics.as_ref().unwrap().1) {
                graphics = Some((i as u32, n_operations));
            }
            if supports_memory && (memory.is_none() || n_operations < memory.as_ref().unwrap().1) {
                memory = Some((i as u32, n_operations));
            }
            if supports_compute && (compute.is_none() || n_operations < compute.as_ref().unwrap().1) {
                compute = Some((i as u32, n_operations));
            }
        }

        // If we didn't find one of the queues, error
        if graphics.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::GRAPHICS });
        }
        if memory.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::TRANSFER });
        }
        if compute.is_none() {
            return Err(QueueError::OperationUnsupported{ index: physical_device_index, name: physical_device_name.to_string(), operation: vk::QueueFlags::COMPUTE });
        }

        // Otherwise, we can populate ourselves!
        Ok(QueueFamilyInfo {
            graphics : graphics.unwrap().0,
            memory   : memory.unwrap().0,
            compute  : compute.unwrap().0,
        })
    }



    /// Returns an iterator over the **different** families in the QueueFamilyInfo.
    #[inline]
    pub fn unique(&self) -> QueueFamilyInfoUniqueIterator {
        QueueFamilyInfoUniqueIterator::new(self)
    }

    /// Returns the number of **different** families in the QueueFamilyInfo.
    pub fn unique_len(&self) -> usize {
        if self.graphics != self.memory && self.graphics != self.compute && self.memory != self.compute {
            3
        } else if self.graphics != self.memory || self.graphics != self.compute || self.memory != self.compute {
            2
        } else {
            1
        }
    }
}



/// Implements an iterator over the unique family indices in the QueueFamilyInfo.
#[derive(Debug)]
pub struct QueueFamilyInfoUniqueIterator<'a> {
    /// The QueueFamilyInfo over which we iterate
    family_info : &'a QueueFamilyInfo,
    /// The current 'position' in the family info
    index       : usize,
}

impl<'a> QueueFamilyInfoUniqueIterator<'a> {
    /// Constructor for the QueueFamilyInfoUniqueIterator.
    /// 
    /// Prepares a new iterator over the given QueueFamilyInfo.
    /// 
    /// Note that it's passed by reference, so it's probably not a good idea to modify queue families while iterating over them.
    #[inline]
    pub(crate) fn new(family_info: &'a QueueFamilyInfo) -> Self {
        Self {
            family_info,
            index : 0,
        }
    }
}

impl<'a> Iterator for QueueFamilyInfoUniqueIterator<'a> {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Match based on the index
        match self.index {
            0 => { self.index += 1; Some(self.family_info.graphics) },
            1 => {
                // Only do this one if it's unique
                self.index += 1;
                if self.family_info.memory != self.family_info.graphics {
                    Some(self.family_info.memory)
                } else {
                    // Skip to the next value
                    self.next()
                }
            },
            2 => {
                // Only do this one if it's unique
                self.index += 1;
                if self.family_info.compute != self.family_info.graphics && self.family_info.compute != self.family_info.memory {
                    Some(self.family_info.compute)
                } else {
                    // Skip to the next value
                    self.next()
                }
            }
            _ => None,
        }
    }
}



/// Central place where we store the queues of the created logical device.
#[derive(Debug)]
pub struct Queues {
    /// The graphics queue
    pub graphics : vk::Queue,
    /// The memory queue
    pub memory   : vk::Queue,
    /// The compute queue
    pub compute  : vk::Queue,
}

impl Queues {
    /// Constructor for the Queues.
    /// 
    /// Requests the three queues from the queue families in the given QueueFamilyInfo on the given vk::Device.
    #[inline]
    pub(crate) fn new(device: &ash::Device, family_info: &QueueFamilyInfo) -> Self {
        Self {
            graphics : unsafe { device.get_device_queue(family_info.graphics, 0) },
            memory   : unsafe { device.get_device_queue(family_info.memory, 0) },
            compute  : unsafe { device.get_device_queue(family_info.compute, 0) },
        }
    }
}





/***** SURFACES *****/
/// Collects information about the SwapchainSupport for this device.
#[derive(Debug)]
pub struct SwapchainSupport {
    /// Lists the capabilities of the chosen device/surface combo.
    pub capabilities  : vk::SurfaceCapabilitiesKHR,
    /// Lists the formats supported by the chosen device/surface combo.
    pub formats       : Vec<vk::SurfaceFormatKHR>,
    /// Lists the present modes supported by the chosen device/surface combo.
    pub present_modes : Vec<vk::PresentModeKHR>,
}




/***** SHADERS *****/
/// The ShaderStage where a shader or a resource lives.
#[derive(Clone, Copy, Debug)]
pub struct ShaderStage(u16);

impl ShaderStage {
    /// A ShaderStage that hits all stages
    pub const ALL: Self   = Self(0xFFFF);
    /// A ShaderStage that hits all graphics stages
    pub const ALL_GRAPHICS: Self   = Self(0x001F);
    /// An empty ShaderStage
    pub const EMPTY: Self = Self(0x0000);

    /// The Vertex stage
    pub const VERTEX: Self                 = Self(0x0001);
    /// The control stage of the Tesselation stage
    pub const TESSELLATION_CONTROL: Self    = Self(0x0002);
    /// The evaluation stage of the Tesselation stage
    pub const TESSELLATION_EVALUATION: Self = Self(0x0004);
    /// The Geometry stage
    pub const GEOMETRY: Self               = Self(0x0008);
    /// The Fragment stage
    pub const FRAGMENT: Self               = Self(0x0010);
    /// The Compute stage
    pub const COMPUTE: Self                = Self(0x0020);


    /// Returns whether the given ShaderStage is a subset of this one.
    /// 
    /// # Arguments
    /// - `value`: The ShaderStage that should be a subset of this one. For example, if value is Self::VERTEX, then returns true if the Vertex shader stage was enabled in this ShaderStage.
    #[inline]
    pub fn check(&self, other: ShaderStage) -> bool { (self.0 & other.0) == other.0 }
}

impl BitOr for ShaderStage {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ShaderStage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl From<vk::ShaderStageFlags> for ShaderStage {
    #[inline]
    fn from(value: vk::ShaderStageFlags) -> Self {
        // Construct it manually for portability
        let mut result = ShaderStage::EMPTY;
        if (value & vk::ShaderStageFlags::VERTEX).as_raw() != 0 { result |= ShaderStage::VERTEX; }
        if (value & vk::ShaderStageFlags::TESSELLATION_CONTROL).as_raw() != 0 { result |= ShaderStage::TESSELLATION_CONTROL; }
        if (value & vk::ShaderStageFlags::TESSELLATION_EVALUATION).as_raw() != 0 { result |= ShaderStage::TESSELLATION_EVALUATION; }
        if (value & vk::ShaderStageFlags::GEOMETRY).as_raw() != 0 { result |= ShaderStage::GEOMETRY; }
        if (value & vk::ShaderStageFlags::FRAGMENT).as_raw() != 0 { result |= ShaderStage::FRAGMENT; }
        if (value & vk::ShaderStageFlags::COMPUTE).as_raw() != 0 { result |= ShaderStage::COMPUTE; }

        // Return it
        result
    }
}

impl From<ShaderStage> for vk::ShaderStageFlags {
    fn from(value: ShaderStage) -> Self {
        // Construct it manually due to private constructors ;(
        let mut result = vk::ShaderStageFlags::empty();
        if value.check(ShaderStage::VERTEX) { result |= vk::ShaderStageFlags::VERTEX; }
        if value.check(ShaderStage::TESSELLATION_CONTROL) { result |= vk::ShaderStageFlags::TESSELLATION_CONTROL; }
        if value.check(ShaderStage::TESSELLATION_EVALUATION) { result |= vk::ShaderStageFlags::TESSELLATION_EVALUATION; }
        if value.check(ShaderStage::GEOMETRY) { result |= vk::ShaderStageFlags::GEOMETRY; }
        if value.check(ShaderStage::FRAGMENT) { result |= vk::ShaderStageFlags::FRAGMENT; }
        if value.check(ShaderStage::COMPUTE) { result |= vk::ShaderStageFlags::COMPUTE; }

        // Return it
        result
    }
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



/// Defines a single binding for the DescriptorSetLayout
#[derive(Clone, Debug)]
pub struct DescriptorBinding {
    /// The binding index of this binding (for use in shaders).
    pub binding : u32,
    /// The type of this binding.
    pub kind    : DescriptorKind,
    /// The shader stage where this binding is bound to.
    pub stage   : ShaderStage,
    /// The number of descriptors in this binding.
    pub count   : u32,
}

impl From<vk::DescriptorSetLayoutBinding> for DescriptorBinding {
    #[inline]
    fn from(value: vk::DescriptorSetLayoutBinding) -> Self {
        // Use the reference one instead
        Self::from(&value)
    }
}

impl From<&vk::DescriptorSetLayoutBinding> for DescriptorBinding {
    #[inline]
    fn from(value: &vk::DescriptorSetLayoutBinding) -> Self {
        Self {
            binding : value.binding,
            kind    : value.descriptor_type.into(),
            stage   : value.stage_flags.into(),
            count   : value.descriptor_count,
        }
    }
}

impl From<DescriptorBinding> for vk::DescriptorSetLayoutBinding {
    #[inline]
    fn from(value: DescriptorBinding) -> Self {
        // Use the reference one instead
        Self::from(&value)
    }
}

impl From<&DescriptorBinding> for vk::DescriptorSetLayoutBinding {
    #[inline]
    fn from(value: &DescriptorBinding) -> Self {
        Self {
            binding              : value.binding,
            descriptor_type      : value.kind.into(),
            stage_flags          : value.stage.into(),
            descriptor_count     : value.count,
            p_immutable_samplers : ptr::null(),
        }
    }
}





/***** PIPELINE *****/
/// Defines the possible layouts for an attribute
#[derive(Clone, Copy, Debug)]
pub enum AttributeLayout {
    /// A three-dimensional vector of 32-bit floating-point numbers
    Float3,
}

impl TryFrom<vk::Format> for AttributeLayout {
    type Error = AttributeLayoutError;

    fn try_from(value: vk::Format) -> Result<Self, Self::Error> {
        match value {
            vk::Format::R32G32B32_SFLOAT => Ok(AttributeLayout::Float3),
            value                        => Err(AttributeLayoutError::IllegalFormatValue{ value }),
        }
    }
}

impl From<AttributeLayout> for vk::Format {
    fn from(value: AttributeLayout) -> Self {
        match value {
            AttributeLayout::Float3 => vk::Format::R32G32B32_SFLOAT,
        }
    }
}



/// Defines how a single attribute (i.e., field in the Vertex struct) looks like.
#[derive(Clone, Debug)]
pub struct VertexAttribute {
    /// The location in the shader of this attribute (must be arbitrary but unique).
    pub location : u32,
    /// The binding (i.e., the Vertex buffer) where this attribute's vertex lives
    pub binding  : u32,
    /// Describes the byte layout of this attribute.
    pub layout   : AttributeLayout,
    /// Notes where to find the attribute in the parent Vertex struct (offset as bytes)
    pub offset   : usize,
}

impl From<vk::VertexInputAttributeDescription> for VertexAttribute {
    #[inline]
    fn from(value: vk::VertexInputAttributeDescription) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&vk::VertexInputAttributeDescription> for VertexAttribute {
    #[inline]
    fn from(value: &vk::VertexInputAttributeDescription) -> Self {
        // Simply populate the VertexAttribute via its struct interface
        Self {
            location : value.location,
            binding  : value.binding,
            layout   : match value.format.try_into() {
                Ok(layout) => layout,
                Err(err)   => { panic!("Illegal attribute format: {}", err); }
            },
            offset   : value.offset as usize,
        }
    }
}

impl From<VertexAttribute> for vk::VertexInputAttributeDescription {
    #[inline]
    fn from(value: VertexAttribute) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&VertexAttribute> for vk::VertexInputAttributeDescription {
    #[inline]
    fn from(value: &VertexAttribute) -> Self {
        // Simply populate the VertexInputAttributeDescription via its struct interface
        Self {
            location : value.location,
            binding  : value.binding,
            format   : value.layout.into(),
            offset   : value.offset as u32,
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



/// Defines how a single binding (i.e., vector to a ) looks like.
#[derive(Clone, Debug)]
pub struct VertexBinding {
    /// The binding index of this buffer
    pub binding : u32,
    /// The stride, i.e., size of each vertex
    pub stride  : usize,
    /// The input rate of the vertices. Concretely, this is either a function of an index or directly reading the vertices from the buffer.
    pub rate    : VertexInputRate,
}

impl From<vk::VertexInputBindingDescription> for VertexBinding {
    #[inline]
    fn from(value: vk::VertexInputBindingDescription) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&vk::VertexInputBindingDescription> for VertexBinding {
    #[inline]
    fn from(value: &vk::VertexInputBindingDescription) -> Self {
        // Simply populate the VertexBinding via its struct interface
        Self {
            binding : value.binding,
            stride  : value.stride as usize,
            rate    : value.input_rate.into(),
        }
    }
}

impl From<VertexBinding> for vk::VertexInputBindingDescription {
    #[inline]
    fn from(value: VertexBinding) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&VertexBinding> for vk::VertexInputBindingDescription {
    #[inline]
    fn from(value: &VertexBinding) -> Self {
        // Simply populate the VertexInputBindingDescription via its struct interface
        Self {
            binding    : value.binding,
            stride     : value.stride as u32,
            input_rate : value.rate.into(),
        }
    }
}



/// Defines the layout of the input vertices given to the pipeline.
#[derive(Clone, Debug)]
pub struct VertexInputState {
    /// A list of attributes (as VertexAttribute) of each incoming vertex.
    pub attributes : Vec<VertexAttribute>,
    /// A list of bindings (as VertexBinding) of the different Vertex buffers.
    pub bindings   : Vec<VertexBinding>,
}

impl From<&vk::PipelineVertexInputStateCreateInfo> for VertexInputState {
    fn from(value: &vk::PipelineVertexInputStateCreateInfo) -> Self {
        // Create the two vectors with copies from the vertex attributes
        let attributes: &[vk::VertexInputAttributeDescription] = unsafe { slice::from_raw_parts(value.p_vertex_attribute_descriptions, value.vertex_attribute_description_count as usize) };
        let bindings: &[vk::VertexInputBindingDescription]     = unsafe { slice::from_raw_parts(value.p_vertex_binding_descriptions, value.vertex_binding_description_count as usize) };

        // Copy the vectors to our structs
        let attributes: Vec<VertexAttribute> = attributes.iter().map(|attr| attr.into()).collect();
        let bindings: Vec<VertexBinding>     = bindings.iter().map(|bind| bind.into()).collect();

        // Return the new instance with these vectors
        Self {
            attributes,
            bindings,
        }
    }
}

impl Into<(vk::PipelineVertexInputStateCreateInfo, (Vec<vk::VertexInputAttributeDescription>, Vec<vk::VertexInputBindingDescription>))> for VertexInputState {
    /// Converts the VertexInputState into a VkPipelineVertexInputStateCreateInfo.
    /// 
    /// However, due to the external references made in the VkPipelineVertexInputStateCreateInfo struct, it also returns two vectors that manage the external memory referenced.
    /// 
    /// # Returns
    /// A tuple with:
    /// - The new VkPipelineVertexInputStateCreateInfo instance
    /// - A tuple with:
    ///   - The vector with the attributes
    ///   - The vector with the bindings
    fn into(self) -> (vk::PipelineVertexInputStateCreateInfo, (Vec<vk::VertexInputAttributeDescription>, Vec<vk::VertexInputBindingDescription>)) {
        // Cast the vectors to their Vulkan counterparts
        let attributes: Vec<vk::VertexInputAttributeDescription> = self.attributes.iter().map(|attr| attr.into()).collect();
        let bindings: Vec<vk::VertexInputBindingDescription>     = self.bindings.iter().map(|bind| bind.into()).collect();

        // Create the new instance with these vectors
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            // Do the standard stuff
            s_type : vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineVertexInputStateCreateFlags::empty(),

            // Add the attributes
            vertex_attribute_description_count : attributes.len() as u32,
            p_vertex_attribute_descriptions    : attributes.as_ptr(),

            // Add the bindings
            vertex_binding_description_count : bindings.len() as u32,
            p_vertex_binding_descriptions    : bindings.as_ptr(),
        };

        // Return the struct with its memory managers
        (vertex_input_state, (attributes, bindings))
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



/// Defines how to construct primitives from the input vertices.
#[derive(Clone, Debug)]
pub struct VertexAssemblyState {
    /// The topology of the input vertices
    pub topology          : VertexTopology,
    /// Whether or not a special vertex value is reserved for resetting a primitive mid-way
    pub restart_primitive : bool,
}

impl From<vk::PipelineInputAssemblyStateCreateInfo> for VertexAssemblyState {
    #[inline]
    fn from(value: vk::PipelineInputAssemblyStateCreateInfo) -> Self {
        // Simply use the default struct constructor
        Self {
            topology          : value.topology.into(),
            restart_primitive : value.primitive_restart_enable != 0,
        }
    }
}

impl From<VertexAssemblyState> for vk::PipelineInputAssemblyStateCreateInfo {
    #[inline]
    fn from(value: VertexAssemblyState) -> Self {
        // Simply use the default struct constructor
        Self {
            // Do the default stuff
            s_type : vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineInputAssemblyStateCreateFlags::empty(),

            // Set the topology and the bool
            topology                 : value.topology.into(),
            primitive_restart_enable : value.restart_primitive as u32,
        }
    }
}



/// Defines the dimensions of a resulting frame.
#[derive(Clone, Debug)]
pub struct ViewportState {
    /// The rectangle that defines the viewport's dimensions.
    /// 
    /// Note that this will actually be ignored if the viewport is given as a dynamic state.
    pub viewport : Rect2D<f32>,
    /// The rectangle that defines any cutoff to the viewport.
    /// 
    /// Note that this will actually be ignored if the scissor is given as a dynamic state.
    pub scissor  : Rect2D<i32, u32>,
    /// The depth range of the Viewport. Anything that falls outside of it will be clipped.
    pub depth    : Range<f32>,
}

impl From<&vk::PipelineViewportStateCreateInfo> for ViewportState {
    #[inline]
    fn from(value: &vk::PipelineViewportStateCreateInfo) -> Self {
        // Make sure the viewport state does not use multiple viewports / scissors
        if value.viewport_count != 1 || value.scissor_count != 1 { panic!("Encountered VkPipelineViewportStateCreateInfo with multiple viewports and/or scissors"); }

        // Fetch the only viewport and scissor
        let viewport: vk::Viewport = unsafe { slice::from_raw_parts(value.p_viewports, 1) }[0];
        let scissor: vk::Rect2D    = unsafe { slice::from_raw_parts(value.p_scissors, 1) }[0];

        // Use the default constructor syntax
        Self {
            viewport : Rect2D::new(viewport.x, viewport.y, viewport.width, viewport.height),
            scissor  : scissor.into(),
            depth    : viewport.min_depth..viewport.max_depth,
        }
    }
}

impl Into<(vk::PipelineViewportStateCreateInfo, (Box<vk::Viewport>, Box<vk::Rect2D>))> for ViewportState {
    /// Converts the Viewport into a VkPipelineViewportStateCreateInfo.
    /// 
    /// However, due to the external references made in the VkPipelineViewportStateCreateInfo struct, it also returns two Boxes that manage the external memory referenced.
    /// 
    /// # Returns
    /// A tuple with:
    /// - The new VkPipelineViewportStateCreateInfo instance
    /// - A tuple with:
    ///   - The Box with the viewport
    ///   - The Box with the scissor
    fn into(self) -> (vk::PipelineViewportStateCreateInfo, (Box<vk::Viewport>, Box<vk::Rect2D>)) {
        // Cast the viewport and scissor to their Vulkan counterparts
        let viewport: Box<vk::Viewport> = Box::new(vk::Viewport {
            x         : self.viewport.x(),
            y         : self.viewport.y(),
            width     : self.viewport.w(),
            height    : self.viewport.h(),
            min_depth : self.depth.start,
            max_depth : self.depth.end,
        });
        let scissor: Box<vk::Rect2D> = Box::new(self.scissor.into());

        // Put the pointers in the new struct to return
        let result = vk::PipelineViewportStateCreateInfo {
            // Set the standard fields
            s_type : vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineViewportStateCreateFlags::empty(),
            
            // Set the only viewport
            viewport_count : 1,
            p_viewports    : &*viewport,

            // Set the only scissor
            scissor_count : 1,
            p_scissors    : &*scissor,
        };

        // Now return the new struct plus its memory manages
        (result, (viewport, scissor))
    }
}

impl From<ViewportState> for vk::Viewport {
    fn from(value: ViewportState) -> Self {
        // Use the default constructor syntax
        Self {
            x         : value.viewport.x(),
            y         : value.viewport.y(),
            width     : value.viewport.w(),
            height    : value.viewport.h(),
            min_depth : value.depth.start,
            max_depth : value.depth.end,
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



/// Defines the fixed rasterization stage for a Pipeline.
#[derive(Clone, Debug)]
pub struct RasterizerState {
    /// Defines the culling mode for the Rasterization stage
    pub cull_mode  : CullMode,
    /// Defines which winding direction we consider to be 'front'
    pub front_face : FrontFace,

    /// Defines the thickness of the lines drawn by the rasterizer. Note, though, that anything larger than a 1.0f requires a GPU feature
    pub line_width : f32,
    /// Defines what to draw in between the vertices
    pub draw_mode  : DrawMode,

    /// Whether or not to discard the fragments after the rasterizer (lol)
    pub discard_result : bool,

    /// Whether to enable depth clamping or not (i.e., clamping objects to a certain depth)
    pub depth_clamp : bool,
    /// The value to clamp the depth to (whether upper or lower depends on testing op used)
    pub clamp_value : f32,

    /// Whether or not to change the depth value before testing and writing
    pub depth_bias : bool,
    /// The factor of depth to apply to the depth of each fragment (i.e., scaling)
    pub depth_factor : f32,
    /// The factor to apply to the slope(?) of the fragment during depth calculation
    pub depth_slope  : f32,
}

impl From<vk::PipelineRasterizationStateCreateInfo> for RasterizerState {
    #[inline]
    fn from(value: vk::PipelineRasterizationStateCreateInfo) -> Self {
        // Simply use the default construction syntax
        Self {
            cull_mode  : value.cull_mode.into(),
            front_face : value.front_face.into(),

            line_width : value.line_width,
            draw_mode  : value.polygon_mode.into(),

            discard_result : value.rasterizer_discard_enable != 0,

            depth_clamp : value.depth_clamp_enable != 0,
            clamp_value : value.depth_bias_clamp,

            depth_bias   : value.depth_bias_enable != 0,
            depth_factor : value.depth_bias_constant_factor,
            depth_slope  : value.depth_bias_slope_factor,
        }
    }
}

impl From<RasterizerState> for vk::PipelineRasterizationStateCreateInfo {
    #[inline]
    fn from(value: RasterizerState) -> Self {
        // Simply use the default construction syntax
        Self {
            // Set the default flags
            s_type : vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineRasterizationStateCreateFlags::empty(),

            // Set the culling mode & associated front face
            cull_mode  : value.cull_mode.into(),
            front_face : value.front_face.into(),
            
            // Set how to draw the fragments
            line_width   : value.line_width,
            polygon_mode : value.draw_mode.into(),

            // Determine whether to keep everything or not (invert that)
            rasterizer_discard_enable : value.discard_result as u32,

            // Set the depth clamp stuff
            depth_clamp_enable : value.depth_clamp as u32,
            depth_bias_clamp   : value.clamp_value,

            // Set the depth bias stuff
            depth_bias_enable          : value.depth_bias as u32,
            depth_bias_constant_factor : value.depth_factor,
            depth_bias_slope_factor    : value.depth_slope,
        }
    }
}



/// Defines if and how to multisample for a Pipeline
#[derive(Clone, Debug)]
pub struct MultisampleState {}

impl From<vk::PipelineMultisampleStateCreateInfo> for MultisampleState {
    #[inline]
    fn from(_value: vk::PipelineMultisampleStateCreateInfo) -> Self {
        Self {}
    }
}

impl From<MultisampleState> for vk::PipelineMultisampleStateCreateInfo {
    #[inline]
    fn from(_value: MultisampleState) -> Self {
        Self {
            // Set the default values
            s_type : vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineMultisampleStateCreateFlags::empty(),
            
            // Set the number of samples
            rasterization_samples : vk::SampleCountFlags::TYPE_1,

            // Set whether to shade the samples
            sample_shading_enable : vk::FALSE,
            min_sample_shading    : 0.0,

            // Set a possible mask for the different samples
            p_sample_mask : ptr::null(),

            // Set some alpha properties for the samples
            alpha_to_one_enable      : vk::FALSE,
            alpha_to_coverage_enable : vk::FALSE,
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



/// Defines how to interact with a given stencil.
#[derive(Clone, Debug)]
pub struct StencilOpState {
    /// Defines what to do if the stencil test fails
    pub on_stencil_fail : StencilOp,
    /// Defines what to do if the depth test fails
    pub on_depth_fail   : StencilOp,
    /// Defines what to do if the stencil test and depth test succeed
    pub on_success      : StencilOp,

    /// Defines the operator used in the stencil test
    pub compare_op   : CompareOp,
    /// Defines the mask to apply to value that are considered during the test
    pub compare_mask : u32,
    /// Defines the mask to apply when writing a victorious value
    pub write_mask   : u32,
    /// The integer reference that is used during the stencil test
    pub reference    : u32,
}

impl From<vk::StencilOpState> for StencilOpState {
    #[inline]
    fn from(value: vk::StencilOpState) -> Self {
        Self {
            on_stencil_fail : value.fail_op.into(),
            on_depth_fail   : value.depth_fail_op.into(),
            on_success      : value.pass_op.into(),

            compare_op   : value.compare_op.into(),
            compare_mask : value.compare_mask,
            write_mask   : value.write_mask,
            reference    : value.reference,
        }
    }
}

impl From<StencilOpState> for vk::StencilOpState {
    #[inline]
    fn from(value: StencilOpState) -> Self {
        Self {
            fail_op       : value.on_stencil_fail.into(),
            depth_fail_op : value.on_depth_fail.into(),
            pass_op       : value.on_success.into(),

            compare_op   : value.compare_op.into(),
            compare_mask : value.compare_mask,
            write_mask   : value.write_mask,
            reference    : value.reference,
        }
    }
}




/// Defines if a depth stencil is present in the Pipeline and how.
#[derive(Clone, Debug)]
pub struct DepthTestingState {
    /// Whether to enable depth testing
    pub enable_depth   : bool,
    /// Whether to enable depth writing (only if `enable_depth` is true).
    pub enable_write   : bool,
    /// Whether to enable normal stencil testing
    pub enable_stencil : bool,
    /// Whether to enable depth bounds testing
    pub enable_bounds : bool,

    /// The compare operation to perform when testing the depth
    pub compare_op    : CompareOp,

    /// The properties of the stencil test before the depth testing
    pub pre_stencil_test : StencilOpState,
    /// The properties of the stencil test after the depth testing
    pub post_stencil_test : StencilOpState,

    /// The minimum depth bound used in the depth bounds test
    pub min_bound : f32,
    /// The maximum depth bound used in the depth bounds test
    pub max_bound : f32,
}

impl From<vk::PipelineDepthStencilStateCreateInfo> for DepthTestingState {
    #[inline]
    fn from(value: vk::PipelineDepthStencilStateCreateInfo) -> Self {
        Self {
            enable_depth   : value.depth_test_enable != 0,
            enable_write   : value.depth_write_enable != 0,
            enable_stencil : value.stencil_test_enable != 0,
            enable_bounds  : value.depth_bounds_test_enable != 0,

            compare_op : value.depth_compare_op.into(),

            pre_stencil_test  : value.front.into(),
            post_stencil_test : value.back.into(),

            min_bound : value.min_depth_bounds,
            max_bound : value.max_depth_bounds,
        }
    }
}

impl From<DepthTestingState> for vk::PipelineDepthStencilStateCreateInfo {
    #[inline]
    fn from(value: DepthTestingState) -> Self {
        Self {
            // Do the default stuff
            s_type : vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineDepthStencilStateCreateFlags::empty(),

            // Define which tests to enable
            depth_test_enable        : value.enable_depth as u32,
            depth_write_enable       : value.enable_write as u32,
            stencil_test_enable      : value.enable_stencil as u32,
            depth_bounds_test_enable : value.enable_bounds as u32,

            // Define the compare operation for the depth test
            depth_compare_op : value.compare_op.into(),

            // Define the stencil test states
            front : value.pre_stencil_test.into(),
            back  : value.post_stencil_test.into(),

            // Define the bounds for the bounds test
            min_depth_bounds : value.min_bound,
            max_depth_bounds : value.max_bound,
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
    /// ```
    /// Rd = Rs * FCs + Rd * FCd
    /// Gd = Gs * FCs + Gd * FCd
    /// Bd = Bs * FCs + Bd * FCd
    /// Ad = As * FAs + Ad * FAd
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    Add,
    /// Subtract the proper fractions of the colours from each other:
    /// ```
    /// Rd = Rs * FCs - Rd * FCd
    /// Gd = Gs * FCs - Gd * FCd
    /// Bd = Bs * FCs - Bd * FCd
    /// Ad = As * FAs - Ad * FAd
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    Sub,
    /// Subtract the proper fractions of the colours from each other:
    /// ```
    /// Rd = Rd * FCd - Rs * FCs
    /// Gd = Gd * FCd - Gs * FCs
    /// Bd = Bd * FCd - Bs * FCs
    /// Ad = Ad * FAd - As * FAs
    /// ```
    /// (`Xs` is the source channel, `Xd` is the destination channel, `FCx` is the source or destination colour fraction and `FAx` is the source or destination alpha fraction)
    SubRev,

    /// Take the minimal value of the colours (ignoring fractions):
    /// ```
    /// Rd = min(Rs, Rd)
    /// Gd = min(Gs, Gd)
    /// Bd = min(Bs, Bd)
    /// Ad = min(As, Ad)
    /// ```
    /// (`Xs` is the source channel and `Xd` is the destination channel)
    Min,
    /// Take the maximum value of the colours (ignoring fractions):
    /// ```
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



/// Defines the channel mask to use when writing.
#[derive(Clone, Copy, Debug)]
pub struct ColourMask(u8);

impl ColourMask {
    /// A ColourMask that hits all channels
    pub const ALL: Self   = Self(0b00001111);
    /// An empty ColourMask
    pub const EMPTY: Self = Self(0b00000000);

    /// A colour mask for only the red colour channel.
    pub const R: Self = Self(0b00000001);
    /// A colour mask for only the green colour channel.
    pub const G: Self = Self(0b00000010);
    /// A colour mask for only the blue colour channel.
    pub const B: Self = Self(0b00000100);
    /// A colour mask for only the alpha channel.
    pub const A: Self = Self(0b00001000);


    /// Returns whether the given ColourMask is a subset of this one.
    /// 
    /// # Arguments
    /// - `value`: The ColourMask that should be a subset of this one. For example, if value is Self::R, then returns true if the red colour channel was enabled in this ColourMask.
    #[inline]
    pub fn check(&self, other: ColourMask) -> bool { (self.0 & other.0) == other.0 }
}

impl BitOr for ColourMask {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ColourMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl From<vk::ColorComponentFlags> for ColourMask {
    #[inline]
    fn from(value: vk::ColorComponentFlags) -> Self {
        // Construct it manually for portability
        let mut result = ColourMask::EMPTY;
        if (value & vk::ColorComponentFlags::R).as_raw() != 0 { result |= ColourMask::R; }
        if (value & vk::ColorComponentFlags::G).as_raw() != 0 { result |= ColourMask::G; }
        if (value & vk::ColorComponentFlags::B).as_raw() != 0 { result |= ColourMask::B; }
        if (value & vk::ColorComponentFlags::A).as_raw() != 0 { result |= ColourMask::A; }

        // Return it
        result
    }
}

impl From<ColourMask> for vk::ColorComponentFlags {
    fn from(value: ColourMask) -> Self {
        // Construct it manually due to private constructors ;(
        let mut result = vk::ColorComponentFlags::empty();
        if value.check(ColourMask::R) { result |= vk::ColorComponentFlags::R; }
        if value.check(ColourMask::G) { result |= vk::ColorComponentFlags::G; }
        if value.check(ColourMask::B) { result |= vk::ColorComponentFlags::B; }
        if value.check(ColourMask::A) { result |= vk::ColorComponentFlags::A; }

        // Return it
        result
    }
}



/// Defines how to write colours to a single colour attachment.
#[derive(Clone, Debug)]
pub struct AttachmentBlendState {
    /// Whether to enable blending or not (values pass through unmodified if false).
    pub enable_blend : bool,

    /// The proportion of colour we take from the source.
    pub src_colour : BlendFactor,
    /// The proportion of colour we take from the destination.
    pub dst_colour : BlendFactor,
    /// The operator to use to blend the two colours
    pub colour_op  : BlendOp,

    /// The proportion of alpha we take from the source.
    pub src_alpha : BlendFactor,
    /// The proportion of alpha we take from the destination.
    pub dst_alpha : BlendFactor,
    /// The operator to use to blend the two alphas
    pub alpha_op  : BlendOp,

    /// A mask that specifies which channels are available for writing the blend.
    pub write_mask : ColourMask,
}

impl From<vk::PipelineColorBlendAttachmentState> for AttachmentBlendState {
    #[inline]
    fn from(value: vk::PipelineColorBlendAttachmentState) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&vk::PipelineColorBlendAttachmentState> for AttachmentBlendState {
    #[inline]
    fn from(value: &vk::PipelineColorBlendAttachmentState) -> Self {
        Self {
            enable_blend : value.blend_enable != 0,

            src_colour : value.src_color_blend_factor.into(),
            dst_colour : value.dst_color_blend_factor.into(),
            colour_op  : value.color_blend_op.into(),

            src_alpha : value.src_alpha_blend_factor.into(),
            dst_alpha : value.dst_alpha_blend_factor.into(),
            alpha_op  : value.alpha_blend_op.into(),

            write_mask : value.color_write_mask.into(),
        }
    }
}

impl From<AttachmentBlendState> for vk::PipelineColorBlendAttachmentState {
    #[inline]
    fn from(value: AttachmentBlendState) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&AttachmentBlendState> for vk::PipelineColorBlendAttachmentState {
    #[inline]
    fn from(value: &AttachmentBlendState) -> Self {
        Self {
            blend_enable : value.enable_blend as u32,

            src_color_blend_factor : value.src_colour.into(),
            dst_color_blend_factor : value.dst_colour.into(),
            color_blend_op         : value.colour_op.into(),

            src_alpha_blend_factor : value.src_alpha.into(),
            dst_alpha_blend_factor : value.dst_alpha.into(),
            alpha_blend_op         : value.alpha_op.into(),

            color_write_mask : value.write_mask.into(),
        }
    }
}



/// Defines how to write colours to the (multiple) colour attachments.
#[derive(Clone, Debug)]
pub struct ColourBlendState {
    /// Whether to apply any logic operations for all attachments.
    /// 
    /// If set to true, then ignores the attachment operations.
    pub enable_logic : bool,
    /// The logic operator to apply, if enabled.
    pub logic_op     : LogicOp,

    /// The list of colour attachment blend states that describe the per-attachment stats.
    pub attachment_states : Vec<AttachmentBlendState>,
    /// The constants for blending.
    pub blend_constants   : [f32; 4],
}

impl From<&vk::PipelineColorBlendStateCreateInfo> for ColourBlendState {
    fn from(value: &vk::PipelineColorBlendStateCreateInfo) -> Self {
        // Collect the raw pointers in a slice
        let attachments = unsafe { slice::from_raw_parts(value.p_attachments, value.attachment_count as usize) };

        // Cast them to our attachments, in a vec
        let attachments: Vec<AttachmentBlendState> = attachments.iter().map(|att| att.into()).collect();

        // Now create the struct with it and other properties
        Self {
            enable_logic : value.logic_op_enable != 0,
            logic_op     : value.logic_op.into(),

            attachment_states : attachments,
            blend_constants   : value.blend_constants.clone(),
        }
    }
}

impl Into<(vk::PipelineColorBlendStateCreateInfo, Vec<vk::PipelineColorBlendAttachmentState>)> for ColourBlendState {
    /// Converts the ColourBlendState into a VkPipelineColorBlendStateCreateInfo.
    /// 
    /// However, due to the external references made in the VkPipelineColorBlendStateCreateInfo struct, it also returns one Vec that manages the external memory referenced.
    /// 
    /// # Returns
    /// A tuple with:
    /// - The new VkPipelineColorBlendStateCreateInfo instance
    /// - The Vec with the referenced memory
    fn into(self) -> (vk::PipelineColorBlendStateCreateInfo, Vec<vk::PipelineColorBlendAttachmentState>) {
        // Cast our own attachment states to Vulkan's
        let attachments: Vec<vk::PipelineColorBlendAttachmentState> = self.attachment_states.iter().map(|att| att.into()).collect();

        // Now create the struct with it and other properties
        let result = vk::PipelineColorBlendStateCreateInfo {
            // Set the default stuff
            s_type : vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::PipelineColorBlendStateCreateFlags::empty(),

            // Set the logic properties
            logic_op_enable : self.enable_logic as u32,
            logic_op        : self.logic_op.into(),

            // Set the attachments and the blend constants
            p_attachments    : attachments.as_ptr(),
            attachment_count : attachments.len() as u32,
            blend_constants  : self.blend_constants.clone(),
        };

        // Done, return both it and the memory
        (result, attachments)
    }
}



/// Defines a part of the pipeline that may be set to dynamic
#[derive(Clone, Debug)]
pub enum DynamicState {
    /// Defines that the viewport of the ViewportState may be dynamic.
    Viewport,
    /// Defines that the scissor of the ViewportState may be dynamic.
    Scissor,
    /// Defines that the drawn line width may be dynamic.
    LineWidth,
    /// Defines that the depth bias for depth testing may be dynamic.
    DepthBias,
    /// Defines that the depth bounds for depth testing may be dynamic.
    DepthBounds,
    /// Defines that the compare mask of a stencil test may be dynamic.
    StencilCompareMask,
    /// Defines that the write mask of a stencil test may be dynamic.
    StencilWriteMask,
    /// Defines that the reference of a stencil test may be dynamic.
    StencilReference,
    /// Defines that the blend constants in colour blending may be dynamic.
    BlendConstants,
}

impl From<vk::DynamicState> for DynamicState {
    #[inline]
    fn from(value: vk::DynamicState) -> Self {
        match value {
            vk::DynamicState::VIEWPORT             => DynamicState::Viewport,
            vk::DynamicState::SCISSOR              => DynamicState::Scissor,
            vk::DynamicState::LINE_WIDTH           => DynamicState::LineWidth,
            vk::DynamicState::DEPTH_BIAS           => DynamicState::DepthBias,
            vk::DynamicState::DEPTH_BOUNDS         => DynamicState::DepthBounds,
            vk::DynamicState::STENCIL_COMPARE_MASK => DynamicState::StencilCompareMask,
            vk::DynamicState::STENCIL_WRITE_MASK   => DynamicState::StencilWriteMask,
            vk::DynamicState::STENCIL_REFERENCE    => DynamicState::StencilReference,
            vk::DynamicState::BLEND_CONSTANTS      => DynamicState::BlendConstants,

            value => { panic!("Encountered illegal VkDynamicState value '{}'", value.as_raw()); }
        }
    }
}

impl From<DynamicState> for vk::DynamicState {
    #[inline]
    fn from(value: DynamicState) -> Self {
        match value {
            DynamicState::Viewport           => vk::DynamicState::VIEWPORT,
            DynamicState::Scissor            => vk::DynamicState::SCISSOR,
            DynamicState::LineWidth          => vk::DynamicState::LINE_WIDTH,
            DynamicState::DepthBias          => vk::DynamicState::DEPTH_BIAS,
            DynamicState::DepthBounds        => vk::DynamicState::DEPTH_BOUNDS,
            DynamicState::StencilCompareMask => vk::DynamicState::STENCIL_COMPARE_MASK,
            DynamicState::StencilWriteMask   => vk::DynamicState::STENCIL_WRITE_MASK,
            DynamicState::StencilReference   => vk::DynamicState::STENCIL_REFERENCE,
            DynamicState::BlendConstants     => vk::DynamicState::BLEND_CONSTANTS,
        }
    }
}





/***** IMAGES *****/
/// The type of the ImageView
#[derive(Clone, Copy, Debug)]
pub enum ImageViewKind {
    /// A simple, one-dimensional image (i.e., a line of pixels)
    OneD,
    /// A simple, one-dimensional image but as an array (i.e., for stereophonic 3D)
    OneDArray,

    /// A simple, two-dimensional image (i.e., a grid of pixels)
    TwoD,
    /// A simple, two-dimensional image but as an array (i.e., for stereophonic 3D)
    TwoDArray,

    /// A simple, three-dimensional image
    ThreeD,

    /// A cubic (3D?) image
    Cube,
    /// A cubic (3D?) image but an array (i.e., for stereophonic 3D)
    CubeArray,
}

impl Default for ImageViewKind {
    #[inline]
    fn default() -> Self {
        ImageViewKind::TwoD
    }
}

impl Display for ImageViewKind {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ImageViewKind::*;
        match self {
            OneD      => write!(f, "1D"),
            OneDArray => write!(f, "1D (Array)"),
            TwoD      => write!(f, "2D"),
            TwoDArray => write!(f, "2D (Array)"),
            ThreeD    => write!(f, "3D"),
            Cube      => write!(f, "Cube"),
            CubeArray => write!(f, "Cube (Array)"),
        }
    }
}

impl From<vk::ImageViewType> for ImageViewKind {
    fn from(value: vk::ImageViewType) -> Self {
        match value {
            vk::ImageViewType::TYPE_1D       => ImageViewKind::OneD,
            vk::ImageViewType::TYPE_1D_ARRAY => ImageViewKind::OneDArray,
            vk::ImageViewType::TYPE_2D       => ImageViewKind::TwoD,
            vk::ImageViewType::TYPE_2D_ARRAY => ImageViewKind::TwoDArray,
            vk::ImageViewType::TYPE_3D       => ImageViewKind::ThreeD,
            vk::ImageViewType::CUBE          => ImageViewKind::Cube,
            vk::ImageViewType::CUBE_ARRAY    => ImageViewKind::CubeArray,
            _                                => { panic!("Encountered illegal ImageViewType value '{}'", value.as_raw()); }
        }
    }
}

impl From<ImageViewKind> for vk::ImageViewType {
    fn from(value: ImageViewKind) -> Self {
        match value {
            ImageViewKind::OneD      => vk::ImageViewType::TYPE_1D,
            ImageViewKind::OneDArray => vk::ImageViewType::TYPE_1D_ARRAY,
            ImageViewKind::TwoD      => vk::ImageViewType::TYPE_2D,
            ImageViewKind::TwoDArray => vk::ImageViewType::TYPE_2D_ARRAY,
            ImageViewKind::ThreeD    => vk::ImageViewType::TYPE_3D,
            ImageViewKind::Cube      => vk::ImageViewType::CUBE,
            ImageViewKind::CubeArray => vk::ImageViewType::CUBE_ARRAY,
        }
    }
}



/// The format of an Image.
#[derive(Clone, Copy, Debug)]
pub enum Format {
    /// The format is unknown
    Undefined,

    /// R4G4_UNORM_PACK8
    R4G4UNormPack8,
    /// R4G4B4A4_UNORM_PACK16
    R4G4B4A4UNormPack16,
    /// B4G4R4A4_UNORM_PACK16
    B4G4R4A4UNormPack16,
    /// R5G6B5_UNORM_PACK16
    R5G6B5UNormPack16,
    /// B5G6R5_UNORM_PACK16
    B5G6R5UNormPack16,
    /// R5G5B5A1_UNORM_PACK16
    R5G5B5A1UNormPack16,
    /// B5G5R5A1_UNORM_PACK16
    B5G5R5A1UNormPack16,
    /// A1R5G5B5_UNORM_PACK16
    A1R5G5B5UNormPack16,
    /// R8_UNORM
    R8UNorm,
    /// R8_SNORM
    R8SNorm,
    /// R8_USCALED
    R8UScaled,
    /// R8_SSCALED
    R8SScaled,
    /// R8_UINT
    R8UInt,
    /// R8_SINT
    R8SInt,
    /// R8_SRGB
    R8SRgb,
    /// R8G8_UNORM
    R8G8UNorm,
    /// R8G8_SNORM
    R8G8SNorm,
    /// R8G8_USCALED
    R8G8UScaled,
    /// R8G8_SSCALED
    R8G8SScaled,
    /// R8G8_UINT
    R8G8UInt,
    /// R8G8_SINT
    R8G8SInt,
    /// R8G8_SRGB
    R8G8SRgb,
    /// R8G8B8_UNORM
    R8G8B8UNorm,
    /// R8G8B8_SNORM
    R8G8B8SNorm,
    /// R8G8B8_USCALED
    R8G8B8UScaled,
    /// R8G8B8_SSCALED
    R8G8B8SScaled,
    /// R8G8B8_UINT
    R8G8B8UInt,
    /// R8G8B8_SINT
    R8G8B8SInt,
    /// R8G8B8_SRGB
    R8G8B8SRgb,
    /// B8G8R8_UNORM
    B8G8R8UNorm,
    /// B8G8R8_SNORM
    B8G8R8SNorm,
    /// B8G8R8_USCALED
    B8G8R8UScaled,
    /// B8G8R8_SSCALED
    B8G8R8SScaled,
    /// B8G8R8_UINT
    B8G8R8UInt,
    /// B8G8R8_SINT
    B8G8R8SInt,
    /// B8G8R8_SRGB
    B8G8R8SRgb,
    /// R8G8B8A8_UNORM
    R8G8B8A8UNorm,
    /// R8G8B8A8_SNORM
    R8G8B8A8SNorm,
    /// R8G8B8A8_USCALED
    R8G8B8A8UScaled,
    /// R8G8B8A8_SSCALED
    R8G8B8A8SScaled,
    /// R8G8B8A8_UINT
    R8G8B8A8UInt,
    /// R8G8B8A8_SINT
    R8G8B8A8SInt,
    /// R8G8B8A8_SRGB
    R8G8B8A8SRgb,
    /// B8G8R8A8_UNORM
    B8G8R8A8UNorm,
    /// B8G8R8A8_SNORM
    B8G8R8A8SNorm,
    /// B8G8R8A8_USCALED
    B8G8R8A8UScaled,
    /// B8G8R8A8_SSCALED
    B8G8R8A8SScaled,
    /// B8G8R8A8_UINT
    B8G8R8A8UInt,
    /// B8G8R8A8_SINT
    B8G8R8A8SInt,
    /// B8G8R8A8_SRGB
    B8G8R8A8SRgb,
    /// A8B8G8R8_UNORM_PACK32
    A8B8G8R8UNormPack32,
    /// A8B8G8R8_SNORM_PACK32
    A8B8G8R8SNormPack32,
    /// A8B8G8R8_USCALED_PACK32
    A8B8G8R8UScaledPack32,
    /// A8B8G8R8_SSCALED_PACK32
    A8B8G8R8SScaledPack32,
    /// A8B8G8R8_UINT_PACK32
    A8B8G8R8UIntPack32,
    /// A8B8G8R8_SINT_PACK32
    A8B8G8R8SIntPack32,
    /// A8B8G8R8_SRGB_PACK32
    A8B8G8R8SRgbPack32,
    /// A2R10G10B10_UNORM_PACK32
    A2R10G10B10UNormPack32,
    /// A2R10G10B10_SNORM_PACK32
    A2R10G10B10SNormPack32,
    /// A2R10G10B10_USCALED_PACK32
    A2R10G10B10UScaledPack32,
    /// A2R10G10B10_SSCALED_PACK32
    A2R10G10B10SScaledPack32,
    /// A2R10G10B10_UINT_PACK32
    A2R10G10B10UIntPack32,
    /// A2R10G10B10_SINT_PACK32
    A2R10G10B10SIntPack32,
    /// A2B10G10R10_UNORM_PACK32
    A2B10G10R10UNormPack32,
    /// A2B10G10R10_SNORM_PACK32
    A2B10G10R10SNormPack32,
    /// A2B10G10R10_USCALED_PACK32
    A2B10G10R10UScaledPack32,
    /// A2B10G10R10_SSCALED_PACK32
    A2B10G10R10SScaledPack32,
    /// A2B10G10R10_UINT_PACK32
    A2B10G10R10UIntPack32,
    /// A2B10G10R10_SINT_PACK32
    A2B10G10R10SIntPack32,
    /// R16_UNORM
    R16UNorm,
    /// R16_SNORM
    R16SNorm,
    /// R16_USCALED
    R16UScaled,
    /// R16_SSCALED
    R16SScaled,
    /// R16_UINT
    R16UInt,
    /// R16_SINT
    R16SInt,
    /// R16_SFLOAT
    R16SFloat,
    /// R16G16_UNORM
    R16G16UNorm,
    /// R16G16_SNORM
    R16G16SNorm,
    /// R16G16_USCALED
    R16G16UScaled,
    /// R16G16_SSCALED
    R16G16SScaled,
    /// R16G16_UINT
    R16G16UInt,
    /// R16G16_SINT
    R16G16SInt,
    /// R16G16_SFLOAT
    R16G16SFloat,
    /// R16G16B16_UNORM
    R16G16B16UNorm,
    /// R16G16B16_SNORM
    R16G16B16SNorm,
    /// R16G16B16_USCALED
    R16G16B16UScaled,
    /// R16G16B16_SSCALED
    R16G16B16SScaled,
    /// R16G16B16_UINT
    R16G16B16UInt,
    /// R16G16B16_SINT
    R16G16B16SInt,
    /// R16G16B16_SFLOAT
    R16G16B16SFloat,
    /// R16G16B16A16_UNORM
    R16G16B16A16UNorm,
    /// R16G16B16A16_SNORM
    R16G16B16A16SNorm,
    /// R16G16B16A16_USCALED
    R16G16B16A16UScaled,
    /// R16G16B16A16_SSCALED
    R16G16B16A16SScaled,
    /// R16G16B16A16_UINT
    R16G16B16A16UInt,
    /// R16G16B16A16_SINT
    R16G16B16A16SInt,
    /// R16G16B16A16_SFLOAT
    R16G16B16A16SFloat,
    /// R32_UINT
    R32UInt,
    /// R32_SINT
    R32SInt,
    /// R32_SFLOAT
    R32SFloat,
    /// R32G32_UINT
    R32G32UInt,
    /// R32G32_SINT
    R32G32SInt,
    /// R32G32_SFLOAT
    R32G32SFloat,
    /// R32G32B32_UINT
    R32G32B32UInt,
    /// R32G32B32_SINT
    R32G32B32SInt,
    /// R32G32B32_SFLOAT
    R32G32B32SFloat,
    /// R32G32B32A32_UINT
    R32G32B32A32UInt,
    /// R32G32B32A32_SINT
    R32G32B32A32SInt,
    /// R32G32B32A32_SFLOAT
    R32G32B32A32SFloat,
    /// R64_UINT
    R64UInt,
    /// R64_SINT
    R64SInt,
    /// R64_SFLOAT
    R64SFloat,
    /// R64G64_UINT
    R64G64UInt,
    /// R64G64_SINT
    R64G64SInt,
    /// R64G64_SFLOAT
    R64G64SFloat,
    /// R64G64B64_UINT
    R64G64B64UInt,
    /// R64G64B64_SINT
    R64G64B64SInt,
    /// R64G64B64_SFLOAT
    R64G64B64SFloat,
    /// R64G64B64A64_UINT
    R64G64B64A64UInt,
    /// R64G64B64A64_SINT
    R64G64B64A64SInt,
    /// R64G64B64A64_SFLOAT
    R64G64B64A64SFloat,
    /// B10G11R11_UFLOAT_PACK32
    B10G11R11UFloatPack32,
    /// E5B9G9R9_UFLOAT_PACK32
    E5B9G9R9UFloatPack32,
    /// D16_UNORM
    D16UNorm,
    /// X8_D24_UNORM_PACK32
    X8D24UNormPack32,
    /// D32_SFLOAT
    D32SFloat,
    /// S8_UINT
    S8UInt,
    /// D16_UNORM_S8_UINT
    D16UNormS8UInt,
    /// D24_UNORM_S8_UINT
    D24UNormS8UInt,
    /// D32_SFLOAT_S8_UINT
    D32SFloatS8UInt,
    /// BC1_RGB_UNORM_BLOCK
    BC1RGBUNormBlock,
    /// BC1_RGB_SRGB_BLOCK
    BC1RGBSRgbBlock,
    /// BC1_RGBA_UNORM_BLOCK
    BC1RGBAUNormBlock,
    /// BC1_RGBA_SRGB_BLOCK
    BC1RGBASRgbBlock,
    /// BC2_UNORM_BLOCK
    BC2UNormBlock,
    /// BC2_SRGB_BLOCK
    BC2SRgbBlock,
    /// BC3_UNORM_BLOCK
    BC3UNormBlock,
    /// BC3_SRGB_BLOCK
    BC3SRgbBlock,
    /// BC4_UNORM_BLOCK
    BC4UNormBlock,
    /// BC4_SNORM_BLOCK
    BC4SNormBlock,
    /// BC5_UNORM_BLOCK
    BC5UNormBlock,
    /// BC5_SNORM_BLOCK
    BC5SNormBlock,
    /// BC6H_UFLOAT_BLOCK
    BC6HUFloatBlock,
    /// BC6H_SFLOAT_BLOCK
    BC6HSFloatBlock,
    /// BC7_UNORM_BLOCK
    BC7UNormBlock,
    /// BC7_SRGB_BLOCK
    BC7SRgbBlock,
    /// ETC2_R8G8B8_UNORM_BLOCK
    ETC2R8G8B8UNormBlock,
    /// ETC2_R8G8B8_SRGB_BLOCK
    ETC2R8G8B8SRgbBlock,
    /// ETC2_R8G8B8A1_UNORM_BLOCK
    ETC2R8G8B8A1UNormBlock,
    /// ETC2_R8G8B8A1_SRGB_BLOCK
    ETC2R8G8B8A1SRgbBlock,
    /// ETC2_R8G8B8A8_UNORM_BLOCK
    ETC2R8G8B8A8UNormBlock,
    /// ETC2_R8G8B8A8_SRGB_BLOCK
    ETC2R8G8B8A8SRgbBlock,
    /// EAC_R11_UNORM_BLOCK
    EACR11UNormBlock,
    /// EAC_R11_SNORM_BLOCK
    EACR11SNormBlock,
    /// EAC_R11G11_UNORM_BLOCK
    EACR11G11UNormBlock,
    /// EAC_R11G11_SNORM_BLOCK
    EACR11G11SNormBlock,
    /// ASTC_4X4_UNORM_BLOCK
    ASTC4X4UNormBlock,
    /// ASTC_4X4_SRGB_BLOCK
    ASTC4X4SRgbBlock,
    /// ASTC_5X4_UNORM_BLOCK
    ASTC5X4UNormBlock,
    /// ASTC_5X4_SRGB_BLOCK
    ASTC5X4SRgbBlock,
    /// ASTC_5X5_UNORM_BLOCK
    ASTC5X5UNormBlock,
    /// ASTC_5X5_SRGB_BLOCK
    ASTC5X5SRgbBlock,
    /// ASTC_6X5_UNORM_BLOCK
    ASTC6X5UNormBlock,
    /// ASTC_6X5_SRGB_BLOCK
    ASTC6X5SRgbBlock,
    /// ASTC_6X6_UNORM_BLOCK
    ASTC6X6UNormBlock,
    /// ASTC_6X6_SRGB_BLOCK
    ASTC6X6SRgbBlock,
    /// ASTC_8X5_UNORM_BLOCK
    ASTC8X5UNormBlock,
    /// ASTC_8X5_SRGB_BLOCK
    ASTC8X5SRgbBlock,
    /// ASTC_8X6_UNORM_BLOCK
    ASTC8X6UNormBlock,
    /// ASTC_8X6_SRGB_BLOCK
    ASTC8X6SRgbBlock,
    /// ASTC_8X8_UNORM_BLOCK
    ASTC8X8UNormBlock,
    /// ASTC_8X8_SRGB_BLOCK
    ASTC8X8SRgbBlock,
    /// ASTC_10X5_UNORM_BLOCK
    ASTC10X5UNormBlock,
    /// ASTC_10X5_SRGB_BLOCK
    ASTC10X5SRgbBlock,
    /// ASTC_10X6_UNORM_BLOCK
    ASTC10X6UNormBlock,
    /// ASTC_10X6_SRGB_BLOCK
    ASTC10X6SRgbBlock,
    /// ASTC_10X8_UNORM_BLOCK
    ASTC10X8UNormBlock,
    /// ASTC_10X8_SRGB_BLOCK
    ASTC10X8SRgbBlock,
    /// ASTC_10X10_UNORM_BLOCK
    ASTC10X10UNormBlock,
    /// ASTC_10X10_SRGB_BLOCK
    ASTC10X10SRgbBlock,
    /// ASTC_12X10_UNORM_BLOCK
    ASTC12X10UNormBlock,
    /// ASTC_12X10_SRGB_BLOCK
    ASTC12X10SRgbBlock,
    /// ASTC_12X12_UNORM_BLOCK
    ASTC12X12UNormBlock,
    /// ASTC_12X12_SRGB_BLOCK
    ASTC12X12SRgbBlock,
}

impl Default for Format {
    #[inline]
    fn default() -> Self {
        Format::B8G8R8A8SRgb
    }
}

impl Display for Format {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Format::*;
        match self {
            Undefined => write!(f, "Undefined"),

            R4G4UNormPack8 => write!(f, "R4G4UNormPack8"),
            R4G4B4A4UNormPack16 => write!(f, "R4G4B4A4UNormPack16"),
            B4G4R4A4UNormPack16 => write!(f, "B4G4R4A4UNormPack16"),
            R5G6B5UNormPack16 => write!(f, "R5G6B5UNormPack16"),
            B5G6R5UNormPack16 => write!(f, "B5G6R5UNormPack16"),
            R5G5B5A1UNormPack16 => write!(f, "R5G5B5A1UNormPack16"),
            B5G5R5A1UNormPack16 => write!(f, "B5G5R5A1UNormPack16"),
            A1R5G5B5UNormPack16 => write!(f, "A1R5G5B5UNormPack16"),
            R8UNorm => write!(f, "R8UNorm"),
            R8SNorm => write!(f, "R8SNorm"),
            R8UScaled => write!(f, "R8UScaled"),
            R8SScaled => write!(f, "R8SScaled"),
            R8UInt => write!(f, "R8UInt"),
            R8SInt => write!(f, "R8SInt"),
            R8SRgb => write!(f, "R8SRgb"),
            R8G8UNorm => write!(f, "R8G8UNorm"),
            R8G8SNorm => write!(f, "R8G8SNorm"),
            R8G8UScaled => write!(f, "R8G8UScaled"),
            R8G8SScaled => write!(f, "R8G8SScaled"),
            R8G8UInt => write!(f, "R8G8UInt"),
            R8G8SInt => write!(f, "R8G8SInt"),
            R8G8SRgb => write!(f, "R8G8SRgb"),
            R8G8B8UNorm => write!(f, "R8G8B8UNorm"),
            R8G8B8SNorm => write!(f, "R8G8B8SNorm"),
            R8G8B8UScaled => write!(f, "R8G8B8UScaled"),
            R8G8B8SScaled => write!(f, "R8G8B8SScaled"),
            R8G8B8UInt => write!(f, "R8G8B8UInt"),
            R8G8B8SInt => write!(f, "R8G8B8SInt"),
            R8G8B8SRgb => write!(f, "R8G8B8SRgb"),
            B8G8R8UNorm => write!(f, "B8G8R8UNorm"),
            B8G8R8SNorm => write!(f, "B8G8R8SNorm"),
            B8G8R8UScaled => write!(f, "B8G8R8UScaled"),
            B8G8R8SScaled => write!(f, "B8G8R8SScaled"),
            B8G8R8UInt => write!(f, "B8G8R8UInt"),
            B8G8R8SInt => write!(f, "B8G8R8SInt"),
            B8G8R8SRgb => write!(f, "B8G8R8SRgb"),
            R8G8B8A8UNorm => write!(f, "R8G8B8A8UNorm"),
            R8G8B8A8SNorm => write!(f, "R8G8B8A8SNorm"),
            R8G8B8A8UScaled => write!(f, "R8G8B8A8UScaled"),
            R8G8B8A8SScaled => write!(f, "R8G8B8A8SScaled"),
            R8G8B8A8UInt => write!(f, "R8G8B8A8UInt"),
            R8G8B8A8SInt => write!(f, "R8G8B8A8SInt"),
            R8G8B8A8SRgb => write!(f, "R8G8B8A8SRgb"),
            B8G8R8A8UNorm => write!(f, "B8G8R8A8UNorm"),
            B8G8R8A8SNorm => write!(f, "B8G8R8A8SNorm"),
            B8G8R8A8UScaled => write!(f, "B8G8R8A8UScaled"),
            B8G8R8A8SScaled => write!(f, "B8G8R8A8SScaled"),
            B8G8R8A8UInt => write!(f, "B8G8R8A8UInt"),
            B8G8R8A8SInt => write!(f, "B8G8R8A8SInt"),
            B8G8R8A8SRgb => write!(f, "B8G8R8A8SRgb"),
            A8B8G8R8UNormPack32 => write!(f, "A8B8G8R8UNormPack32"),
            A8B8G8R8SNormPack32 => write!(f, "A8B8G8R8SNormPack32"),
            A8B8G8R8UScaledPack32 => write!(f, "A8B8G8R8UScaledPack32"),
            A8B8G8R8SScaledPack32 => write!(f, "A8B8G8R8SScaledPack32"),
            A8B8G8R8UIntPack32 => write!(f, "A8B8G8R8UIntPack32"),
            A8B8G8R8SIntPack32 => write!(f, "A8B8G8R8SIntPack32"),
            A8B8G8R8SRgbPack32 => write!(f, "A8B8G8R8SRgbPack32"),
            A2R10G10B10UNormPack32 => write!(f, "A2R10G10B10UNormPack32"),
            A2R10G10B10SNormPack32 => write!(f, "A2R10G10B10SNormPack32"),
            A2R10G10B10UScaledPack32 => write!(f, "A2R10G10B10UScaledPack32"),
            A2R10G10B10SScaledPack32 => write!(f, "A2R10G10B10SScaledPack32"),
            A2R10G10B10UIntPack32 => write!(f, "A2R10G10B10UIntPack32"),
            A2R10G10B10SIntPack32 => write!(f, "A2R10G10B10SIntPack32"),
            A2B10G10R10UNormPack32 => write!(f, "A2B10G10R10UNormPack32"),
            A2B10G10R10SNormPack32 => write!(f, "A2B10G10R10SNormPack32"),
            A2B10G10R10UScaledPack32 => write!(f, "A2B10G10R10UScaledPack32"),
            A2B10G10R10SScaledPack32 => write!(f, "A2B10G10R10SScaledPack32"),
            A2B10G10R10UIntPack32 => write!(f, "A2B10G10R10UIntPack32"),
            A2B10G10R10SIntPack32 => write!(f, "A2B10G10R10SIntPack32"),
            R16UNorm => write!(f, "R16UNorm"),
            R16SNorm => write!(f, "R16SNorm"),
            R16UScaled => write!(f, "R16UScaled"),
            R16SScaled => write!(f, "R16SScaled"),
            R16UInt => write!(f, "R16UInt"),
            R16SInt => write!(f, "R16SInt"),
            R16SFloat => write!(f, "R16SFloat"),
            R16G16UNorm => write!(f, "R16G16UNorm"),
            R16G16SNorm => write!(f, "R16G16SNorm"),
            R16G16UScaled => write!(f, "R16G16UScaled"),
            R16G16SScaled => write!(f, "R16G16SScaled"),
            R16G16UInt => write!(f, "R16G16UInt"),
            R16G16SInt => write!(f, "R16G16SInt"),
            R16G16SFloat => write!(f, "R16G16SFloat"),
            R16G16B16UNorm => write!(f, "R16G16B16UNorm"),
            R16G16B16SNorm => write!(f, "R16G16B16SNorm"),
            R16G16B16UScaled => write!(f, "R16G16B16UScaled"),
            R16G16B16SScaled => write!(f, "R16G16B16SScaled"),
            R16G16B16UInt => write!(f, "R16G16B16UInt"),
            R16G16B16SInt => write!(f, "R16G16B16SInt"),
            R16G16B16SFloat => write!(f, "R16G16B16SFloat"),
            R16G16B16A16UNorm => write!(f, "R16G16B16A16UNorm"),
            R16G16B16A16SNorm => write!(f, "R16G16B16A16SNorm"),
            R16G16B16A16UScaled => write!(f, "R16G16B16A16UScaled"),
            R16G16B16A16SScaled => write!(f, "R16G16B16A16SScaled"),
            R16G16B16A16UInt => write!(f, "R16G16B16A16UInt"),
            R16G16B16A16SInt => write!(f, "R16G16B16A16SInt"),
            R16G16B16A16SFloat => write!(f, "R16G16B16A16SFloat"),
            R32UInt => write!(f, "R32UInt"),
            R32SInt => write!(f, "R32SInt"),
            R32SFloat => write!(f, "R32SFloat"),
            R32G32UInt => write!(f, "R32G32UInt"),
            R32G32SInt => write!(f, "R32G32SInt"),
            R32G32SFloat => write!(f, "R32G32SFloat"),
            R32G32B32UInt => write!(f, "R32G32B32UInt"),
            R32G32B32SInt => write!(f, "R32G32B32SInt"),
            R32G32B32SFloat => write!(f, "R32G32B32SFloat"),
            R32G32B32A32UInt => write!(f, "R32G32B32A32UInt"),
            R32G32B32A32SInt => write!(f, "R32G32B32A32SInt"),
            R32G32B32A32SFloat => write!(f, "R32G32B32A32SFloat"),
            R64UInt => write!(f, "R64UInt"),
            R64SInt => write!(f, "R64SInt"),
            R64SFloat => write!(f, "R64SFloat"),
            R64G64UInt => write!(f, "R64G64UInt"),
            R64G64SInt => write!(f, "R64G64SInt"),
            R64G64SFloat => write!(f, "R64G64SFloat"),
            R64G64B64UInt => write!(f, "R64G64B64UInt"),
            R64G64B64SInt => write!(f, "R64G64B64SInt"),
            R64G64B64SFloat => write!(f, "R64G64B64SFloat"),
            R64G64B64A64UInt => write!(f, "R64G64B64A64UInt"),
            R64G64B64A64SInt => write!(f, "R64G64B64A64SInt"),
            R64G64B64A64SFloat => write!(f, "R64G64B64A64SFloat"),
            B10G11R11UFloatPack32 => write!(f, "B10G11R11UFloatPack32"),
            E5B9G9R9UFloatPack32 => write!(f, "E5B9G9R9UFloatPack32"),
            D16UNorm => write!(f, "D16UNorm"),
            X8D24UNormPack32 => write!(f, "X8D24UNormPack32"),
            D32SFloat => write!(f, "D32SFloat"),
            S8UInt => write!(f, "S8UInt"),
            D16UNormS8UInt => write!(f, "D16UNormS8UInt"),
            D24UNormS8UInt => write!(f, "D24UNormS8UInt"),
            D32SFloatS8UInt => write!(f, "D32SFloatS8UInt"),
            BC1RGBUNormBlock => write!(f, "BC1RGBUNormBlock"),
            BC1RGBSRgbBlock => write!(f, "BC1RGBSRgbBlock"),
            BC1RGBAUNormBlock => write!(f, "BC1RGBAUNormBlock"),
            BC1RGBASRgbBlock => write!(f, "BC1RGBASRgbBlock"),
            BC2UNormBlock => write!(f, "BC2UNormBlock"),
            BC2SRgbBlock => write!(f, "BC2SRgbBlock"),
            BC3UNormBlock => write!(f, "BC3UNormBlock"),
            BC3SRgbBlock => write!(f, "BC3SRgbBlock"),
            BC4UNormBlock => write!(f, "BC4UNormBlock"),
            BC4SNormBlock => write!(f, "BC4SNormBlock"),
            BC5UNormBlock => write!(f, "BC5UNormBlock"),
            BC5SNormBlock => write!(f, "BC5SNormBlock"),
            BC6HUFloatBlock => write!(f, "BC6HUFloatBlock"),
            BC6HSFloatBlock => write!(f, "BC6HSFloatBlock"),
            BC7UNormBlock => write!(f, "BC7UNormBlock"),
            BC7SRgbBlock => write!(f, "BC7SRgbBlock"),
            ETC2R8G8B8UNormBlock => write!(f, "ETC2R8G8B8UNormBlock"),
            ETC2R8G8B8SRgbBlock => write!(f, "ETC2R8G8B8SRgbBlock"),
            ETC2R8G8B8A1UNormBlock => write!(f, "ETC2R8G8B8A1UNormBlock"),
            ETC2R8G8B8A1SRgbBlock => write!(f, "ETC2R8G8B8A1SRgbBlock"),
            ETC2R8G8B8A8UNormBlock => write!(f, "ETC2R8G8B8A8UNormBlock"),
            ETC2R8G8B8A8SRgbBlock => write!(f, "ETC2R8G8B8A8SRgbBlock"),
            EACR11UNormBlock => write!(f, "EACR11UNormBlock"),
            EACR11SNormBlock => write!(f, "EACR11SNormBlock"),
            EACR11G11UNormBlock => write!(f, "EACR11G11UNormBlock"),
            EACR11G11SNormBlock => write!(f, "EACR11G11SNormBlock"),
            ASTC4X4UNormBlock => write!(f, "ASTC4X4UNormBlock"),
            ASTC4X4SRgbBlock => write!(f, "ASTC4X4SRgbBlock"),
            ASTC5X4UNormBlock => write!(f, "ASTC5X4UNormBlock"),
            ASTC5X4SRgbBlock => write!(f, "ASTC5X4SRgbBlock"),
            ASTC5X5UNormBlock => write!(f, "ASTC5X5UNormBlock"),
            ASTC5X5SRgbBlock => write!(f, "ASTC5X5SRgbBlock"),
            ASTC6X5UNormBlock => write!(f, "ASTC6X5UNormBlock"),
            ASTC6X5SRgbBlock => write!(f, "ASTC6X5SRgbBlock"),
            ASTC6X6UNormBlock => write!(f, "ASTC6X6UNormBlock"),
            ASTC6X6SRgbBlock => write!(f, "ASTC6X6SRgbBlock"),
            ASTC8X5UNormBlock => write!(f, "ASTC8X5UNormBlock"),
            ASTC8X5SRgbBlock => write!(f, "ASTC8X5SRgbBlock"),
            ASTC8X6UNormBlock => write!(f, "ASTC8X6UNormBlock"),
            ASTC8X6SRgbBlock => write!(f, "ASTC8X6SRgbBlock"),
            ASTC8X8UNormBlock => write!(f, "ASTC8X8UNormBlock"),
            ASTC8X8SRgbBlock => write!(f, "ASTC8X8SRgbBlock"),
            ASTC10X5UNormBlock => write!(f, "ASTC10X5UNormBlock"),
            ASTC10X5SRgbBlock => write!(f, "ASTC10X5SRgbBlock"),
            ASTC10X6UNormBlock => write!(f, "ASTC10X6UNormBlock"),
            ASTC10X6SRgbBlock => write!(f, "ASTC10X6SRgbBlock"),
            ASTC10X8UNormBlock => write!(f, "ASTC10X8UNormBlock"),
            ASTC10X8SRgbBlock => write!(f, "ASTC10X8SRgbBlock"),
            ASTC10X10UNormBlock => write!(f, "ASTC10X10UNormBlock"),
            ASTC10X10SRgbBlock => write!(f, "ASTC10X10SRgbBlock"),
            ASTC12X10UNormBlock => write!(f, "ASTC12X10UNormBlock"),
            ASTC12X10SRgbBlock => write!(f, "ASTC12X10SRgbBlock"),
            ASTC12X12UNormBlock => write!(f, "ASTC12X12UNormBlock"),
            ASTC12X12SRgbBlock => write!(f, "ASTC12X12SRgbBlock"),
        }
    }
}

impl From<vk::Format> for Format {
    fn from(value: vk::Format) -> Self {
        match value {
            vk::Format::UNDEFINED => Format::Undefined,

            vk::Format::R4G4_UNORM_PACK8 => Format::R4G4UNormPack8,
            vk::Format::R4G4B4A4_UNORM_PACK16 => Format::R4G4B4A4UNormPack16,
            vk::Format::B4G4R4A4_UNORM_PACK16 => Format::B4G4R4A4UNormPack16,
            vk::Format::R5G6B5_UNORM_PACK16 => Format::R5G6B5UNormPack16,
            vk::Format::B5G6R5_UNORM_PACK16 => Format::B5G6R5UNormPack16,
            vk::Format::R5G5B5A1_UNORM_PACK16 => Format::R5G5B5A1UNormPack16,
            vk::Format::B5G5R5A1_UNORM_PACK16 => Format::B5G5R5A1UNormPack16,
            vk::Format::A1R5G5B5_UNORM_PACK16 => Format::A1R5G5B5UNormPack16,
            vk::Format::R8_UNORM => Format::R8UNorm,
            vk::Format::R8_SNORM => Format::R8SNorm,
            vk::Format::R8_USCALED => Format::R8UScaled,
            vk::Format::R8_SSCALED => Format::R8SScaled,
            vk::Format::R8_UINT => Format::R8UInt,
            vk::Format::R8_SINT => Format::R8SInt,
            vk::Format::R8_SRGB => Format::R8SRgb,
            vk::Format::R8G8_UNORM => Format::R8G8UNorm,
            vk::Format::R8G8_SNORM => Format::R8G8SNorm,
            vk::Format::R8G8_USCALED => Format::R8G8UScaled,
            vk::Format::R8G8_SSCALED => Format::R8G8SScaled,
            vk::Format::R8G8_UINT => Format::R8G8UInt,
            vk::Format::R8G8_SINT => Format::R8G8SInt,
            vk::Format::R8G8_SRGB => Format::R8G8SRgb,
            vk::Format::R8G8B8_UNORM => Format::R8G8B8UNorm,
            vk::Format::R8G8B8_SNORM => Format::R8G8B8SNorm,
            vk::Format::R8G8B8_USCALED => Format::R8G8B8UScaled,
            vk::Format::R8G8B8_SSCALED => Format::R8G8B8SScaled,
            vk::Format::R8G8B8_UINT => Format::R8G8B8UInt,
            vk::Format::R8G8B8_SINT => Format::R8G8B8SInt,
            vk::Format::R8G8B8_SRGB => Format::R8G8B8SRgb,
            vk::Format::B8G8R8_UNORM => Format::B8G8R8UNorm,
            vk::Format::B8G8R8_SNORM => Format::B8G8R8SNorm,
            vk::Format::B8G8R8_USCALED => Format::B8G8R8UScaled,
            vk::Format::B8G8R8_SSCALED => Format::B8G8R8SScaled,
            vk::Format::B8G8R8_UINT => Format::B8G8R8UInt,
            vk::Format::B8G8R8_SINT => Format::B8G8R8SInt,
            vk::Format::B8G8R8_SRGB => Format::B8G8R8SRgb,
            vk::Format::R8G8B8A8_UNORM => Format::R8G8B8A8UNorm,
            vk::Format::R8G8B8A8_SNORM => Format::R8G8B8A8SNorm,
            vk::Format::R8G8B8A8_USCALED => Format::R8G8B8A8UScaled,
            vk::Format::R8G8B8A8_SSCALED => Format::R8G8B8A8SScaled,
            vk::Format::R8G8B8A8_UINT => Format::R8G8B8A8UInt,
            vk::Format::R8G8B8A8_SINT => Format::R8G8B8A8SInt,
            vk::Format::R8G8B8A8_SRGB => Format::R8G8B8A8SRgb,
            vk::Format::B8G8R8A8_UNORM => Format::B8G8R8A8UNorm,
            vk::Format::B8G8R8A8_SNORM => Format::B8G8R8A8SNorm,
            vk::Format::B8G8R8A8_USCALED => Format::B8G8R8A8UScaled,
            vk::Format::B8G8R8A8_SSCALED => Format::B8G8R8A8SScaled,
            vk::Format::B8G8R8A8_UINT => Format::B8G8R8A8UInt,
            vk::Format::B8G8R8A8_SINT => Format::B8G8R8A8SInt,
            vk::Format::B8G8R8A8_SRGB => Format::B8G8R8A8SRgb,
            vk::Format::A8B8G8R8_UNORM_PACK32 => Format::A8B8G8R8UNormPack32,
            vk::Format::A8B8G8R8_SNORM_PACK32 => Format::A8B8G8R8SNormPack32,
            vk::Format::A8B8G8R8_USCALED_PACK32 => Format::A8B8G8R8UScaledPack32,
            vk::Format::A8B8G8R8_SSCALED_PACK32 => Format::A8B8G8R8SScaledPack32,
            vk::Format::A8B8G8R8_UINT_PACK32 => Format::A8B8G8R8UIntPack32,
            vk::Format::A8B8G8R8_SINT_PACK32 => Format::A8B8G8R8SIntPack32,
            vk::Format::A8B8G8R8_SRGB_PACK32 => Format::A8B8G8R8SRgbPack32,
            vk::Format::A2R10G10B10_UNORM_PACK32 => Format::A2R10G10B10UNormPack32,
            vk::Format::A2R10G10B10_SNORM_PACK32 => Format::A2R10G10B10SNormPack32,
            vk::Format::A2R10G10B10_USCALED_PACK32 => Format::A2R10G10B10UScaledPack32,
            vk::Format::A2R10G10B10_SSCALED_PACK32 => Format::A2R10G10B10SScaledPack32,
            vk::Format::A2R10G10B10_UINT_PACK32 => Format::A2R10G10B10UIntPack32,
            vk::Format::A2R10G10B10_SINT_PACK32 => Format::A2R10G10B10SIntPack32,
            vk::Format::A2B10G10R10_UNORM_PACK32 => Format::A2B10G10R10UNormPack32,
            vk::Format::A2B10G10R10_SNORM_PACK32 => Format::A2B10G10R10SNormPack32,
            vk::Format::A2B10G10R10_USCALED_PACK32 => Format::A2B10G10R10UScaledPack32,
            vk::Format::A2B10G10R10_SSCALED_PACK32 => Format::A2B10G10R10SScaledPack32,
            vk::Format::A2B10G10R10_UINT_PACK32 => Format::A2B10G10R10UIntPack32,
            vk::Format::A2B10G10R10_SINT_PACK32 => Format::A2B10G10R10SIntPack32,
            vk::Format::R16_UNORM => Format::R16UNorm,
            vk::Format::R16_SNORM => Format::R16SNorm,
            vk::Format::R16_USCALED => Format::R16UScaled,
            vk::Format::R16_SSCALED => Format::R16SScaled,
            vk::Format::R16_UINT => Format::R16UInt,
            vk::Format::R16_SINT => Format::R16SInt,
            vk::Format::R16_SFLOAT => Format::R16SFloat,
            vk::Format::R16G16_UNORM => Format::R16G16UNorm,
            vk::Format::R16G16_SNORM => Format::R16G16SNorm,
            vk::Format::R16G16_USCALED => Format::R16G16UScaled,
            vk::Format::R16G16_SSCALED => Format::R16G16SScaled,
            vk::Format::R16G16_UINT => Format::R16G16UInt,
            vk::Format::R16G16_SINT => Format::R16G16SInt,
            vk::Format::R16G16_SFLOAT => Format::R16G16SFloat,
            vk::Format::R16G16B16_UNORM => Format::R16G16B16UNorm,
            vk::Format::R16G16B16_SNORM => Format::R16G16B16SNorm,
            vk::Format::R16G16B16_USCALED => Format::R16G16B16UScaled,
            vk::Format::R16G16B16_SSCALED => Format::R16G16B16SScaled,
            vk::Format::R16G16B16_UINT => Format::R16G16B16UInt,
            vk::Format::R16G16B16_SINT => Format::R16G16B16SInt,
            vk::Format::R16G16B16_SFLOAT => Format::R16G16B16SFloat,
            vk::Format::R16G16B16A16_UNORM => Format::R16G16B16A16UNorm,
            vk::Format::R16G16B16A16_SNORM => Format::R16G16B16A16SNorm,
            vk::Format::R16G16B16A16_USCALED => Format::R16G16B16A16UScaled,
            vk::Format::R16G16B16A16_SSCALED => Format::R16G16B16A16SScaled,
            vk::Format::R16G16B16A16_UINT => Format::R16G16B16A16UInt,
            vk::Format::R16G16B16A16_SINT => Format::R16G16B16A16SInt,
            vk::Format::R16G16B16A16_SFLOAT => Format::R16G16B16A16SFloat,
            vk::Format::R32_UINT => Format::R32UInt,
            vk::Format::R32_SINT => Format::R32SInt,
            vk::Format::R32_SFLOAT => Format::R32SFloat,
            vk::Format::R32G32_UINT => Format::R32G32UInt,
            vk::Format::R32G32_SINT => Format::R32G32SInt,
            vk::Format::R32G32_SFLOAT => Format::R32G32SFloat,
            vk::Format::R32G32B32_UINT => Format::R32G32B32UInt,
            vk::Format::R32G32B32_SINT => Format::R32G32B32SInt,
            vk::Format::R32G32B32_SFLOAT => Format::R32G32B32SFloat,
            vk::Format::R32G32B32A32_UINT => Format::R32G32B32A32UInt,
            vk::Format::R32G32B32A32_SINT => Format::R32G32B32A32SInt,
            vk::Format::R32G32B32A32_SFLOAT => Format::R32G32B32A32SFloat,
            vk::Format::R64_UINT => Format::R64UInt,
            vk::Format::R64_SINT => Format::R64SInt,
            vk::Format::R64_SFLOAT => Format::R64SFloat,
            vk::Format::R64G64_UINT => Format::R64G64UInt,
            vk::Format::R64G64_SINT => Format::R64G64SInt,
            vk::Format::R64G64_SFLOAT => Format::R64G64SFloat,
            vk::Format::R64G64B64_UINT => Format::R64G64B64UInt,
            vk::Format::R64G64B64_SINT => Format::R64G64B64SInt,
            vk::Format::R64G64B64_SFLOAT => Format::R64G64B64SFloat,
            vk::Format::R64G64B64A64_UINT => Format::R64G64B64A64UInt,
            vk::Format::R64G64B64A64_SINT => Format::R64G64B64A64SInt,
            vk::Format::R64G64B64A64_SFLOAT => Format::R64G64B64A64SFloat,
            vk::Format::B10G11R11_UFLOAT_PACK32 => Format::B10G11R11UFloatPack32,
            vk::Format::E5B9G9R9_UFLOAT_PACK32 => Format::E5B9G9R9UFloatPack32,
            vk::Format::D16_UNORM => Format::D16UNorm,
            vk::Format::X8_D24_UNORM_PACK32 => Format::X8D24UNormPack32,
            vk::Format::D32_SFLOAT => Format::D32SFloat,
            vk::Format::S8_UINT => Format::S8UInt,
            vk::Format::D16_UNORM_S8_UINT => Format::D16UNormS8UInt,
            vk::Format::D24_UNORM_S8_UINT => Format::D24UNormS8UInt,
            vk::Format::D32_SFLOAT_S8_UINT => Format::D32SFloatS8UInt,
            vk::Format::BC1_RGB_UNORM_BLOCK => Format::BC1RGBUNormBlock,
            vk::Format::BC1_RGB_SRGB_BLOCK => Format::BC1RGBSRgbBlock,
            vk::Format::BC1_RGBA_UNORM_BLOCK => Format::BC1RGBAUNormBlock,
            vk::Format::BC1_RGBA_SRGB_BLOCK => Format::BC1RGBASRgbBlock,
            vk::Format::BC2_UNORM_BLOCK => Format::BC2UNormBlock,
            vk::Format::BC2_SRGB_BLOCK => Format::BC2SRgbBlock,
            vk::Format::BC3_UNORM_BLOCK => Format::BC3UNormBlock,
            vk::Format::BC3_SRGB_BLOCK => Format::BC3SRgbBlock,
            vk::Format::BC4_UNORM_BLOCK => Format::BC4UNormBlock,
            vk::Format::BC4_SNORM_BLOCK => Format::BC4SNormBlock,
            vk::Format::BC5_UNORM_BLOCK => Format::BC5UNormBlock,
            vk::Format::BC5_SNORM_BLOCK => Format::BC5SNormBlock,
            vk::Format::BC6H_UFLOAT_BLOCK => Format::BC6HUFloatBlock,
            vk::Format::BC6H_SFLOAT_BLOCK => Format::BC6HSFloatBlock,
            vk::Format::BC7_UNORM_BLOCK => Format::BC7UNormBlock,
            vk::Format::BC7_SRGB_BLOCK => Format::BC7SRgbBlock,
            vk::Format::ETC2_R8G8B8_UNORM_BLOCK => Format::ETC2R8G8B8UNormBlock,
            vk::Format::ETC2_R8G8B8_SRGB_BLOCK => Format::ETC2R8G8B8SRgbBlock,
            vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK => Format::ETC2R8G8B8A1UNormBlock,
            vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK => Format::ETC2R8G8B8A1SRgbBlock,
            vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK => Format::ETC2R8G8B8A8UNormBlock,
            vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK => Format::ETC2R8G8B8A8SRgbBlock,
            vk::Format::EAC_R11_UNORM_BLOCK => Format::EACR11UNormBlock,
            vk::Format::EAC_R11_SNORM_BLOCK => Format::EACR11SNormBlock,
            vk::Format::EAC_R11G11_UNORM_BLOCK => Format::EACR11G11UNormBlock,
            vk::Format::EAC_R11G11_SNORM_BLOCK => Format::EACR11G11SNormBlock,
            vk::Format::ASTC_4X4_UNORM_BLOCK => Format::ASTC4X4UNormBlock,
            vk::Format::ASTC_4X4_SRGB_BLOCK => Format::ASTC4X4SRgbBlock,
            vk::Format::ASTC_5X4_UNORM_BLOCK => Format::ASTC5X4UNormBlock,
            vk::Format::ASTC_5X4_SRGB_BLOCK => Format::ASTC5X4SRgbBlock,
            vk::Format::ASTC_5X5_UNORM_BLOCK => Format::ASTC5X5UNormBlock,
            vk::Format::ASTC_5X5_SRGB_BLOCK => Format::ASTC5X5SRgbBlock,
            vk::Format::ASTC_6X5_UNORM_BLOCK => Format::ASTC6X5UNormBlock,
            vk::Format::ASTC_6X5_SRGB_BLOCK => Format::ASTC6X5SRgbBlock,
            vk::Format::ASTC_6X6_UNORM_BLOCK => Format::ASTC6X6UNormBlock,
            vk::Format::ASTC_6X6_SRGB_BLOCK => Format::ASTC6X6SRgbBlock,
            vk::Format::ASTC_8X5_UNORM_BLOCK => Format::ASTC8X5UNormBlock,
            vk::Format::ASTC_8X5_SRGB_BLOCK => Format::ASTC8X5SRgbBlock,
            vk::Format::ASTC_8X6_UNORM_BLOCK => Format::ASTC8X6UNormBlock,
            vk::Format::ASTC_8X6_SRGB_BLOCK => Format::ASTC8X6SRgbBlock,
            vk::Format::ASTC_8X8_UNORM_BLOCK => Format::ASTC8X8UNormBlock,
            vk::Format::ASTC_8X8_SRGB_BLOCK => Format::ASTC8X8SRgbBlock,
            vk::Format::ASTC_10X5_UNORM_BLOCK => Format::ASTC10X5UNormBlock,
            vk::Format::ASTC_10X5_SRGB_BLOCK => Format::ASTC10X5SRgbBlock,
            vk::Format::ASTC_10X6_UNORM_BLOCK => Format::ASTC10X6UNormBlock,
            vk::Format::ASTC_10X6_SRGB_BLOCK => Format::ASTC10X6SRgbBlock,
            vk::Format::ASTC_10X8_UNORM_BLOCK => Format::ASTC10X8UNormBlock,
            vk::Format::ASTC_10X8_SRGB_BLOCK => Format::ASTC10X8SRgbBlock,
            vk::Format::ASTC_10X10_UNORM_BLOCK => Format::ASTC10X10UNormBlock,
            vk::Format::ASTC_10X10_SRGB_BLOCK => Format::ASTC10X10SRgbBlock,
            vk::Format::ASTC_12X10_UNORM_BLOCK => Format::ASTC12X10UNormBlock,
            vk::Format::ASTC_12X10_SRGB_BLOCK => Format::ASTC12X10SRgbBlock,
            vk::Format::ASTC_12X12_UNORM_BLOCK => Format::ASTC12X12UNormBlock,
            vk::Format::ASTC_12X12_SRGB_BLOCK => Format::ASTC12X12SRgbBlock,
            
            _ => { panic!("Encountered illegal VkFormat value '{}'", value.as_raw()) }
        }
    }
}

impl From<Format> for vk::Format {
    fn from(value: Format) -> Self {
        match value {
            Format::Undefined => vk::Format::UNDEFINED,

            Format::R4G4UNormPack8 => vk::Format::R4G4_UNORM_PACK8,
            Format::R4G4B4A4UNormPack16 => vk::Format::R4G4B4A4_UNORM_PACK16,
            Format::B4G4R4A4UNormPack16 => vk::Format::B4G4R4A4_UNORM_PACK16,
            Format::R5G6B5UNormPack16 => vk::Format::R5G6B5_UNORM_PACK16,
            Format::B5G6R5UNormPack16 => vk::Format::B5G6R5_UNORM_PACK16,
            Format::R5G5B5A1UNormPack16 => vk::Format::R5G5B5A1_UNORM_PACK16,
            Format::B5G5R5A1UNormPack16 => vk::Format::B5G5R5A1_UNORM_PACK16,
            Format::A1R5G5B5UNormPack16 => vk::Format::A1R5G5B5_UNORM_PACK16,
            Format::R8UNorm => vk::Format::R8_UNORM,
            Format::R8SNorm => vk::Format::R8_SNORM,
            Format::R8UScaled => vk::Format::R8_USCALED,
            Format::R8SScaled => vk::Format::R8_SSCALED,
            Format::R8UInt => vk::Format::R8_UINT,
            Format::R8SInt => vk::Format::R8_SINT,
            Format::R8SRgb => vk::Format::R8_SRGB,
            Format::R8G8UNorm => vk::Format::R8G8_UNORM,
            Format::R8G8SNorm => vk::Format::R8G8_SNORM,
            Format::R8G8UScaled => vk::Format::R8G8_USCALED,
            Format::R8G8SScaled => vk::Format::R8G8_SSCALED,
            Format::R8G8UInt => vk::Format::R8G8_UINT,
            Format::R8G8SInt => vk::Format::R8G8_SINT,
            Format::R8G8SRgb => vk::Format::R8G8_SRGB,
            Format::R8G8B8UNorm => vk::Format::R8G8B8_UNORM,
            Format::R8G8B8SNorm => vk::Format::R8G8B8_SNORM,
            Format::R8G8B8UScaled => vk::Format::R8G8B8_USCALED,
            Format::R8G8B8SScaled => vk::Format::R8G8B8_SSCALED,
            Format::R8G8B8UInt => vk::Format::R8G8B8_UINT,
            Format::R8G8B8SInt => vk::Format::R8G8B8_SINT,
            Format::R8G8B8SRgb => vk::Format::R8G8B8_SRGB,
            Format::B8G8R8UNorm => vk::Format::B8G8R8_UNORM,
            Format::B8G8R8SNorm => vk::Format::B8G8R8_SNORM,
            Format::B8G8R8UScaled => vk::Format::B8G8R8_USCALED,
            Format::B8G8R8SScaled => vk::Format::B8G8R8_SSCALED,
            Format::B8G8R8UInt => vk::Format::B8G8R8_UINT,
            Format::B8G8R8SInt => vk::Format::B8G8R8_SINT,
            Format::B8G8R8SRgb => vk::Format::B8G8R8_SRGB,
            Format::R8G8B8A8UNorm => vk::Format::R8G8B8A8_UNORM,
            Format::R8G8B8A8SNorm => vk::Format::R8G8B8A8_SNORM,
            Format::R8G8B8A8UScaled => vk::Format::R8G8B8A8_USCALED,
            Format::R8G8B8A8SScaled => vk::Format::R8G8B8A8_SSCALED,
            Format::R8G8B8A8UInt => vk::Format::R8G8B8A8_UINT,
            Format::R8G8B8A8SInt => vk::Format::R8G8B8A8_SINT,
            Format::R8G8B8A8SRgb => vk::Format::R8G8B8A8_SRGB,
            Format::B8G8R8A8UNorm => vk::Format::B8G8R8A8_UNORM,
            Format::B8G8R8A8SNorm => vk::Format::B8G8R8A8_SNORM,
            Format::B8G8R8A8UScaled => vk::Format::B8G8R8A8_USCALED,
            Format::B8G8R8A8SScaled => vk::Format::B8G8R8A8_SSCALED,
            Format::B8G8R8A8UInt => vk::Format::B8G8R8A8_UINT,
            Format::B8G8R8A8SInt => vk::Format::B8G8R8A8_SINT,
            Format::B8G8R8A8SRgb => vk::Format::B8G8R8A8_SRGB,
            Format::A8B8G8R8UNormPack32 => vk::Format::A8B8G8R8_UNORM_PACK32,
            Format::A8B8G8R8SNormPack32 => vk::Format::A8B8G8R8_SNORM_PACK32,
            Format::A8B8G8R8UScaledPack32 => vk::Format::A8B8G8R8_USCALED_PACK32,
            Format::A8B8G8R8SScaledPack32 => vk::Format::A8B8G8R8_SSCALED_PACK32,
            Format::A8B8G8R8UIntPack32 => vk::Format::A8B8G8R8_UINT_PACK32,
            Format::A8B8G8R8SIntPack32 => vk::Format::A8B8G8R8_SINT_PACK32,
            Format::A8B8G8R8SRgbPack32 => vk::Format::A8B8G8R8_SRGB_PACK32,
            Format::A2R10G10B10UNormPack32 => vk::Format::A2R10G10B10_UNORM_PACK32,
            Format::A2R10G10B10SNormPack32 => vk::Format::A2R10G10B10_SNORM_PACK32,
            Format::A2R10G10B10UScaledPack32 => vk::Format::A2R10G10B10_USCALED_PACK32,
            Format::A2R10G10B10SScaledPack32 => vk::Format::A2R10G10B10_SSCALED_PACK32,
            Format::A2R10G10B10UIntPack32 => vk::Format::A2R10G10B10_UINT_PACK32,
            Format::A2R10G10B10SIntPack32 => vk::Format::A2R10G10B10_SINT_PACK32,
            Format::A2B10G10R10UNormPack32 => vk::Format::A2B10G10R10_UNORM_PACK32,
            Format::A2B10G10R10SNormPack32 => vk::Format::A2B10G10R10_SNORM_PACK32,
            Format::A2B10G10R10UScaledPack32 => vk::Format::A2B10G10R10_USCALED_PACK32,
            Format::A2B10G10R10SScaledPack32 => vk::Format::A2B10G10R10_SSCALED_PACK32,
            Format::A2B10G10R10UIntPack32 => vk::Format::A2B10G10R10_UINT_PACK32,
            Format::A2B10G10R10SIntPack32 => vk::Format::A2B10G10R10_SINT_PACK32,
            Format::R16UNorm => vk::Format::R16_UNORM,
            Format::R16SNorm => vk::Format::R16_SNORM,
            Format::R16UScaled => vk::Format::R16_USCALED,
            Format::R16SScaled => vk::Format::R16_SSCALED,
            Format::R16UInt => vk::Format::R16_UINT,
            Format::R16SInt => vk::Format::R16_SINT,
            Format::R16SFloat => vk::Format::R16_SFLOAT,
            Format::R16G16UNorm => vk::Format::R16G16_UNORM,
            Format::R16G16SNorm => vk::Format::R16G16_SNORM,
            Format::R16G16UScaled => vk::Format::R16G16_USCALED,
            Format::R16G16SScaled => vk::Format::R16G16_SSCALED,
            Format::R16G16UInt => vk::Format::R16G16_UINT,
            Format::R16G16SInt => vk::Format::R16G16_SINT,
            Format::R16G16SFloat => vk::Format::R16G16_SFLOAT,
            Format::R16G16B16UNorm => vk::Format::R16G16B16_UNORM,
            Format::R16G16B16SNorm => vk::Format::R16G16B16_SNORM,
            Format::R16G16B16UScaled => vk::Format::R16G16B16_USCALED,
            Format::R16G16B16SScaled => vk::Format::R16G16B16_SSCALED,
            Format::R16G16B16UInt => vk::Format::R16G16B16_UINT,
            Format::R16G16B16SInt => vk::Format::R16G16B16_SINT,
            Format::R16G16B16SFloat => vk::Format::R16G16B16_SFLOAT,
            Format::R16G16B16A16UNorm => vk::Format::R16G16B16A16_UNORM,
            Format::R16G16B16A16SNorm => vk::Format::R16G16B16A16_SNORM,
            Format::R16G16B16A16UScaled => vk::Format::R16G16B16A16_USCALED,
            Format::R16G16B16A16SScaled => vk::Format::R16G16B16A16_SSCALED,
            Format::R16G16B16A16UInt => vk::Format::R16G16B16A16_UINT,
            Format::R16G16B16A16SInt => vk::Format::R16G16B16A16_SINT,
            Format::R16G16B16A16SFloat => vk::Format::R16G16B16A16_SFLOAT,
            Format::R32UInt => vk::Format::R32_UINT,
            Format::R32SInt => vk::Format::R32_SINT,
            Format::R32SFloat => vk::Format::R32_SFLOAT,
            Format::R32G32UInt => vk::Format::R32G32_UINT,
            Format::R32G32SInt => vk::Format::R32G32_SINT,
            Format::R32G32SFloat => vk::Format::R32G32_SFLOAT,
            Format::R32G32B32UInt => vk::Format::R32G32B32_UINT,
            Format::R32G32B32SInt => vk::Format::R32G32B32_SINT,
            Format::R32G32B32SFloat => vk::Format::R32G32B32_SFLOAT,
            Format::R32G32B32A32UInt => vk::Format::R32G32B32A32_UINT,
            Format::R32G32B32A32SInt => vk::Format::R32G32B32A32_SINT,
            Format::R32G32B32A32SFloat => vk::Format::R32G32B32A32_SFLOAT,
            Format::R64UInt => vk::Format::R64_UINT,
            Format::R64SInt => vk::Format::R64_SINT,
            Format::R64SFloat => vk::Format::R64_SFLOAT,
            Format::R64G64UInt => vk::Format::R64G64_UINT,
            Format::R64G64SInt => vk::Format::R64G64_SINT,
            Format::R64G64SFloat => vk::Format::R64G64_SFLOAT,
            Format::R64G64B64UInt => vk::Format::R64G64B64_UINT,
            Format::R64G64B64SInt => vk::Format::R64G64B64_SINT,
            Format::R64G64B64SFloat => vk::Format::R64G64B64_SFLOAT,
            Format::R64G64B64A64UInt => vk::Format::R64G64B64A64_UINT,
            Format::R64G64B64A64SInt => vk::Format::R64G64B64A64_SINT,
            Format::R64G64B64A64SFloat => vk::Format::R64G64B64A64_SFLOAT,
            Format::B10G11R11UFloatPack32 => vk::Format::B10G11R11_UFLOAT_PACK32,
            Format::E5B9G9R9UFloatPack32 => vk::Format::E5B9G9R9_UFLOAT_PACK32,
            Format::D16UNorm => vk::Format::D16_UNORM,
            Format::X8D24UNormPack32 => vk::Format::X8_D24_UNORM_PACK32,
            Format::D32SFloat => vk::Format::D32_SFLOAT,
            Format::S8UInt => vk::Format::S8_UINT,
            Format::D16UNormS8UInt => vk::Format::D16_UNORM_S8_UINT,
            Format::D24UNormS8UInt => vk::Format::D24_UNORM_S8_UINT,
            Format::D32SFloatS8UInt => vk::Format::D32_SFLOAT_S8_UINT,
            Format::BC1RGBUNormBlock => vk::Format::BC1_RGB_UNORM_BLOCK,
            Format::BC1RGBSRgbBlock => vk::Format::BC1_RGB_SRGB_BLOCK,
            Format::BC1RGBAUNormBlock => vk::Format::BC1_RGBA_UNORM_BLOCK,
            Format::BC1RGBASRgbBlock => vk::Format::BC1_RGBA_SRGB_BLOCK,
            Format::BC2UNormBlock => vk::Format::BC2_UNORM_BLOCK,
            Format::BC2SRgbBlock => vk::Format::BC2_SRGB_BLOCK,
            Format::BC3UNormBlock => vk::Format::BC3_UNORM_BLOCK,
            Format::BC3SRgbBlock => vk::Format::BC3_SRGB_BLOCK,
            Format::BC4UNormBlock => vk::Format::BC4_UNORM_BLOCK,
            Format::BC4SNormBlock => vk::Format::BC4_SNORM_BLOCK,
            Format::BC5UNormBlock => vk::Format::BC5_UNORM_BLOCK,
            Format::BC5SNormBlock => vk::Format::BC5_SNORM_BLOCK,
            Format::BC6HUFloatBlock => vk::Format::BC6H_UFLOAT_BLOCK,
            Format::BC6HSFloatBlock => vk::Format::BC6H_SFLOAT_BLOCK,
            Format::BC7UNormBlock => vk::Format::BC7_UNORM_BLOCK,
            Format::BC7SRgbBlock => vk::Format::BC7_SRGB_BLOCK,
            Format::ETC2R8G8B8UNormBlock => vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
            Format::ETC2R8G8B8SRgbBlock => vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
            Format::ETC2R8G8B8A1UNormBlock => vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK,
            Format::ETC2R8G8B8A1SRgbBlock => vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
            Format::ETC2R8G8B8A8UNormBlock => vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK,
            Format::ETC2R8G8B8A8SRgbBlock => vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
            Format::EACR11UNormBlock => vk::Format::EAC_R11_UNORM_BLOCK,
            Format::EACR11SNormBlock => vk::Format::EAC_R11_SNORM_BLOCK,
            Format::EACR11G11UNormBlock => vk::Format::EAC_R11G11_UNORM_BLOCK,
            Format::EACR11G11SNormBlock => vk::Format::EAC_R11G11_SNORM_BLOCK,
            Format::ASTC4X4UNormBlock => vk::Format::ASTC_4X4_UNORM_BLOCK,
            Format::ASTC4X4SRgbBlock => vk::Format::ASTC_4X4_SRGB_BLOCK,
            Format::ASTC5X4UNormBlock => vk::Format::ASTC_5X4_UNORM_BLOCK,
            Format::ASTC5X4SRgbBlock => vk::Format::ASTC_5X4_SRGB_BLOCK,
            Format::ASTC5X5UNormBlock => vk::Format::ASTC_5X5_UNORM_BLOCK,
            Format::ASTC5X5SRgbBlock => vk::Format::ASTC_5X5_SRGB_BLOCK,
            Format::ASTC6X5UNormBlock => vk::Format::ASTC_6X5_UNORM_BLOCK,
            Format::ASTC6X5SRgbBlock => vk::Format::ASTC_6X5_SRGB_BLOCK,
            Format::ASTC6X6UNormBlock => vk::Format::ASTC_6X6_UNORM_BLOCK,
            Format::ASTC6X6SRgbBlock => vk::Format::ASTC_6X6_SRGB_BLOCK,
            Format::ASTC8X5UNormBlock => vk::Format::ASTC_8X5_UNORM_BLOCK,
            Format::ASTC8X5SRgbBlock => vk::Format::ASTC_8X5_SRGB_BLOCK,
            Format::ASTC8X6UNormBlock => vk::Format::ASTC_8X6_UNORM_BLOCK,
            Format::ASTC8X6SRgbBlock => vk::Format::ASTC_8X6_SRGB_BLOCK,
            Format::ASTC8X8UNormBlock => vk::Format::ASTC_8X8_UNORM_BLOCK,
            Format::ASTC8X8SRgbBlock => vk::Format::ASTC_8X8_SRGB_BLOCK,
            Format::ASTC10X5UNormBlock => vk::Format::ASTC_10X5_UNORM_BLOCK,
            Format::ASTC10X5SRgbBlock => vk::Format::ASTC_10X5_SRGB_BLOCK,
            Format::ASTC10X6UNormBlock => vk::Format::ASTC_10X6_UNORM_BLOCK,
            Format::ASTC10X6SRgbBlock => vk::Format::ASTC_10X6_SRGB_BLOCK,
            Format::ASTC10X8UNormBlock => vk::Format::ASTC_10X8_UNORM_BLOCK,
            Format::ASTC10X8SRgbBlock => vk::Format::ASTC_10X8_SRGB_BLOCK,
            Format::ASTC10X10UNormBlock => vk::Format::ASTC_10X10_UNORM_BLOCK,
            Format::ASTC10X10SRgbBlock => vk::Format::ASTC_10X10_SRGB_BLOCK,
            Format::ASTC12X10UNormBlock => vk::Format::ASTC_12X10_UNORM_BLOCK,
            Format::ASTC12X10SRgbBlock => vk::Format::ASTC_12X10_SRGB_BLOCK,
            Format::ASTC12X12UNormBlock => vk::Format::ASTC_12X12_UNORM_BLOCK,
            Format::ASTC12X12SRgbBlock => vk::Format::ASTC_12X12_SRGB_BLOCK,
        }
    }
}



/// Defines how we might use an Image.
#[derive(Clone, Copy, Debug)]
pub enum ImageAspect {
    /// The image will be used as a colour attachment.
    Colour,
    /// The image will be used as a Depth stencil.
    Depth,
    /// The image will be used as a gemeral stencil.
    Stencil,
    /// The image will be used to carry metadata.
    Metadata,
}

impl Default for ImageAspect {
    #[inline]
    fn default() -> Self {
        ImageAspect::Colour
    }
}

impl Display for ImageAspect {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ImageAspect::*;
        match self {
            Colour   => write!(f, "Colour"),
            Depth    => write!(f, "Depth"),
            Stencil  => write!(f, "Stencil"),
            Metadata => write!(f, "Metadata"),
        }
    }
}

impl From<vk::ImageAspectFlags> for ImageAspect {
    fn from(value: vk::ImageAspectFlags) -> Self {
        match value {
            vk::ImageAspectFlags::COLOR    => ImageAspect::Colour,
            vk::ImageAspectFlags::DEPTH    => ImageAspect::Depth,
            vk::ImageAspectFlags::STENCIL  => ImageAspect::Stencil,
            vk::ImageAspectFlags::METADATA => ImageAspect::Metadata,
            _                              => { panic!("Encountered VkImageAspectFlags with illegal value '{}'", value.as_raw()) }
        }
    }
}

impl From<ImageAspect> for vk::ImageAspectFlags {
    fn from(value: ImageAspect) -> Self {
        match value {
            ImageAspect::Colour   => vk::ImageAspectFlags::COLOR,
            ImageAspect::Depth    => vk::ImageAspectFlags::DEPTH,
            ImageAspect::Stencil  => vk::ImageAspectFlags::STENCIL,
            ImageAspect::Metadata => vk::ImageAspectFlags::METADATA,
        }
    }
}



/// Defines any potential re-mapping of an image's channels.
#[derive(Debug, Clone)]
pub struct ComponentSwizzle {
    /// The mapping of the red channel
    pub red   : vk::ComponentSwizzle,
    /// The mapping of the green channel
    pub green : vk::ComponentSwizzle,
    /// The mapping of the blue channel
    pub blue  : vk::ComponentSwizzle,
    /// The mapping of the alpha channel
    pub alpha : vk::ComponentSwizzle,
}

impl Default for ComponentSwizzle {
    fn default() -> Self {
        Self {
            red   : vk::ComponentSwizzle::IDENTITY,
            green : vk::ComponentSwizzle::IDENTITY,
            blue  : vk::ComponentSwizzle::IDENTITY,
            alpha : vk::ComponentSwizzle::IDENTITY,
        }
    }
}

impl From<vk::ComponentMapping> for ComponentSwizzle {
    #[inline]
    fn from(value: vk::ComponentMapping) -> Self {
        Self {
            red   : value.r,
            green : value.g,
            blue  : value.b,
            alpha : value.a,
        }
    }
}

impl From<ComponentSwizzle> for vk::ComponentMapping {
    #[inline]
    fn from(value: ComponentSwizzle) -> Self {
        Self {
            r : value.red,
            g : value.green,
            b : value.blue,
            a : value.alpha,
        }
    }
}
