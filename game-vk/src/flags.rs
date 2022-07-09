/* FLAGS.rs
 *   by Lut99
 *
 * Created:
 *   09 Jul 2022, 10:44:36
 * Last edited:
 *   09 Jul 2022, 11:41:33
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





/***** MEMORY *****/
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
