/* AUXILLARY.rs
 *   by Lut99
 *
 * Created:
 *   18 Apr 2022, 12:27:51
 * Last edited:
 *   09 Jul 2022, 13:03:32
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
use std::rc::Rc;
use std::slice;
use std::str::FromStr;

use ash::vk;

pub use crate::errors::{AttributeLayoutError, ExtensionError, QueueError};
use crate::instance::Instance;


/***** MEMORY POOLS *****/
/// Defines the memory requirements of a buffer or image.
#[derive(Clone, Debug)]
pub struct MemoryRequirements {
    /// The minimum size of the required memory block.
    pub size  : usize,
    /// The alignment (in bytes) of the start of the required memory block. Must be a multiple of two.
    pub align : u8,
    /// The device memory types that are supported by the buffer or image for this particular usage.
    pub types : DeviceMemoryTypeFlags,
}

impl From<vk::MemoryRequirements> for MemoryRequirements {
    #[inline]
    fn from(value: vk::MemoryRequirements) -> Self {
        Self {
            size  : value.size as usize,
            align : value.alignment as u8,
            types : value.memory_type_bits.into(),
        }
    }
}

impl From<MemoryRequirements> for vk::MemoryRequirements {
    #[inline]
    fn from(value: MemoryRequirements) -> Self {
        Self {
            size             : value.size as vk::DeviceSize,
            alignment        : value.align as vk::DeviceSize,
            memory_type_bits : value.types.into(),
        }
    }
}



/// Define a single type of memory that a device has to offer.
/// 
/// Note: because the actual list is device-dependent, there are no constants available for this "enum" implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceMemoryType(u32);

impl Display for DeviceMemoryType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for DeviceMemoryType {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<DeviceMemoryType> for u32 {
    #[inline]
    fn from(value: DeviceMemoryType) -> Self {
        value.0
    }
}

impl From<DeviceMemoryTypeFlags> for DeviceMemoryType {
    fn from(value: DeviceMemoryTypeFlags) -> Self {
        // Sanity check that it has only one value set
        if value.0.count_ones() != 1 { panic!("Cannot cast a DeviceMemoryTypeFlags to a DeviceMemoryType if it has less or more than one flags set"); }
        Self(value.0)
    }
}

impl From<DeviceMemoryType> for DeviceMemoryTypeFlags {
    #[inline]
    fn from(value: DeviceMemoryType) -> Self {
        Self(value.0)
    }
}



/// Define a multiple types of memory that a device has to offer.
/// 
/// Note: because the actual list is device-dependent, there are no constants available for this "flags" implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DeviceMemoryTypeFlags(u32);

impl DeviceMemoryTypeFlags {
    /// A DeviceMemoryTypeFlags struct with _all_ memory types.
    pub const ALL: Self   = Self(!0);
    /// An empty DeviceMemoryTypeFlags struct.
    pub const EMPTY: Self = Self(0);

    /// Checks if this DeviceMemoryTypeFlags is a superset of the given one.
    #[inline]
    pub fn check<T: Into<u32>>(&self, other: T) -> bool { let other = other.into(); (self.0 & other) == other }
}

impl Display for DeviceMemoryTypeFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Construct a list
        let mut first = true;
        let mut i: u32 = 0x1;
        while i != 0 {
            // Check if this property is enabled
            if self.0 & i != 0 {
                // Write the comma if necessary
                if first { first = false; }
                else { write!(f, ", ")?; }

                // Write the name of this property
                write!(f, "{}", self.0)?;
            }

            // Increment the i
            i = i << 1;
        }

        // Done
        Ok(())
    }
}

impl BitOr for DeviceMemoryTypeFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for DeviceMemoryTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

impl From<u32> for DeviceMemoryTypeFlags {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<DeviceMemoryTypeFlags> for u32 {
    #[inline]
    fn from(value: DeviceMemoryTypeFlags) -> Self {
        value.0
    }
}



/// The BufferUsageFlags that determine what we can use a buffer for.
#[derive(Clone, Copy, Debug)]
pub struct BufferUsageFlags(u16);

impl BufferUsageFlags {
    /// Defines no flags
    pub const EMPTY: Self = Self(0x0000);
    /// Defines all flags
    pub const ALL: Self = Self(0xFFFF);

    /// The buffer may be used as a source buffer in a memory transfer operation.
    pub const TRANSFER_SRC: Self = Self(0x0001);
    /// The buffer may be used as a target buffer in a memory transfer operation.
    pub const TRANSFER_DST: Self = Self(0x0002);
    /// The buffer may be used as a uniform texel buffer.
    /// 
    /// Uniform buffers are much smaller but slightly faster than storage buffers.
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0x0004);
    /// The buffer may be used as a storage texel buffer.
    /// 
    /// Storage buffers are much larger but slightly slower than uniform buffers.
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0x0008);
    /// The buffer may be used as a uniform buffer.
    /// 
    /// Uniform buffers are much smaller but slightly faster than storage buffers.
    pub const UNIFORM_BUFFER: Self = Self(0x0010);
    /// The buffer may be used as a storage buffer.
    /// 
    /// Storage buffers are much larger but slightly slower than uniform buffers.
    pub const STORAGE_BUFFER: Self = Self(0x0020);
    /// The buffer may be used to storage indices.
    pub const INDEX_BUFFER: Self = Self(0x0040);
    /// The buffer may be used to storage vertices.
    pub const VERTEX_BUFFER: Self = Self(0x0080);
    /// The buffer may be used for indirect draw commands (various applications).
    pub const INDIRECT_BUFFER: Self = Self(0x0100);



    /// Creates a BufferUsageFlags from a raw value.
    /// 
    /// # Arguments
    /// - `value`: The raw flags value. Note that this is the BufferUsageFlags raw, _not_ the Vulkan raw.
    /// 
    /// # Returns
    /// A new BufferUsageFlags with the raw flags set.
    pub const fn raw(value: u16) -> Self { Self(value) }

    /// Checks if this BufferUsageFlags is a superset of the given one. For example, if this is `DEVICE_LOCAL | HOST_VISIBLE` and the given one is `DEVICE_LOCAL`, returns true.
    #[inline]
    pub fn check(&self, other: BufferUsageFlags) -> bool { (self.0 & other.0) == other.0 }
}

impl BitOr for BufferUsageFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for BufferUsageFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

impl From<vk::BufferUsageFlags> for BufferUsageFlags {
    fn from(value: vk::BufferUsageFlags) -> Self {
        // Construct one-by-one to maintain compatibility
        let mut result = Self::EMPTY;
        if (value & vk::BufferUsageFlags::TRANSFER_SRC).as_raw() != 0 { result |= BufferUsageFlags::TRANSFER_SRC; }
        if (value & vk::BufferUsageFlags::TRANSFER_DST).as_raw() != 0 { result |= BufferUsageFlags::TRANSFER_DST; }
        if (value & vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER).as_raw() != 0 { result |= BufferUsageFlags::UNIFORM_TEXEL_BUFFER; }
        if (value & vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER).as_raw() != 0 { result |= BufferUsageFlags::STORAGE_TEXEL_BUFFER; }
        if (value & vk::BufferUsageFlags::UNIFORM_BUFFER).as_raw() != 0 { result |= BufferUsageFlags::UNIFORM_BUFFER; }
        if (value & vk::BufferUsageFlags::STORAGE_BUFFER).as_raw() != 0 { result |= BufferUsageFlags::STORAGE_BUFFER; }
        result
    }
}

impl From<BufferUsageFlags> for vk::BufferUsageFlags {
    fn from(value: BufferUsageFlags) -> Self {
        // Construct one-by-one to maintain compatibility
        let mut result = Self::empty();
        if value.check(BufferUsageFlags::TRANSFER_SRC) { result |= vk::BufferUsageFlags::TRANSFER_SRC; }
        if value.check(BufferUsageFlags::TRANSFER_DST) { result |= vk::BufferUsageFlags::TRANSFER_DST; }
        if value.check(BufferUsageFlags::UNIFORM_TEXEL_BUFFER) { result |= vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER; }
        if value.check(BufferUsageFlags::STORAGE_TEXEL_BUFFER) { result |= vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER; }
        if value.check(BufferUsageFlags::UNIFORM_BUFFER) { result |= vk::BufferUsageFlags::UNIFORM_BUFFER; }
        if value.check(BufferUsageFlags::STORAGE_BUFFER) { result |= vk::BufferUsageFlags::STORAGE_BUFFER; }
        if value.check(BufferUsageFlags::INDEX_BUFFER) { result |= vk::BufferUsageFlags::INDEX_BUFFER; }
        if value.check(BufferUsageFlags::VERTEX_BUFFER) { result |= vk::BufferUsageFlags::VERTEX_BUFFER; }
        if value.check(BufferUsageFlags::INDIRECT_BUFFER) { result |= vk::BufferUsageFlags::INDIRECT_BUFFER; }
        result
    }
}



/// Determines how a Buffer may be accessed.
#[derive(Clone, Debug)]
pub enum SharingMode {
    /// The buffer may be accessed by one queue family only. First come, first serve.
    Exclusive,
    /// The buffer may be accessed by multiple queue families. The queues have to be specified, though, as a list of queue family indices.
    Concurrent(Vec<u32>),
}

impl SharingMode {
    /// Construct the SharingMode from a given VkSharingMode and a (possible) list of concurrent queue family indices.
    #[inline]
    pub fn from_vk(sharing_mode: vk::SharingMode, queue_family_indices: Option<&[u32]>) -> Self {
        // Simply match the sharing mode
        match sharing_mode {
            vk::SharingMode::EXCLUSIVE  => SharingMode::Exclusive,
            vk::SharingMode::CONCURRENT => SharingMode::Concurrent(queue_family_indices.expect("Cannot set SharingMode to SharingMode::Concurrent without specifying a list of allowed queue family indices").into()),
            sharing_mode                => { panic!("Encountered illegal VkSharingMode value '{}'", sharing_mode.as_raw()); }
        }
    }
}

impl From<SharingMode> for (vk::SharingMode, Option<Vec<u32>>) {
    #[inline]
    fn from(value: SharingMode) -> Self {
        match value {
            SharingMode::Exclusive           => (vk::SharingMode::EXCLUSIVE, None),
            SharingMode::Concurrent(indices) => (vk::SharingMode::CONCURRENT, Some(indices)),
        }
    }
}



/// Determines the kind of memory allocator supported by the MemoryPool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryAllocatorKind {
    /// Defines a "normal" allocator, that is reasonably space efficient but not so much time-wise.
    Dense,
    /// Defines a linear allocator, which is fast in allocation but which will not re-use deallocated buffer space.
    /// 
    /// The index in this linear allocator is referencing some block that was allocated beforehand.
    Linear(u64),
}

impl Default for MemoryAllocatorKind {
    fn default() -> Self { MemoryAllocatorKind::Dense }
}

impl Display for MemoryAllocatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use MemoryAllocatorKind::*;
        match self {
            Dense      => write!(f, "Dense"),
            Linear(id) => write!(f, "Linear({})", id),
        }
    }
}



/// An auxillary struct that describes the memory requirements and properties of a given Buffer.
#[derive(Clone, Debug)]
pub struct BufferAllocateInfo {
    /// The usage flags of this Buffer
    pub usage_flags  : BufferUsageFlags,
    /// The sharing mode that determines how this Buffer may be accessed.
    pub sharing_mode : SharingMode,

    /// The size of the buffer, in bytes. Note that the resulting *actual* size may be slightly more based on alignment requirements.
    pub size         : usize,
    /// The additional properties that we require of the memory behind this buffer.
    pub memory_props : MemoryPropertyFlags,

    /// The type of allocator to use for this buffer.
    pub allocator : MemoryAllocatorKind,
}





/***** COMMAND POOLS *****/
/// Possible levels for a CommandBuffer.
#[derive(Clone, Copy, Debug)]
pub enum CommandBufferLevel {
    /// The command buffer is primary, i.e., only able to be submitted to a queue.
    Primary,
    /// The command buffer is secondary, i.e., only able to be called from another (primary) command buffer.
    Secondary,
}

impl From<vk::CommandBufferLevel> for CommandBufferLevel {
    #[inline]
    fn from(value: vk::CommandBufferLevel) -> Self {
        match value {
            vk::CommandBufferLevel::PRIMARY   => CommandBufferLevel::Primary,
            vk::CommandBufferLevel::SECONDARY => CommandBufferLevel::Secondary,

            value => { panic!("Encountered illegal VkCommandBufferLevel value '{}'", value.as_raw()); }
        }
    }
}

impl From<CommandBufferLevel> for vk::CommandBufferLevel {
    #[inline]
    fn from(value: CommandBufferLevel) -> Self {
        match value {
            CommandBufferLevel::Primary   => vk::CommandBufferLevel::PRIMARY,
            CommandBufferLevel::Secondary => vk::CommandBufferLevel::SECONDARY,
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
pub enum ImageFormat {
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

impl Default for ImageFormat {
    #[inline]
    fn default() -> Self {
        ImageFormat::B8G8R8A8SRgb
    }
}

impl Display for ImageFormat {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use ImageFormat::*;
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

impl From<vk::Format> for ImageFormat {
    fn from(value: vk::Format) -> Self {
        match value {
            vk::Format::UNDEFINED => ImageFormat::Undefined,

            vk::Format::R4G4_UNORM_PACK8 => ImageFormat::R4G4UNormPack8,
            vk::Format::R4G4B4A4_UNORM_PACK16 => ImageFormat::R4G4B4A4UNormPack16,
            vk::Format::B4G4R4A4_UNORM_PACK16 => ImageFormat::B4G4R4A4UNormPack16,
            vk::Format::R5G6B5_UNORM_PACK16 => ImageFormat::R5G6B5UNormPack16,
            vk::Format::B5G6R5_UNORM_PACK16 => ImageFormat::B5G6R5UNormPack16,
            vk::Format::R5G5B5A1_UNORM_PACK16 => ImageFormat::R5G5B5A1UNormPack16,
            vk::Format::B5G5R5A1_UNORM_PACK16 => ImageFormat::B5G5R5A1UNormPack16,
            vk::Format::A1R5G5B5_UNORM_PACK16 => ImageFormat::A1R5G5B5UNormPack16,
            vk::Format::R8_UNORM => ImageFormat::R8UNorm,
            vk::Format::R8_SNORM => ImageFormat::R8SNorm,
            vk::Format::R8_USCALED => ImageFormat::R8UScaled,
            vk::Format::R8_SSCALED => ImageFormat::R8SScaled,
            vk::Format::R8_UINT => ImageFormat::R8UInt,
            vk::Format::R8_SINT => ImageFormat::R8SInt,
            vk::Format::R8_SRGB => ImageFormat::R8SRgb,
            vk::Format::R8G8_UNORM => ImageFormat::R8G8UNorm,
            vk::Format::R8G8_SNORM => ImageFormat::R8G8SNorm,
            vk::Format::R8G8_USCALED => ImageFormat::R8G8UScaled,
            vk::Format::R8G8_SSCALED => ImageFormat::R8G8SScaled,
            vk::Format::R8G8_UINT => ImageFormat::R8G8UInt,
            vk::Format::R8G8_SINT => ImageFormat::R8G8SInt,
            vk::Format::R8G8_SRGB => ImageFormat::R8G8SRgb,
            vk::Format::R8G8B8_UNORM => ImageFormat::R8G8B8UNorm,
            vk::Format::R8G8B8_SNORM => ImageFormat::R8G8B8SNorm,
            vk::Format::R8G8B8_USCALED => ImageFormat::R8G8B8UScaled,
            vk::Format::R8G8B8_SSCALED => ImageFormat::R8G8B8SScaled,
            vk::Format::R8G8B8_UINT => ImageFormat::R8G8B8UInt,
            vk::Format::R8G8B8_SINT => ImageFormat::R8G8B8SInt,
            vk::Format::R8G8B8_SRGB => ImageFormat::R8G8B8SRgb,
            vk::Format::B8G8R8_UNORM => ImageFormat::B8G8R8UNorm,
            vk::Format::B8G8R8_SNORM => ImageFormat::B8G8R8SNorm,
            vk::Format::B8G8R8_USCALED => ImageFormat::B8G8R8UScaled,
            vk::Format::B8G8R8_SSCALED => ImageFormat::B8G8R8SScaled,
            vk::Format::B8G8R8_UINT => ImageFormat::B8G8R8UInt,
            vk::Format::B8G8R8_SINT => ImageFormat::B8G8R8SInt,
            vk::Format::B8G8R8_SRGB => ImageFormat::B8G8R8SRgb,
            vk::Format::R8G8B8A8_UNORM => ImageFormat::R8G8B8A8UNorm,
            vk::Format::R8G8B8A8_SNORM => ImageFormat::R8G8B8A8SNorm,
            vk::Format::R8G8B8A8_USCALED => ImageFormat::R8G8B8A8UScaled,
            vk::Format::R8G8B8A8_SSCALED => ImageFormat::R8G8B8A8SScaled,
            vk::Format::R8G8B8A8_UINT => ImageFormat::R8G8B8A8UInt,
            vk::Format::R8G8B8A8_SINT => ImageFormat::R8G8B8A8SInt,
            vk::Format::R8G8B8A8_SRGB => ImageFormat::R8G8B8A8SRgb,
            vk::Format::B8G8R8A8_UNORM => ImageFormat::B8G8R8A8UNorm,
            vk::Format::B8G8R8A8_SNORM => ImageFormat::B8G8R8A8SNorm,
            vk::Format::B8G8R8A8_USCALED => ImageFormat::B8G8R8A8UScaled,
            vk::Format::B8G8R8A8_SSCALED => ImageFormat::B8G8R8A8SScaled,
            vk::Format::B8G8R8A8_UINT => ImageFormat::B8G8R8A8UInt,
            vk::Format::B8G8R8A8_SINT => ImageFormat::B8G8R8A8SInt,
            vk::Format::B8G8R8A8_SRGB => ImageFormat::B8G8R8A8SRgb,
            vk::Format::A8B8G8R8_UNORM_PACK32 => ImageFormat::A8B8G8R8UNormPack32,
            vk::Format::A8B8G8R8_SNORM_PACK32 => ImageFormat::A8B8G8R8SNormPack32,
            vk::Format::A8B8G8R8_USCALED_PACK32 => ImageFormat::A8B8G8R8UScaledPack32,
            vk::Format::A8B8G8R8_SSCALED_PACK32 => ImageFormat::A8B8G8R8SScaledPack32,
            vk::Format::A8B8G8R8_UINT_PACK32 => ImageFormat::A8B8G8R8UIntPack32,
            vk::Format::A8B8G8R8_SINT_PACK32 => ImageFormat::A8B8G8R8SIntPack32,
            vk::Format::A8B8G8R8_SRGB_PACK32 => ImageFormat::A8B8G8R8SRgbPack32,
            vk::Format::A2R10G10B10_UNORM_PACK32 => ImageFormat::A2R10G10B10UNormPack32,
            vk::Format::A2R10G10B10_SNORM_PACK32 => ImageFormat::A2R10G10B10SNormPack32,
            vk::Format::A2R10G10B10_USCALED_PACK32 => ImageFormat::A2R10G10B10UScaledPack32,
            vk::Format::A2R10G10B10_SSCALED_PACK32 => ImageFormat::A2R10G10B10SScaledPack32,
            vk::Format::A2R10G10B10_UINT_PACK32 => ImageFormat::A2R10G10B10UIntPack32,
            vk::Format::A2R10G10B10_SINT_PACK32 => ImageFormat::A2R10G10B10SIntPack32,
            vk::Format::A2B10G10R10_UNORM_PACK32 => ImageFormat::A2B10G10R10UNormPack32,
            vk::Format::A2B10G10R10_SNORM_PACK32 => ImageFormat::A2B10G10R10SNormPack32,
            vk::Format::A2B10G10R10_USCALED_PACK32 => ImageFormat::A2B10G10R10UScaledPack32,
            vk::Format::A2B10G10R10_SSCALED_PACK32 => ImageFormat::A2B10G10R10SScaledPack32,
            vk::Format::A2B10G10R10_UINT_PACK32 => ImageFormat::A2B10G10R10UIntPack32,
            vk::Format::A2B10G10R10_SINT_PACK32 => ImageFormat::A2B10G10R10SIntPack32,
            vk::Format::R16_UNORM => ImageFormat::R16UNorm,
            vk::Format::R16_SNORM => ImageFormat::R16SNorm,
            vk::Format::R16_USCALED => ImageFormat::R16UScaled,
            vk::Format::R16_SSCALED => ImageFormat::R16SScaled,
            vk::Format::R16_UINT => ImageFormat::R16UInt,
            vk::Format::R16_SINT => ImageFormat::R16SInt,
            vk::Format::R16_SFLOAT => ImageFormat::R16SFloat,
            vk::Format::R16G16_UNORM => ImageFormat::R16G16UNorm,
            vk::Format::R16G16_SNORM => ImageFormat::R16G16SNorm,
            vk::Format::R16G16_USCALED => ImageFormat::R16G16UScaled,
            vk::Format::R16G16_SSCALED => ImageFormat::R16G16SScaled,
            vk::Format::R16G16_UINT => ImageFormat::R16G16UInt,
            vk::Format::R16G16_SINT => ImageFormat::R16G16SInt,
            vk::Format::R16G16_SFLOAT => ImageFormat::R16G16SFloat,
            vk::Format::R16G16B16_UNORM => ImageFormat::R16G16B16UNorm,
            vk::Format::R16G16B16_SNORM => ImageFormat::R16G16B16SNorm,
            vk::Format::R16G16B16_USCALED => ImageFormat::R16G16B16UScaled,
            vk::Format::R16G16B16_SSCALED => ImageFormat::R16G16B16SScaled,
            vk::Format::R16G16B16_UINT => ImageFormat::R16G16B16UInt,
            vk::Format::R16G16B16_SINT => ImageFormat::R16G16B16SInt,
            vk::Format::R16G16B16_SFLOAT => ImageFormat::R16G16B16SFloat,
            vk::Format::R16G16B16A16_UNORM => ImageFormat::R16G16B16A16UNorm,
            vk::Format::R16G16B16A16_SNORM => ImageFormat::R16G16B16A16SNorm,
            vk::Format::R16G16B16A16_USCALED => ImageFormat::R16G16B16A16UScaled,
            vk::Format::R16G16B16A16_SSCALED => ImageFormat::R16G16B16A16SScaled,
            vk::Format::R16G16B16A16_UINT => ImageFormat::R16G16B16A16UInt,
            vk::Format::R16G16B16A16_SINT => ImageFormat::R16G16B16A16SInt,
            vk::Format::R16G16B16A16_SFLOAT => ImageFormat::R16G16B16A16SFloat,
            vk::Format::R32_UINT => ImageFormat::R32UInt,
            vk::Format::R32_SINT => ImageFormat::R32SInt,
            vk::Format::R32_SFLOAT => ImageFormat::R32SFloat,
            vk::Format::R32G32_UINT => ImageFormat::R32G32UInt,
            vk::Format::R32G32_SINT => ImageFormat::R32G32SInt,
            vk::Format::R32G32_SFLOAT => ImageFormat::R32G32SFloat,
            vk::Format::R32G32B32_UINT => ImageFormat::R32G32B32UInt,
            vk::Format::R32G32B32_SINT => ImageFormat::R32G32B32SInt,
            vk::Format::R32G32B32_SFLOAT => ImageFormat::R32G32B32SFloat,
            vk::Format::R32G32B32A32_UINT => ImageFormat::R32G32B32A32UInt,
            vk::Format::R32G32B32A32_SINT => ImageFormat::R32G32B32A32SInt,
            vk::Format::R32G32B32A32_SFLOAT => ImageFormat::R32G32B32A32SFloat,
            vk::Format::R64_UINT => ImageFormat::R64UInt,
            vk::Format::R64_SINT => ImageFormat::R64SInt,
            vk::Format::R64_SFLOAT => ImageFormat::R64SFloat,
            vk::Format::R64G64_UINT => ImageFormat::R64G64UInt,
            vk::Format::R64G64_SINT => ImageFormat::R64G64SInt,
            vk::Format::R64G64_SFLOAT => ImageFormat::R64G64SFloat,
            vk::Format::R64G64B64_UINT => ImageFormat::R64G64B64UInt,
            vk::Format::R64G64B64_SINT => ImageFormat::R64G64B64SInt,
            vk::Format::R64G64B64_SFLOAT => ImageFormat::R64G64B64SFloat,
            vk::Format::R64G64B64A64_UINT => ImageFormat::R64G64B64A64UInt,
            vk::Format::R64G64B64A64_SINT => ImageFormat::R64G64B64A64SInt,
            vk::Format::R64G64B64A64_SFLOAT => ImageFormat::R64G64B64A64SFloat,
            vk::Format::B10G11R11_UFLOAT_PACK32 => ImageFormat::B10G11R11UFloatPack32,
            vk::Format::E5B9G9R9_UFLOAT_PACK32 => ImageFormat::E5B9G9R9UFloatPack32,
            vk::Format::D16_UNORM => ImageFormat::D16UNorm,
            vk::Format::X8_D24_UNORM_PACK32 => ImageFormat::X8D24UNormPack32,
            vk::Format::D32_SFLOAT => ImageFormat::D32SFloat,
            vk::Format::S8_UINT => ImageFormat::S8UInt,
            vk::Format::D16_UNORM_S8_UINT => ImageFormat::D16UNormS8UInt,
            vk::Format::D24_UNORM_S8_UINT => ImageFormat::D24UNormS8UInt,
            vk::Format::D32_SFLOAT_S8_UINT => ImageFormat::D32SFloatS8UInt,
            vk::Format::BC1_RGB_UNORM_BLOCK => ImageFormat::BC1RGBUNormBlock,
            vk::Format::BC1_RGB_SRGB_BLOCK => ImageFormat::BC1RGBSRgbBlock,
            vk::Format::BC1_RGBA_UNORM_BLOCK => ImageFormat::BC1RGBAUNormBlock,
            vk::Format::BC1_RGBA_SRGB_BLOCK => ImageFormat::BC1RGBASRgbBlock,
            vk::Format::BC2_UNORM_BLOCK => ImageFormat::BC2UNormBlock,
            vk::Format::BC2_SRGB_BLOCK => ImageFormat::BC2SRgbBlock,
            vk::Format::BC3_UNORM_BLOCK => ImageFormat::BC3UNormBlock,
            vk::Format::BC3_SRGB_BLOCK => ImageFormat::BC3SRgbBlock,
            vk::Format::BC4_UNORM_BLOCK => ImageFormat::BC4UNormBlock,
            vk::Format::BC4_SNORM_BLOCK => ImageFormat::BC4SNormBlock,
            vk::Format::BC5_UNORM_BLOCK => ImageFormat::BC5UNormBlock,
            vk::Format::BC5_SNORM_BLOCK => ImageFormat::BC5SNormBlock,
            vk::Format::BC6H_UFLOAT_BLOCK => ImageFormat::BC6HUFloatBlock,
            vk::Format::BC6H_SFLOAT_BLOCK => ImageFormat::BC6HSFloatBlock,
            vk::Format::BC7_UNORM_BLOCK => ImageFormat::BC7UNormBlock,
            vk::Format::BC7_SRGB_BLOCK => ImageFormat::BC7SRgbBlock,
            vk::Format::ETC2_R8G8B8_UNORM_BLOCK => ImageFormat::ETC2R8G8B8UNormBlock,
            vk::Format::ETC2_R8G8B8_SRGB_BLOCK => ImageFormat::ETC2R8G8B8SRgbBlock,
            vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK => ImageFormat::ETC2R8G8B8A1UNormBlock,
            vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK => ImageFormat::ETC2R8G8B8A1SRgbBlock,
            vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK => ImageFormat::ETC2R8G8B8A8UNormBlock,
            vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK => ImageFormat::ETC2R8G8B8A8SRgbBlock,
            vk::Format::EAC_R11_UNORM_BLOCK => ImageFormat::EACR11UNormBlock,
            vk::Format::EAC_R11_SNORM_BLOCK => ImageFormat::EACR11SNormBlock,
            vk::Format::EAC_R11G11_UNORM_BLOCK => ImageFormat::EACR11G11UNormBlock,
            vk::Format::EAC_R11G11_SNORM_BLOCK => ImageFormat::EACR11G11SNormBlock,
            vk::Format::ASTC_4X4_UNORM_BLOCK => ImageFormat::ASTC4X4UNormBlock,
            vk::Format::ASTC_4X4_SRGB_BLOCK => ImageFormat::ASTC4X4SRgbBlock,
            vk::Format::ASTC_5X4_UNORM_BLOCK => ImageFormat::ASTC5X4UNormBlock,
            vk::Format::ASTC_5X4_SRGB_BLOCK => ImageFormat::ASTC5X4SRgbBlock,
            vk::Format::ASTC_5X5_UNORM_BLOCK => ImageFormat::ASTC5X5UNormBlock,
            vk::Format::ASTC_5X5_SRGB_BLOCK => ImageFormat::ASTC5X5SRgbBlock,
            vk::Format::ASTC_6X5_UNORM_BLOCK => ImageFormat::ASTC6X5UNormBlock,
            vk::Format::ASTC_6X5_SRGB_BLOCK => ImageFormat::ASTC6X5SRgbBlock,
            vk::Format::ASTC_6X6_UNORM_BLOCK => ImageFormat::ASTC6X6UNormBlock,
            vk::Format::ASTC_6X6_SRGB_BLOCK => ImageFormat::ASTC6X6SRgbBlock,
            vk::Format::ASTC_8X5_UNORM_BLOCK => ImageFormat::ASTC8X5UNormBlock,
            vk::Format::ASTC_8X5_SRGB_BLOCK => ImageFormat::ASTC8X5SRgbBlock,
            vk::Format::ASTC_8X6_UNORM_BLOCK => ImageFormat::ASTC8X6UNormBlock,
            vk::Format::ASTC_8X6_SRGB_BLOCK => ImageFormat::ASTC8X6SRgbBlock,
            vk::Format::ASTC_8X8_UNORM_BLOCK => ImageFormat::ASTC8X8UNormBlock,
            vk::Format::ASTC_8X8_SRGB_BLOCK => ImageFormat::ASTC8X8SRgbBlock,
            vk::Format::ASTC_10X5_UNORM_BLOCK => ImageFormat::ASTC10X5UNormBlock,
            vk::Format::ASTC_10X5_SRGB_BLOCK => ImageFormat::ASTC10X5SRgbBlock,
            vk::Format::ASTC_10X6_UNORM_BLOCK => ImageFormat::ASTC10X6UNormBlock,
            vk::Format::ASTC_10X6_SRGB_BLOCK => ImageFormat::ASTC10X6SRgbBlock,
            vk::Format::ASTC_10X8_UNORM_BLOCK => ImageFormat::ASTC10X8UNormBlock,
            vk::Format::ASTC_10X8_SRGB_BLOCK => ImageFormat::ASTC10X8SRgbBlock,
            vk::Format::ASTC_10X10_UNORM_BLOCK => ImageFormat::ASTC10X10UNormBlock,
            vk::Format::ASTC_10X10_SRGB_BLOCK => ImageFormat::ASTC10X10SRgbBlock,
            vk::Format::ASTC_12X10_UNORM_BLOCK => ImageFormat::ASTC12X10UNormBlock,
            vk::Format::ASTC_12X10_SRGB_BLOCK => ImageFormat::ASTC12X10SRgbBlock,
            vk::Format::ASTC_12X12_UNORM_BLOCK => ImageFormat::ASTC12X12UNormBlock,
            vk::Format::ASTC_12X12_SRGB_BLOCK => ImageFormat::ASTC12X12SRgbBlock,
            
            _ => { panic!("Encountered illegal VkFormat value '{}'", value.as_raw()) }
        }
    }
}

impl From<ImageFormat> for vk::Format {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Undefined => vk::Format::UNDEFINED,

            ImageFormat::R4G4UNormPack8 => vk::Format::R4G4_UNORM_PACK8,
            ImageFormat::R4G4B4A4UNormPack16 => vk::Format::R4G4B4A4_UNORM_PACK16,
            ImageFormat::B4G4R4A4UNormPack16 => vk::Format::B4G4R4A4_UNORM_PACK16,
            ImageFormat::R5G6B5UNormPack16 => vk::Format::R5G6B5_UNORM_PACK16,
            ImageFormat::B5G6R5UNormPack16 => vk::Format::B5G6R5_UNORM_PACK16,
            ImageFormat::R5G5B5A1UNormPack16 => vk::Format::R5G5B5A1_UNORM_PACK16,
            ImageFormat::B5G5R5A1UNormPack16 => vk::Format::B5G5R5A1_UNORM_PACK16,
            ImageFormat::A1R5G5B5UNormPack16 => vk::Format::A1R5G5B5_UNORM_PACK16,
            ImageFormat::R8UNorm => vk::Format::R8_UNORM,
            ImageFormat::R8SNorm => vk::Format::R8_SNORM,
            ImageFormat::R8UScaled => vk::Format::R8_USCALED,
            ImageFormat::R8SScaled => vk::Format::R8_SSCALED,
            ImageFormat::R8UInt => vk::Format::R8_UINT,
            ImageFormat::R8SInt => vk::Format::R8_SINT,
            ImageFormat::R8SRgb => vk::Format::R8_SRGB,
            ImageFormat::R8G8UNorm => vk::Format::R8G8_UNORM,
            ImageFormat::R8G8SNorm => vk::Format::R8G8_SNORM,
            ImageFormat::R8G8UScaled => vk::Format::R8G8_USCALED,
            ImageFormat::R8G8SScaled => vk::Format::R8G8_SSCALED,
            ImageFormat::R8G8UInt => vk::Format::R8G8_UINT,
            ImageFormat::R8G8SInt => vk::Format::R8G8_SINT,
            ImageFormat::R8G8SRgb => vk::Format::R8G8_SRGB,
            ImageFormat::R8G8B8UNorm => vk::Format::R8G8B8_UNORM,
            ImageFormat::R8G8B8SNorm => vk::Format::R8G8B8_SNORM,
            ImageFormat::R8G8B8UScaled => vk::Format::R8G8B8_USCALED,
            ImageFormat::R8G8B8SScaled => vk::Format::R8G8B8_SSCALED,
            ImageFormat::R8G8B8UInt => vk::Format::R8G8B8_UINT,
            ImageFormat::R8G8B8SInt => vk::Format::R8G8B8_SINT,
            ImageFormat::R8G8B8SRgb => vk::Format::R8G8B8_SRGB,
            ImageFormat::B8G8R8UNorm => vk::Format::B8G8R8_UNORM,
            ImageFormat::B8G8R8SNorm => vk::Format::B8G8R8_SNORM,
            ImageFormat::B8G8R8UScaled => vk::Format::B8G8R8_USCALED,
            ImageFormat::B8G8R8SScaled => vk::Format::B8G8R8_SSCALED,
            ImageFormat::B8G8R8UInt => vk::Format::B8G8R8_UINT,
            ImageFormat::B8G8R8SInt => vk::Format::B8G8R8_SINT,
            ImageFormat::B8G8R8SRgb => vk::Format::B8G8R8_SRGB,
            ImageFormat::R8G8B8A8UNorm => vk::Format::R8G8B8A8_UNORM,
            ImageFormat::R8G8B8A8SNorm => vk::Format::R8G8B8A8_SNORM,
            ImageFormat::R8G8B8A8UScaled => vk::Format::R8G8B8A8_USCALED,
            ImageFormat::R8G8B8A8SScaled => vk::Format::R8G8B8A8_SSCALED,
            ImageFormat::R8G8B8A8UInt => vk::Format::R8G8B8A8_UINT,
            ImageFormat::R8G8B8A8SInt => vk::Format::R8G8B8A8_SINT,
            ImageFormat::R8G8B8A8SRgb => vk::Format::R8G8B8A8_SRGB,
            ImageFormat::B8G8R8A8UNorm => vk::Format::B8G8R8A8_UNORM,
            ImageFormat::B8G8R8A8SNorm => vk::Format::B8G8R8A8_SNORM,
            ImageFormat::B8G8R8A8UScaled => vk::Format::B8G8R8A8_USCALED,
            ImageFormat::B8G8R8A8SScaled => vk::Format::B8G8R8A8_SSCALED,
            ImageFormat::B8G8R8A8UInt => vk::Format::B8G8R8A8_UINT,
            ImageFormat::B8G8R8A8SInt => vk::Format::B8G8R8A8_SINT,
            ImageFormat::B8G8R8A8SRgb => vk::Format::B8G8R8A8_SRGB,
            ImageFormat::A8B8G8R8UNormPack32 => vk::Format::A8B8G8R8_UNORM_PACK32,
            ImageFormat::A8B8G8R8SNormPack32 => vk::Format::A8B8G8R8_SNORM_PACK32,
            ImageFormat::A8B8G8R8UScaledPack32 => vk::Format::A8B8G8R8_USCALED_PACK32,
            ImageFormat::A8B8G8R8SScaledPack32 => vk::Format::A8B8G8R8_SSCALED_PACK32,
            ImageFormat::A8B8G8R8UIntPack32 => vk::Format::A8B8G8R8_UINT_PACK32,
            ImageFormat::A8B8G8R8SIntPack32 => vk::Format::A8B8G8R8_SINT_PACK32,
            ImageFormat::A8B8G8R8SRgbPack32 => vk::Format::A8B8G8R8_SRGB_PACK32,
            ImageFormat::A2R10G10B10UNormPack32 => vk::Format::A2R10G10B10_UNORM_PACK32,
            ImageFormat::A2R10G10B10SNormPack32 => vk::Format::A2R10G10B10_SNORM_PACK32,
            ImageFormat::A2R10G10B10UScaledPack32 => vk::Format::A2R10G10B10_USCALED_PACK32,
            ImageFormat::A2R10G10B10SScaledPack32 => vk::Format::A2R10G10B10_SSCALED_PACK32,
            ImageFormat::A2R10G10B10UIntPack32 => vk::Format::A2R10G10B10_UINT_PACK32,
            ImageFormat::A2R10G10B10SIntPack32 => vk::Format::A2R10G10B10_SINT_PACK32,
            ImageFormat::A2B10G10R10UNormPack32 => vk::Format::A2B10G10R10_UNORM_PACK32,
            ImageFormat::A2B10G10R10SNormPack32 => vk::Format::A2B10G10R10_SNORM_PACK32,
            ImageFormat::A2B10G10R10UScaledPack32 => vk::Format::A2B10G10R10_USCALED_PACK32,
            ImageFormat::A2B10G10R10SScaledPack32 => vk::Format::A2B10G10R10_SSCALED_PACK32,
            ImageFormat::A2B10G10R10UIntPack32 => vk::Format::A2B10G10R10_UINT_PACK32,
            ImageFormat::A2B10G10R10SIntPack32 => vk::Format::A2B10G10R10_SINT_PACK32,
            ImageFormat::R16UNorm => vk::Format::R16_UNORM,
            ImageFormat::R16SNorm => vk::Format::R16_SNORM,
            ImageFormat::R16UScaled => vk::Format::R16_USCALED,
            ImageFormat::R16SScaled => vk::Format::R16_SSCALED,
            ImageFormat::R16UInt => vk::Format::R16_UINT,
            ImageFormat::R16SInt => vk::Format::R16_SINT,
            ImageFormat::R16SFloat => vk::Format::R16_SFLOAT,
            ImageFormat::R16G16UNorm => vk::Format::R16G16_UNORM,
            ImageFormat::R16G16SNorm => vk::Format::R16G16_SNORM,
            ImageFormat::R16G16UScaled => vk::Format::R16G16_USCALED,
            ImageFormat::R16G16SScaled => vk::Format::R16G16_SSCALED,
            ImageFormat::R16G16UInt => vk::Format::R16G16_UINT,
            ImageFormat::R16G16SInt => vk::Format::R16G16_SINT,
            ImageFormat::R16G16SFloat => vk::Format::R16G16_SFLOAT,
            ImageFormat::R16G16B16UNorm => vk::Format::R16G16B16_UNORM,
            ImageFormat::R16G16B16SNorm => vk::Format::R16G16B16_SNORM,
            ImageFormat::R16G16B16UScaled => vk::Format::R16G16B16_USCALED,
            ImageFormat::R16G16B16SScaled => vk::Format::R16G16B16_SSCALED,
            ImageFormat::R16G16B16UInt => vk::Format::R16G16B16_UINT,
            ImageFormat::R16G16B16SInt => vk::Format::R16G16B16_SINT,
            ImageFormat::R16G16B16SFloat => vk::Format::R16G16B16_SFLOAT,
            ImageFormat::R16G16B16A16UNorm => vk::Format::R16G16B16A16_UNORM,
            ImageFormat::R16G16B16A16SNorm => vk::Format::R16G16B16A16_SNORM,
            ImageFormat::R16G16B16A16UScaled => vk::Format::R16G16B16A16_USCALED,
            ImageFormat::R16G16B16A16SScaled => vk::Format::R16G16B16A16_SSCALED,
            ImageFormat::R16G16B16A16UInt => vk::Format::R16G16B16A16_UINT,
            ImageFormat::R16G16B16A16SInt => vk::Format::R16G16B16A16_SINT,
            ImageFormat::R16G16B16A16SFloat => vk::Format::R16G16B16A16_SFLOAT,
            ImageFormat::R32UInt => vk::Format::R32_UINT,
            ImageFormat::R32SInt => vk::Format::R32_SINT,
            ImageFormat::R32SFloat => vk::Format::R32_SFLOAT,
            ImageFormat::R32G32UInt => vk::Format::R32G32_UINT,
            ImageFormat::R32G32SInt => vk::Format::R32G32_SINT,
            ImageFormat::R32G32SFloat => vk::Format::R32G32_SFLOAT,
            ImageFormat::R32G32B32UInt => vk::Format::R32G32B32_UINT,
            ImageFormat::R32G32B32SInt => vk::Format::R32G32B32_SINT,
            ImageFormat::R32G32B32SFloat => vk::Format::R32G32B32_SFLOAT,
            ImageFormat::R32G32B32A32UInt => vk::Format::R32G32B32A32_UINT,
            ImageFormat::R32G32B32A32SInt => vk::Format::R32G32B32A32_SINT,
            ImageFormat::R32G32B32A32SFloat => vk::Format::R32G32B32A32_SFLOAT,
            ImageFormat::R64UInt => vk::Format::R64_UINT,
            ImageFormat::R64SInt => vk::Format::R64_SINT,
            ImageFormat::R64SFloat => vk::Format::R64_SFLOAT,
            ImageFormat::R64G64UInt => vk::Format::R64G64_UINT,
            ImageFormat::R64G64SInt => vk::Format::R64G64_SINT,
            ImageFormat::R64G64SFloat => vk::Format::R64G64_SFLOAT,
            ImageFormat::R64G64B64UInt => vk::Format::R64G64B64_UINT,
            ImageFormat::R64G64B64SInt => vk::Format::R64G64B64_SINT,
            ImageFormat::R64G64B64SFloat => vk::Format::R64G64B64_SFLOAT,
            ImageFormat::R64G64B64A64UInt => vk::Format::R64G64B64A64_UINT,
            ImageFormat::R64G64B64A64SInt => vk::Format::R64G64B64A64_SINT,
            ImageFormat::R64G64B64A64SFloat => vk::Format::R64G64B64A64_SFLOAT,
            ImageFormat::B10G11R11UFloatPack32 => vk::Format::B10G11R11_UFLOAT_PACK32,
            ImageFormat::E5B9G9R9UFloatPack32 => vk::Format::E5B9G9R9_UFLOAT_PACK32,
            ImageFormat::D16UNorm => vk::Format::D16_UNORM,
            ImageFormat::X8D24UNormPack32 => vk::Format::X8_D24_UNORM_PACK32,
            ImageFormat::D32SFloat => vk::Format::D32_SFLOAT,
            ImageFormat::S8UInt => vk::Format::S8_UINT,
            ImageFormat::D16UNormS8UInt => vk::Format::D16_UNORM_S8_UINT,
            ImageFormat::D24UNormS8UInt => vk::Format::D24_UNORM_S8_UINT,
            ImageFormat::D32SFloatS8UInt => vk::Format::D32_SFLOAT_S8_UINT,
            ImageFormat::BC1RGBUNormBlock => vk::Format::BC1_RGB_UNORM_BLOCK,
            ImageFormat::BC1RGBSRgbBlock => vk::Format::BC1_RGB_SRGB_BLOCK,
            ImageFormat::BC1RGBAUNormBlock => vk::Format::BC1_RGBA_UNORM_BLOCK,
            ImageFormat::BC1RGBASRgbBlock => vk::Format::BC1_RGBA_SRGB_BLOCK,
            ImageFormat::BC2UNormBlock => vk::Format::BC2_UNORM_BLOCK,
            ImageFormat::BC2SRgbBlock => vk::Format::BC2_SRGB_BLOCK,
            ImageFormat::BC3UNormBlock => vk::Format::BC3_UNORM_BLOCK,
            ImageFormat::BC3SRgbBlock => vk::Format::BC3_SRGB_BLOCK,
            ImageFormat::BC4UNormBlock => vk::Format::BC4_UNORM_BLOCK,
            ImageFormat::BC4SNormBlock => vk::Format::BC4_SNORM_BLOCK,
            ImageFormat::BC5UNormBlock => vk::Format::BC5_UNORM_BLOCK,
            ImageFormat::BC5SNormBlock => vk::Format::BC5_SNORM_BLOCK,
            ImageFormat::BC6HUFloatBlock => vk::Format::BC6H_UFLOAT_BLOCK,
            ImageFormat::BC6HSFloatBlock => vk::Format::BC6H_SFLOAT_BLOCK,
            ImageFormat::BC7UNormBlock => vk::Format::BC7_UNORM_BLOCK,
            ImageFormat::BC7SRgbBlock => vk::Format::BC7_SRGB_BLOCK,
            ImageFormat::ETC2R8G8B8UNormBlock => vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
            ImageFormat::ETC2R8G8B8SRgbBlock => vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
            ImageFormat::ETC2R8G8B8A1UNormBlock => vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK,
            ImageFormat::ETC2R8G8B8A1SRgbBlock => vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
            ImageFormat::ETC2R8G8B8A8UNormBlock => vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK,
            ImageFormat::ETC2R8G8B8A8SRgbBlock => vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
            ImageFormat::EACR11UNormBlock => vk::Format::EAC_R11_UNORM_BLOCK,
            ImageFormat::EACR11SNormBlock => vk::Format::EAC_R11_SNORM_BLOCK,
            ImageFormat::EACR11G11UNormBlock => vk::Format::EAC_R11G11_UNORM_BLOCK,
            ImageFormat::EACR11G11SNormBlock => vk::Format::EAC_R11G11_SNORM_BLOCK,
            ImageFormat::ASTC4X4UNormBlock => vk::Format::ASTC_4X4_UNORM_BLOCK,
            ImageFormat::ASTC4X4SRgbBlock => vk::Format::ASTC_4X4_SRGB_BLOCK,
            ImageFormat::ASTC5X4UNormBlock => vk::Format::ASTC_5X4_UNORM_BLOCK,
            ImageFormat::ASTC5X4SRgbBlock => vk::Format::ASTC_5X4_SRGB_BLOCK,
            ImageFormat::ASTC5X5UNormBlock => vk::Format::ASTC_5X5_UNORM_BLOCK,
            ImageFormat::ASTC5X5SRgbBlock => vk::Format::ASTC_5X5_SRGB_BLOCK,
            ImageFormat::ASTC6X5UNormBlock => vk::Format::ASTC_6X5_UNORM_BLOCK,
            ImageFormat::ASTC6X5SRgbBlock => vk::Format::ASTC_6X5_SRGB_BLOCK,
            ImageFormat::ASTC6X6UNormBlock => vk::Format::ASTC_6X6_UNORM_BLOCK,
            ImageFormat::ASTC6X6SRgbBlock => vk::Format::ASTC_6X6_SRGB_BLOCK,
            ImageFormat::ASTC8X5UNormBlock => vk::Format::ASTC_8X5_UNORM_BLOCK,
            ImageFormat::ASTC8X5SRgbBlock => vk::Format::ASTC_8X5_SRGB_BLOCK,
            ImageFormat::ASTC8X6UNormBlock => vk::Format::ASTC_8X6_UNORM_BLOCK,
            ImageFormat::ASTC8X6SRgbBlock => vk::Format::ASTC_8X6_SRGB_BLOCK,
            ImageFormat::ASTC8X8UNormBlock => vk::Format::ASTC_8X8_UNORM_BLOCK,
            ImageFormat::ASTC8X8SRgbBlock => vk::Format::ASTC_8X8_SRGB_BLOCK,
            ImageFormat::ASTC10X5UNormBlock => vk::Format::ASTC_10X5_UNORM_BLOCK,
            ImageFormat::ASTC10X5SRgbBlock => vk::Format::ASTC_10X5_SRGB_BLOCK,
            ImageFormat::ASTC10X6UNormBlock => vk::Format::ASTC_10X6_UNORM_BLOCK,
            ImageFormat::ASTC10X6SRgbBlock => vk::Format::ASTC_10X6_SRGB_BLOCK,
            ImageFormat::ASTC10X8UNormBlock => vk::Format::ASTC_10X8_UNORM_BLOCK,
            ImageFormat::ASTC10X8SRgbBlock => vk::Format::ASTC_10X8_SRGB_BLOCK,
            ImageFormat::ASTC10X10UNormBlock => vk::Format::ASTC_10X10_UNORM_BLOCK,
            ImageFormat::ASTC10X10SRgbBlock => vk::Format::ASTC_10X10_SRGB_BLOCK,
            ImageFormat::ASTC12X10UNormBlock => vk::Format::ASTC_12X10_UNORM_BLOCK,
            ImageFormat::ASTC12X10SRgbBlock => vk::Format::ASTC_12X10_SRGB_BLOCK,
            ImageFormat::ASTC12X12UNormBlock => vk::Format::ASTC_12X12_UNORM_BLOCK,
            ImageFormat::ASTC12X12SRgbBlock => vk::Format::ASTC_12X12_SRGB_BLOCK,
        }
    }
}



/// The layout of an Image.
#[derive(Clone, Copy, Debug)]
pub enum ImageLayout {
    /// We don't care about the layout / it's not yet defined.
    Undefined,
    /// The image has a default layout and _may_ contain data, but its layout is not yet initialized.
    /// 
    /// This can only be used for the initialLayout in the VkImageCreateInfo struct.
    Preinitialized,
    /// A general layout that is applicable to many things (i.e., all types of device access, though probably not optimized).
    General,

    /// Optimal layout for colour attachments.
    ColourAttachment,
    /// Optimal layout for a depth stencil.
    DepthStencil,
    /// Optimal layout for a read-only depth stencil.
    DepthStencilReadOnly,
    /// Optimal layout for an image that is read during a shader stage.
    ShaderReadOnly,
    /// Optimal layout for presenting to a swapchain.
    Present,

    /// Optimal layout for the image data being transferred to another image.
    TransferSrc,
    /// Optimal layout for the image's data being overwritten with transferred data from another image.
    TransferDst,
}

impl From<vk::ImageLayout> for ImageLayout {
    #[inline]
    fn from(value: vk::ImageLayout) -> Self {
        match value {
            vk::ImageLayout::UNDEFINED      => ImageLayout::Undefined,
            vk::ImageLayout::PREINITIALIZED => ImageLayout::Preinitialized,
            vk::ImageLayout::GENERAL        => ImageLayout::General,

            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL         => ImageLayout::ColourAttachment,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => ImageLayout::DepthStencil,
            vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL  => ImageLayout::DepthStencilReadOnly,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL         => ImageLayout::ShaderReadOnly,
            vk::ImageLayout::PRESENT_SRC_KHR                  => ImageLayout::Present,

            vk::ImageLayout::TRANSFER_SRC_OPTIMAL => ImageLayout::TransferSrc,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL => ImageLayout::TransferDst,

            value => { panic!("Encountered illegal VkImageLayout value '{}'", value.as_raw()); }
        }
    }
}

impl From<ImageLayout> for vk::ImageLayout {
    #[inline]
    fn from(value: ImageLayout) -> Self {
        match value {
            ImageLayout::Undefined      => vk::ImageLayout::UNDEFINED,
            ImageLayout::Preinitialized => vk::ImageLayout::PREINITIALIZED,
            ImageLayout::General        => vk::ImageLayout::GENERAL,

            ImageLayout::ColourAttachment     => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencil         => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilReadOnly => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            ImageLayout::ShaderReadOnly       => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            ImageLayout::Present              => vk::ImageLayout::PRESENT_SRC_KHR,

            ImageLayout::TransferSrc => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            ImageLayout::TransferDst => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
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
