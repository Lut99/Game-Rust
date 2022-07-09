/* FLAGS.rs
 *   by Lut99
 *
 * Created:
 *   09 Jul 2022, 10:44:36
 * Last edited:
 *   09 Jul 2022, 12:38:08
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains auxillary Flag-structs used as representatives of Vulkan
 *   flags.
**/

use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::ops::{BitAnd, BitOr, BitOrAssign, Not};

use ash::vk;
use num_traits::{NumCast, Unsigned};


/***** HELPER MACROS *****/
/// Wrapper macro to shortcut the Display trait for flags
#[macro_export]
macro_rules! flags_display {
    ($flag:ident, $($match:path => $code:literal $(,)?),+) => {
        impl Display for $flag {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Construct a list
                let mut first = true;
                let mut i     = 0x1;
                while i != 0 {
                    // Check if this property is enabled
                    if self.0 & i != 0 {
                        // Write the comma if necessary
                        if first { first = false; }
                        else { write!(f, ", ")?; }

                        // Write the name of this property
                        match $flag(self.0 & i) {
                            $($match => { write!(f, $code)?; }),+
                            val => { panic!(concat!("Encountered illegal ", stringify!($flag), " value '{}'"), val.0); }
                        }
                    }

                    // Increment the i
                    i = i << 1;
                }

                // Done
                Ok(())
            }
        }
    }
}

/// Wrapper macro to shortcut the From trait for flags
#[macro_export]
macro_rules! flags_from {
    (vk::$from:ident, $to:ident, $($match:path => $target:path $(,)?),+) => {
        impl From<vk::$from> for $to {
            fn from(value: vk::$from) -> $to {
                // Construct the resulting flag iteratively
                let mut result: $to = $to::empty();
                $(if (value & $match).as_raw() != 0 { result |= $target });+
                result
            }
        }

        impl From<$to> for vk::$from {
            fn from(value: $to) -> vk::$from {
                // Construct the resulting flag iteratively
                let mut result: vk::$from = vk::$from::empty();
                $(if value.check($target) { result |= $match });+
                result
            }
        }
    };
}

/// Macros that ORs together the given flags at constant-time
#[macro_export]
macro_rules! join_flags {
    ($flag:ident, $value:expr) => {
        $flag::from_raw($value)
    };

    ($flag:ident, $lhs:expr, $rhs:expr, $($values:expr),*) => {
        join_flags!($flag, ($lhs).as_raw() | ($rhs).as_raw(), $($values),*)
    };
}





/***** HELPER TRAIT *****/
/// Provides a uniform interface to all flags.
pub trait Flags: Clone + Copy + Debug + Eq + PartialEq {
    /// Determines the type of the internal value where the flags are stored.
    type RawType: BitAnd<Output = Self::RawType> + BitOr<Output = Self::RawType> + Not<Output = Self::RawType> + NumCast + PartialEq + Unsigned;


    /// Constructor for the Flags object that creates it without any flags initialized.
    /// 
    /// # Returns
    /// A new instance of Self with no flags set.
    #[inline]
    fn empty() -> Self { Self::from_raw(num_traits::cast::cast::<u8, Self::RawType>(0).unwrap()) }

    /// Constructor for the Flags object that creates it with all flags initialized.
    /// 
    /// # Returns
    /// A new instance of Self with all flags set.
    #[inline]
    fn all() -> Self { Self::from_raw(!num_traits::cast::cast::<u8, Self::RawType>(0).unwrap()) }

    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    fn from_raw(value: Self::RawType) -> Self;

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    fn as_raw(&self) -> Self::RawType;



    /// Returns true iff no flags are set.
    #[inline]
    fn is_empty(&self) -> bool { *self == Self::empty() }

    /// Checks if the given argument is a subset of this set of flags.
    /// 
    /// # Arguments
    /// - `other`: The other `Flags` that might be a subset of this Flags.
    /// 
    /// # Returns
    /// `true` if the given set is a subset of this one, or `false` otherwise.
    #[inline]
    fn check(&self, other: Self) -> bool { (self.as_raw() & other.as_raw()) == other.as_raw() }
}

impl<T: Flags> BitOr for T {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self::Output {
        Self::from_raw(self.as_raw() | other.as_raw())
    }
}

impl<T: Flags> BitOrAssign for T {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = self.bitor(other)
    }
}





/***** DEVICES *****/
/// Contains information about what a device heap supports, exactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeapPropertyFlags(u8);

impl HeapPropertyFlags {
    /// The heap corresponds to device-local memory.
    pub const DEVICE_LOCAL: Self = Self(0x01);
    /// In the case of a multi-instance logical device, this heap has a per-device instance. That means that (by default) every allocation will be replicated to each heap.
    pub const MULTI_INSTANCE: Self = Self(0x02);
}

impl Flags for HeapPropertyFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u8;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(HeapPropertyFlags,
    HeapPropertyFlags::DEVICE_LOCAL   => "DEVICE_LOCAL",
    HeapPropertyFlags::MULTI_INSTANCE => "MULTI_INSTANCE",
);

flags_from!(vk::MemoryHeapFlags, HeapPropertyFlags, 
    vk::MemoryHeapFlags::DEVICE_LOCAL       => HeapPropertyFlags::DEVICE_LOCAL,
    vk::MemoryHeapFlags::MULTI_INSTANCE     => HeapPropertyFlags::MULTI_INSTANCE,
    vk::MemoryHeapFlags::MULTI_INSTANCE_KHR => HeapPropertyFlags::MULTI_INSTANCE,
);





/***** SHADERS *****/
/// The ShaderStage is where a shader or a resource lives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShaderStage(u16);

impl ShaderStage {
    /// A ShaderStage that hits all stages
    pub const ALL: Self          = Self(0xFFFF);
    /// A ShaderStage that hits all graphics stages
    pub const ALL_GRAPHICS: Self = Self(0x001F);
    /// An empty ShaderStage
    pub const EMPTY: Self        = Self(0x0000);

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


    /// Creates a ShaderStage from a raw value.
    /// 
    /// # Arguments
    /// - `value`: The raw flags value. Note that this is the ShaderStage raw, _not_ the Vulkan raw.
    /// 
    /// # Returns
    /// A new ShaderStage with the raw flags set.
    pub const fn raw(value: u16) -> Self { Self(value) }

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

impl Display for ShaderStage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Construct a list of shader stages
        let mut stages = Vec::with_capacity(1);
        for value in &[ShaderStage::VERTEX, ShaderStage::TESSELLATION_CONTROL, ShaderStage::TESSELLATION_EVALUATION, ShaderStage::GEOMETRY, ShaderStage::FRAGMENT, ShaderStage::COMPUTE] {
            if self.check(*value) { stages.push(value); }
        }

        // Use that to construct a string list
        for i in 0..stages.len() {
            // Write the grammar
            if i > 0 && i < stages.len() - 1 { write!(f, ", ")?; }
            else if i > 0 { write!(f, " and ")?; }

            // Write the stage
            let stage = stages[i];
            if stage == &ShaderStage::VERTEX { write!(f, "Vertex")?; }
            else if stage == &ShaderStage::TESSELLATION_CONTROL { write!(f, "Tesselation (control)")?; }
            else if stage == &ShaderStage::TESSELLATION_EVALUATION { write!(f, "Tesselation (evaluation)")?; }
            else if stage == &ShaderStage::GEOMETRY { write!(f, "Geometry")?; }
            else if stage == &ShaderStage::FRAGMENT { write!(f, "Fragment")?; }
            else if stage == &ShaderStage::COMPUTE { write!(f, "Compute")?; }
        }

        // Done
        Ok(())
    }
}

impl From<vk::ShaderStageFlags> for ShaderStage {
    #[inline]
    fn from(value: vk::ShaderStageFlags) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&vk::ShaderStageFlags> for ShaderStage {
    #[inline]
    fn from(value: &vk::ShaderStageFlags) -> Self {
        // Construct it manually for portability
        let mut result = ShaderStage::EMPTY;
        if (*value & vk::ShaderStageFlags::VERTEX).as_raw() != 0 { result |= ShaderStage::VERTEX; }
        if (*value & vk::ShaderStageFlags::TESSELLATION_CONTROL).as_raw() != 0 { result |= ShaderStage::TESSELLATION_CONTROL; }
        if (*value & vk::ShaderStageFlags::TESSELLATION_EVALUATION).as_raw() != 0 { result |= ShaderStage::TESSELLATION_EVALUATION; }
        if (*value & vk::ShaderStageFlags::GEOMETRY).as_raw() != 0 { result |= ShaderStage::GEOMETRY; }
        if (*value & vk::ShaderStageFlags::FRAGMENT).as_raw() != 0 { result |= ShaderStage::FRAGMENT; }
        if (*value & vk::ShaderStageFlags::COMPUTE).as_raw() != 0 { result |= ShaderStage::COMPUTE; }

        // Return it
        result
    }
}

impl From<ShaderStage> for vk::ShaderStageFlags {
    fn from(value: ShaderStage) -> Self {
        // Use the reference version
        Self::from(&value)
    }
}

impl From<&ShaderStage> for vk::ShaderStageFlags {
    fn from(value: &ShaderStage) -> Self {
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





/***** RENDER PASSES *****/
/// Defines kinds of operations that are relevant for synchronization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessFlags(u32);

impl AccessFlags {
    /// Defines an operation that reads during the DRAW_INDIRECT pipeline stage(?)
    pub const INDIRECT_COMMAND_READ: Self = Self(0x00001);
    /// Defines a read operation in the index buffer.
    pub const INDEX_READ: Self = Self(0x00002);
    /// Defines a read operation of a vertex attribute in the vertex buffer.
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0x00004);
    /// Defines a read operation of a uniform buffer.
    pub const UNIFORM_READ: Self = Self(0x00008);
    /// Defines a read operation of an input attachment.
    pub const INPUT_ATTACHMENT_READ: Self = Self(0x00010);
    /// Defines a read operation in a shader.
    pub const SHADER_READ: Self = Self(0x00020);
    /// Defines a write operation in a shader.
    pub const SHADER_WRITE: Self = Self(0x00040);
    /// Defines a read operation from a colour attachment.
    pub const COLOUR_ATTACHMENT_READ: Self = Self(0x00080);
    /// Defines a write operation from a colour attachment.
    pub const COLOUR_ATTACHMENT_WRITE: Self = Self(0x00100);
    /// Defines a read operation from a depth stencil.
    pub const DEPTH_STENCIL_READ: Self = Self(0x00200);
    /// Defines a write operation from a depth stencil.
    pub const DEPTH_STENCIL_WRITE: Self = Self(0x00400);
    /// Defines a read operation during the transferring of buffers or images.
    pub const TRANSFER_READ: Self = Self(0x00800);
    /// Defines a write operation during the transferring of buffers or images.
    pub const TRANSFER_WRITE: Self = Self(0x01000);
    /// Defines a read operation performed by the host (I assume on GPU resources in shared memory).
    pub const HOST_READ: Self = Self(0x02000);
    /// Defines a write operation performed by the host (I assume on GPU resources in shared memory).
    pub const HOST_WRITE: Self = Self(0x04000);
    /// Defines _any_ read operation.
    pub const MEMORY_READ: Self  = Self(0x08000);
    /// Defines _any_ write operation.
    pub const MEMORY_WRITE: Self = Self(0x10000);
}

impl Flags for AccessFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u32;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(AccessFlags,
    AccessFlags::INDIRECT_COMMAND_READ   => "INDIRECT_COMMAND_READ",
    AccessFlags::INDEX_READ              => "INDEX_READ",
    AccessFlags::VERTEX_ATTRIBUTE_READ   => "VERTEX_ATTRIBUTE_READ",
    AccessFlags::UNIFORM_READ            => "UNIFORM_READ",
    AccessFlags::INPUT_ATTACHMENT_READ   => "INPUT_ATTACHMENT_READ",
    AccessFlags::SHADER_READ             => "SHADER_READ",
    AccessFlags::SHADER_WRITE            => "SHADER_WRITE",
    AccessFlags::COLOUR_ATTACHMENT_READ  => "COLOUR_ATTACHMENT_READ",
    AccessFlags::COLOUR_ATTACHMENT_WRITE => "COLOUR_ATTACHMENT_WRITE",
    AccessFlags::TRANSFER_READ           => "TRANSFER_READ",
    AccessFlags::TRANSFER_WRITE          => "TRANSFER_WRITE",
    AccessFlags::HOST_READ               => "HOST_READ",
    AccessFlags::HOST_WRITE              => "HOST_WRITE",
    AccessFlags::MEMORY_READ             => "MEMORY_READ",
    AccessFlags::MEMORY_WRITE            => "MEMORY_WRITE",
);

flags_from!(vk::AccessFlags, AccessFlags,
    vk::AccessFlags::INDIRECT_COMMAND_READ  => AccessFlags::INDIRECT_COMMAND_READ,
    vk::AccessFlags::INDEX_READ             => AccessFlags::INDEX_READ,
    vk::AccessFlags::VERTEX_ATTRIBUTE_READ  => AccessFlags::VERTEX_ATTRIBUTE_READ,
    vk::AccessFlags::UNIFORM_READ           => AccessFlags::UNIFORM_READ,
    vk::AccessFlags::INPUT_ATTACHMENT_READ  => AccessFlags::INPUT_ATTACHMENT_READ,
    vk::AccessFlags::SHADER_READ            => AccessFlags::SHADER_READ,
    vk::AccessFlags::SHADER_WRITE           => AccessFlags::SHADER_WRITE,
    vk::AccessFlags::COLOR_ATTACHMENT_READ  => AccessFlags::COLOUR_ATTACHMENT_READ,
    vk::AccessFlags::COLOR_ATTACHMENT_WRITE => AccessFlags::COLOUR_ATTACHMENT_WRITE,
    vk::AccessFlags::TRANSFER_READ          => AccessFlags::TRANSFER_READ,
    vk::AccessFlags::TRANSFER_WRITE         => AccessFlags::TRANSFER_WRITE,
    vk::AccessFlags::HOST_READ              => AccessFlags::HOST_READ,
    vk::AccessFlags::HOST_WRITE             => AccessFlags::HOST_WRITE,
    vk::AccessFlags::MEMORY_READ            => AccessFlags::MEMORY_READ,
    vk::AccessFlags::MEMORY_WRITE           => AccessFlags::MEMORY_WRITE,
);



/// Defines the kind of dependency that we're defining.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DependencyFlags(u8);

impl DependencyFlags {
    /// The dependency is local to each framebuffer (must be given if the stages include framebuffers).
    pub const FRAMEBUFFER_LOCAL: Self = Self(0x01);
    /// Every subpass has more than one ImageView that needs dependencies (must be given if so).
    pub const VIEW_LOCAL: Self = Self(0x02);
    /// If the dependency is not local to a device, this flag should be given.
    pub const NOT_DEVICE_LOCAL: Self = Self(0x04);
}

impl Flags for DependencyFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u8;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(DependencyFlags,
    DependencyFlags::FRAMEBUFFER_LOCAL => "FRAMEBUFFER_LOCAL",
    DependencyFlags::VIEW_LOCAL        => "VIEW_LOCAL",
    DependencyFlags::NOT_DEVICE_LOCAL  => "NOT_DEVICE_LOCAL",
);

flags_from!(vk::DependencyFlags, DependencyFlags,
    vk::DependencyFlags::BY_REGION    => DependencyFlags::FRAMEBUFFER_LOCAL,
    vk::DependencyFlags::VIEW_LOCAL   => DependencyFlags::VIEW_LOCAL,
    vk::DependencyFlags::DEVICE_GROUP => DependencyFlags::NOT_DEVICE_LOCAL,
);



/// The ShaderStage where a shader or a resource lives.
#[derive(Clone, Copy, Debug)]
pub struct PipelineStage(u32);

impl PipelineStage {
    /// An empty PipelineStage
    pub const EMPTY: Self = Self(0x00000);
    /// A PipelineStage that hits all stages
    pub const ALL: Self   = Self(0xFFFFF);

    /// Defines the stage before anything of the pipeline is run.
    pub const TOP_OF_PIPE: Self = Self(0x00001);
    /// The indirect draw stage.
    pub const DRAW_INDIRECT: Self = Self(0x00002);
    /// The stage where vertices (and indices) are read.
    pub const VERTEX_INPUT: Self = Self(0x00004);
    /// The Vertex shader stage.
    pub const VERTEX_SHADER: Self = Self(0x00008);
    /// The control stage of the Tesselation shader stage.
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0x00010);
    /// The evaluation stage of the Tesselation shader stage.
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0x00020);
    /// The Geometry shader stage.
    pub const GEOMETRY_SHADER: Self = Self(0x00040);
    /// The Fragment shader stage.
    pub const FRAGMENT_SHADER: Self = Self(0x00080);
    /// The stage where early fragments tests (depth and stencil tests before fragment shading) are performed. This stage also performs subpass load operations for framebuffers with depth attachments.
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0x00100);
    /// The stage where late fragments tests (depth and stencil tests after fragment shading) are performed. This stage also performs subpass write operations for framebuffers with depth attachments.
    pub const LATE_FRAGMENT_TESTS: Self = Self(0x00200);
    /// The stage where the fragments are written to the colour attachment (after blending).
    pub const COLOUR_ATTACHMENT_OUTPUT: Self = Self(0x00400);
    /// The stage where any compute shaders may be processed.
    pub const COMPUTE_SHADER: Self = Self(0x00800);
    /// The stage where any data is transferred to and from buffers and images (all copy commands, blit, resolve and clear commands (except vkCmdClearAttachments).
    pub const TRANSFER: Self = Self(0x01000);
    /// Defines the stage after the entire pipeline has been completed.
    pub const BOTTOM_OF_PIPE: Self = Self(0x02000);
    /// A (pseudo-)stage where host access to a device is performed.
    pub const HOST: Self = Self(0x04000);
    /// Collection for all graphics-related stages.
    pub const ALL_GRAPHICS: Self = Self(0x08000);
    /// Collection for all commandbuffer-invoked stages _supported on the executing queue_.
    pub const ALL_COMMANDS: Self = Self(0x10000);


    /// Returns whether the given PipelineStage is a subset of this one.
    /// 
    /// # Arguments
    /// - `value`: The PipelineStage that should be a subset of this one. For example, if value is Self::VERTEX, then returns true if the Vertex shader stage was enabled in this PipelineStage.
    #[inline]
    pub fn check(&self, other: PipelineStage) -> bool { (self.0 & other.0) == other.0 }
}

impl BitOr for PipelineStage {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PipelineStage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl From<vk::PipelineStageFlags> for PipelineStage {
    #[inline]
    fn from(value: vk::PipelineStageFlags) -> Self {
        // Construct it manually for portability
        let mut result = PipelineStage::EMPTY;
        if (value & vk::PipelineStageFlags::TOP_OF_PIPE).as_raw() != 0 { result |= PipelineStage::TOP_OF_PIPE; }
        if (value & vk::PipelineStageFlags::DRAW_INDIRECT).as_raw() != 0 { result |= PipelineStage::DRAW_INDIRECT; }
        if (value & vk::PipelineStageFlags::VERTEX_INPUT).as_raw() != 0 { result |= PipelineStage::VERTEX_INPUT; }
        if (value & vk::PipelineStageFlags::VERTEX_SHADER).as_raw() != 0 { result |= PipelineStage::VERTEX_SHADER; }
        if (value & vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER).as_raw() != 0 { result |= PipelineStage::TESSELLATION_CONTROL_SHADER; }
        if (value & vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER).as_raw() != 0 { result |= PipelineStage::TESSELLATION_EVALUATION_SHADER; }
        if (value & vk::PipelineStageFlags::GEOMETRY_SHADER).as_raw() != 0 { result |= PipelineStage::GEOMETRY_SHADER; }
        if (value & vk::PipelineStageFlags::FRAGMENT_SHADER).as_raw() != 0 { result |= PipelineStage::FRAGMENT_SHADER; }
        if (value & vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS).as_raw() != 0 { result |= PipelineStage::EARLY_FRAGMENT_TESTS; }
        if (value & vk::PipelineStageFlags::LATE_FRAGMENT_TESTS).as_raw() != 0 { result |= PipelineStage::LATE_FRAGMENT_TESTS; }
        if (value & vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT).as_raw() != 0 { result |= PipelineStage::COLOUR_ATTACHMENT_OUTPUT; }
        if (value & vk::PipelineStageFlags::COMPUTE_SHADER).as_raw() != 0 { result |= PipelineStage::COMPUTE_SHADER; }
        if (value & vk::PipelineStageFlags::TRANSFER).as_raw() != 0 { result |= PipelineStage::TRANSFER; }
        if (value & vk::PipelineStageFlags::BOTTOM_OF_PIPE).as_raw() != 0 { result |= PipelineStage::BOTTOM_OF_PIPE; }
        if (value & vk::PipelineStageFlags::HOST).as_raw() != 0 { result |= PipelineStage::HOST; }
        if (value & vk::PipelineStageFlags::ALL_GRAPHICS).as_raw() != 0 { result |= PipelineStage::ALL_GRAPHICS; }
        if (value & vk::PipelineStageFlags::ALL_COMMANDS).as_raw() != 0 { result |= PipelineStage::ALL_COMMANDS; }

        // Return it
        result
    }
}

impl From<PipelineStage> for vk::PipelineStageFlags {
    fn from(value: PipelineStage) -> Self {
        // Construct it manually due to private constructors ;(
        let mut result = vk::PipelineStageFlags::empty();
        if value.check(PipelineStage::TOP_OF_PIPE) { result |= vk::PipelineStageFlags::TOP_OF_PIPE; }
        if value.check(PipelineStage::DRAW_INDIRECT) { result |= vk::PipelineStageFlags::DRAW_INDIRECT; }
        if value.check(PipelineStage::VERTEX_INPUT) { result |= vk::PipelineStageFlags::VERTEX_INPUT; }
        if value.check(PipelineStage::VERTEX_SHADER) { result |= vk::PipelineStageFlags::VERTEX_SHADER; }
        if value.check(PipelineStage::TESSELLATION_CONTROL_SHADER) { result |= vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER; }
        if value.check(PipelineStage::TESSELLATION_EVALUATION_SHADER) { result |= vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER; }
        if value.check(PipelineStage::GEOMETRY_SHADER) { result |= vk::PipelineStageFlags::GEOMETRY_SHADER; }
        if value.check(PipelineStage::FRAGMENT_SHADER) { result |= vk::PipelineStageFlags::FRAGMENT_SHADER; }
        if value.check(PipelineStage::EARLY_FRAGMENT_TESTS) { result |= vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS; }
        if value.check(PipelineStage::LATE_FRAGMENT_TESTS) { result |= vk::PipelineStageFlags::LATE_FRAGMENT_TESTS; }
        if value.check(PipelineStage::COLOUR_ATTACHMENT_OUTPUT) { result |= vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT; }
        if value.check(PipelineStage::COMPUTE_SHADER) { result |= vk::PipelineStageFlags::COMPUTE_SHADER; }
        if value.check(PipelineStage::TRANSFER) { result |= vk::PipelineStageFlags::TRANSFER; }
        if value.check(PipelineStage::BOTTOM_OF_PIPE) { result |= vk::PipelineStageFlags::BOTTOM_OF_PIPE; }
        if value.check(PipelineStage::HOST) { result |= vk::PipelineStageFlags::HOST; }
        if value.check(PipelineStage::ALL_GRAPHICS) { result |= vk::PipelineStageFlags::ALL_GRAPHICS; }
        if value.check(PipelineStage::ALL_COMMANDS) { result |= vk::PipelineStageFlags::ALL_COMMANDS; }

        // Return it
        result
    }
}





/***** PIPELINES *****/
/// Defines the channel mask to use when writing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColourComponentFlags(u8);

impl ColourComponentFlags {
    /// A colour mask for only the red colour channel.
    pub const RED  : Self = Self(0b00000001);
    /// A colour mask for only the green colour channel.
    pub const GREEN: Self = Self(0b00000010);
    /// A colour mask for only the blue colour channel.
    pub const BLUE : Self = Self(0b00000100);
    /// A colour mask for only the alpha channel.
    pub const ALPHA: Self = Self(0b00001000);
}

impl Flags for ColourComponentFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u8;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(ColourComponentFlags,
    ColourComponentFlags::RED   => "RED",
    ColourComponentFlags::GREEN => "GREEN",
    ColourComponentFlags::BLUE  => "BLUE",
    ColourComponentFlags::ALPHA => "ALPHA",
);

flags_from!(vk::ColorComponentFlags, ColourComponentFlags,
    vk::ColorComponentFlags::R => ColourComponentFlags::RED,
    vk::ColorComponentFlags::G => ColourComponentFlags::GREEN,
    vk::ColorComponentFlags::B => ColourComponentFlags::BLUE,
    vk::ColorComponentFlags::A => ColourComponentFlags::ALPHA,
);





/***** MEMORY POOLS *****/
/// Lists properties of certain memory areas.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPropertyFlags(u16);

impl MemoryPropertyFlags {
    /// Memory should be local to the Device (i.e., not some shared memory pool).
    pub const DEVICE_LOCAL: Self = Self(0x0001);
    /// Memory should be writeable/readable by the Host.
    pub const HOST_VISIBLE: Self = Self(0x0002);
    /// Memory should be coherent with the host (not requiring separate flush calls).
    pub const HOST_COHERENT: Self = Self(0x0004);
    /// Memory is cached, which is faster but non-coherent.
    pub const HOST_CACHED: Self = Self(0x0008);
    /// Memory might need to be allocated on first access.
    pub const LAZILY_ALLOCATED: Self = Self(0x0010);
    /// Memory is protected; only Device may access it and some special queue operations.
    pub const PROTECTED: Self = Self(0x0020);
}

impl Flags for MemoryPropertyFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u16;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(MemoryPropertyFlags,
    MemoryPropertyFlags::DEVICE_LOCAL     => "DEVICE_LOCAL",
    MemoryPropertyFlags::HOST_VISIBLE     => "HOST_VISIBLE",
    MemoryPropertyFlags::HOST_COHERENT    => "HOST_COHERENT",
    MemoryPropertyFlags::HOST_CACHED      => "HOST_CACHED",
    MemoryPropertyFlags::LAZILY_ALLOCATED => "LAZILY_ALLOCATED",
    MemoryPropertyFlags::PROTECTED        => "PROTECTED",
);

flags_from!(vk::MemoryPropertyFlags, MemoryPropertyFlags, 
    vk::MemoryPropertyFlags::DEVICE_LOCAL     => MemoryPropertyFlags::DEVICE_LOCAL,
    vk::MemoryPropertyFlags::HOST_VISIBLE     => MemoryPropertyFlags::HOST_VISIBLE,
    vk::MemoryPropertyFlags::HOST_COHERENT    => MemoryPropertyFlags::HOST_COHERENT,
    vk::MemoryPropertyFlags::HOST_CACHED      => MemoryPropertyFlags::HOST_CACHED,
    vk::MemoryPropertyFlags::LAZILY_ALLOCATED => MemoryPropertyFlags::LAZILY_ALLOCATED,
    vk::MemoryPropertyFlags::PROTECTED        => MemoryPropertyFlags::PROTECTED,
);





/***** COMMANDS POOLS *****/
/// Flags for the CommandPool construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CommandBufferFlags(u8);

impl CommandBufferFlags {
    /// The buffers coming from this CommandPool will be short-lived.
    pub const TRANSIENT: Self = Self(0x01);
    /// The buffers coming from this CommandPool may be individually reset instead of only all at once by resetting the pool.
    pub const ALLOW_RESET: Self = Self(0x02);
}

impl Flags for CommandBufferFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u8;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(CommandBufferFlags,
    CommandBufferFlags::TRANSIENT   => "TRANSIENT",
    CommandBufferFlags::ALLOW_RESET => "ALLOW_RESET",
);

flags_from!(vk::CommandPoolCreateFlags, CommandBufferFlags,
    vk::CommandPoolCreateFlags::TRANSIENT            => CommandBufferFlags::TRANSIENT,
    vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER => CommandBufferFlags::ALLOW_RESET,
);



/// Flags to set options when beginning a command buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandBufferUsageFlags(u8);

impl CommandBufferUsageFlags {
    /// Tells the Vulkan driver that this command buffer will only be submitted once, and reset or destroyed afterwards.
    pub const ONE_TIME_SUBMIT: Self = Self(0x01);
    /// If the CommandBuffer is secondary, then this bit indicates that it lives entirely within the RenderPass.
    pub const RENDER_PASS_ONLY: Self = Self(0x02);
    /// The buffer can be resubmitted while it is pending and recorded into multiple primary command buffers.
    pub const SIMULTANEOUS_USE: Self = Self(0x04);
}

impl Flags for CommandBufferUsageFlags {
    /// Determines the type of the internal value where the flags are stored.
    type RawType = u8;


    /// Constructor for the Flags object that creates it from a raw value.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::as_raw()`.
    /// 
    /// # Arguments
    /// - `value`: The raw value (of type `T`) around which to construct this Flags.
    /// 
    /// # Returns
    /// A new instance of Self with the flags set as in the raw value.
    #[inline]
    fn from_raw(value: Self::RawType) -> Self { Self(value) }

    /// Returns the raw integer with the flags that is at the core of the Flags.
    /// 
    /// Note that this is a _Game_ raw flags rather than a _Vulkan_ raw flags; the two might not align! The only guarantee made by this raw value is that it is compatible with that of `Flags::from_raw()`.
    /// 
    /// # Returns
    /// The raw value at the heart of this Flags.
    #[inline]
    fn as_raw(&self) -> Self::RawType { self.0 }
}

flags_display!(CommandBufferUsageFlags,
    CommandBufferUsageFlags::ONE_TIME_SUBMIT  => "ONE_TIME_SUBMIT",
    CommandBufferUsageFlags::RENDER_PASS_ONLY => "RENDER_PASS_ONLY",
    CommandBufferUsageFlags::SIMULTANEOUS_USE => "SIMULTANEOUS_USE",
);

flags_from!(vk::CommandBufferUsageFlags, CommandBufferUsageFlags,
    vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT      => CommandBufferUsageFlags::ONE_TIME_SUBMIT,
    vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE => CommandBufferUsageFlags::RENDER_PASS_ONLY,
    vk::CommandBufferUsageFlags::SIMULTANEOUS_USE     => CommandBufferUsageFlags::SIMULTANEOUS_USE,
);
