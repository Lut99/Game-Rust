/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   03 Jul 2022, 16:59:53
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and definitions for the MemoryPools.
**/

use std::ffi::c_void;
use std::fmt::{Debug, Formatter, Result as FResult};
use std::ops::{Add, AddAssign};
use std::ptr;
use std::rc::Rc;
use std::slice;
use std::sync::{Arc, RwLock};

use ash::vk;
use log::warn;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{BufferUsageFlags, CommandBufferFlags, CommandBufferUsageFlags, MemoryPropertyFlags, MemoryRequirements, SharingMode};
use crate::device::Device;
use crate::pools::command::{Buffer as CommandBuffer, Pool as CommandPool};


/***** UNIT TESTS *****/
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests GpuPtr's initializers
    #[test]
    fn test_gpuptr_init() {
        // Test new first
        assert_eq!(GpuPtr::new(0, 0, 0).as_raw(), 0);
        assert_eq!(GpuPtr::new(0, 0, 0x42).as_raw(), 0x42);
        assert_eq!(GpuPtr::new(0, 0, 0xFFFFFFFFFFFF).as_raw(), 0xFFFFFFFFFFFF);
        assert_eq!(GpuPtr::new(0, 5, 0x42).as_raw(), 0x0005000000000042);
        assert_eq!(GpuPtr::new(5, 0, 0x42).as_raw(), 0x2800000000000042);
        assert_eq!(GpuPtr::new(5, 5, 0x42).as_raw(), 0x2805000000000042);
        assert_eq!(GpuPtr::new(31, 2047, 0x42).as_raw(), 0xFFFF000000000042);
        assert_eq!(GpuPtr::new(31, 1023, 0x42).as_raw(), 0xFBFF000000000042);

        // Test the null constructor
        assert_eq!(GpuPtr::null().is_null(), true);
        assert_eq!(GpuPtr::new(5, 5, 0x42).is_null(), false);

        // Test the aligned constructor
        assert_eq!(GpuPtr::aligned(5, 5, 0x42, 16).as_raw(), 0x2805000000000050);
    }

    /// Tests GpuPtr's `align` and `agnostic` functions
    #[test]
    fn test_operators() {
        // Test align
        assert_eq!(GpuPtr::new(5, 0, 0x42).align(1).as_raw(),   0x2800000000000042);
        assert_eq!(GpuPtr::new(5, 0, 0x42).align(4).as_raw(),   0x2800000000000044);
        assert_eq!(GpuPtr::new(5, 0, 0x42).align(16).as_raw(),  0x2800000000000050);

        // Test agnostic
        assert_eq!(GpuPtr::new(0, 0, 0x42).agnostic().as_raw(), 0x42);
        assert_eq!(GpuPtr::new(5, 0, 0x42).agnostic().as_raw(), 0x42);
        assert_eq!(GpuPtr::new(0, 0, 0x42).agnostic().as_raw(), 0x42);
        assert_eq!(GpuPtr::new(5, 5, 0x42).agnostic().as_raw(), 0x42);
    }

    /// Tests GpuPtr's `set_*` functions
    #[test]
    fn test_set() {
        // Test set_type_idx
        let mut ptr1 = GpuPtr::new(0, 0, 0   ); ptr1.set_type_idx(15);
        let mut ptr2 = GpuPtr::new(5, 0, 0   ); ptr2.set_type_idx(15);
        let mut ptr3 = GpuPtr::new(0, 5, 0x42); ptr3.set_type_idx(15);
        let mut ptr4 = GpuPtr::new(5, 5, 0x42); ptr4.set_type_idx(15);
        assert_eq!(ptr1, GpuPtr::new(15, 0, 0));
        assert_eq!(ptr2, GpuPtr::new(15, 0, 0));
        assert_eq!(ptr3, GpuPtr::new(15, 5, 0x42));
        assert_eq!(ptr4, GpuPtr::new(15, 5, 0x42));
        
        // Test set_pool_idx
        let mut ptr1 = GpuPtr::new(0, 0, 0   ); ptr1.set_pool_idx(15);
        let mut ptr2 = GpuPtr::new(5, 0, 0   ); ptr2.set_pool_idx(15);
        let mut ptr3 = GpuPtr::new(0, 5, 0x42); ptr3.set_pool_idx(15);
        let mut ptr4 = GpuPtr::new(5, 5, 0x42); ptr4.set_pool_idx(15);
        assert_eq!(ptr1, GpuPtr::new(0, 15, 0));
        assert_eq!(ptr2, GpuPtr::new(5, 15, 0));
        assert_eq!(ptr3, GpuPtr::new(0, 15, 0x42));
        assert_eq!(ptr4, GpuPtr::new(5, 15, 0x42));

        // Test set_ptr
        let mut ptr1 = GpuPtr::new(0, 0, 0   ); ptr1.set_ptr(0x84);
        let mut ptr2 = GpuPtr::new(5, 0, 0   ); ptr2.set_ptr(0x84);
        let mut ptr3 = GpuPtr::new(0, 5, 0x42); ptr3.set_ptr(0x84);
        let mut ptr4 = GpuPtr::new(5, 5, 0x42); ptr4.set_ptr(0x84);
        assert_eq!(ptr1, GpuPtr::new(0, 0, 0x84));
        assert_eq!(ptr2, GpuPtr::new(5, 0, 0x84));
        assert_eq!(ptr3, GpuPtr::new(0, 5, 0x84));
        assert_eq!(ptr4, GpuPtr::new(5, 5, 0x84));
    }

    /// Tests GpuPtr's arithmetic operators
    #[test]
    fn test_arithmetic() {
        // Test normal add
        assert_eq!(GpuPtr::new(0, 0, 0   ) + GpuPtr::new(0, 0, 0   ), GpuPtr::new(0, 0, 0   ));
        assert_eq!(GpuPtr::new(0, 0, 0x42) + GpuPtr::new(0, 0, 0   ), GpuPtr::new(0, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 0, 0   ) + GpuPtr::new(0, 0, 0x42), GpuPtr::new(0, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 0, 0x42) + GpuPtr::new(0, 0, 0x42), GpuPtr::new(0, 0, 0x84));
        assert_eq!(GpuPtr::new(5, 0, 0x42) + GpuPtr::new(5, 0, 0   ), GpuPtr::new(5, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 5, 0   ) + GpuPtr::new(0, 5, 0x42), GpuPtr::new(0, 5, 0x42));
        assert_eq!(GpuPtr::new(5, 5, 0x42) + GpuPtr::new(5, 5, 0x42), GpuPtr::new(5, 5, 0x84));
        
        // Test assign add
        let mut ptr1 = GpuPtr::new(0, 0, 0   ); ptr1 += GpuPtr::new(0, 0, 0   );
        let mut ptr2 = GpuPtr::new(0, 0, 0x42); ptr2 += GpuPtr::new(0, 0, 0   );
        let mut ptr3 = GpuPtr::new(0, 0, 0   ); ptr3 += GpuPtr::new(0, 0, 0x42);
        let mut ptr4 = GpuPtr::new(0, 0, 0x42); ptr4 += GpuPtr::new(0, 0, 0x42);
        let mut ptr5 = GpuPtr::new(5, 0, 0x42); ptr5 += GpuPtr::new(5, 0, 0   );
        let mut ptr6 = GpuPtr::new(0, 5, 0   ); ptr6 += GpuPtr::new(0, 5, 0x42);
        let mut ptr7 = GpuPtr::new(5, 5, 0x42); ptr7 += GpuPtr::new(5, 5, 0x42);
        assert_eq!(ptr1, GpuPtr::new(0, 0, 0   ));
        assert_eq!(ptr2, GpuPtr::new(0, 0, 0x42));
        assert_eq!(ptr3, GpuPtr::new(0, 0, 0x42));
        assert_eq!(ptr4, GpuPtr::new(0, 0, 0x84));
        assert_eq!(ptr5, GpuPtr::new(5, 0, 0x42));
        assert_eq!(ptr6, GpuPtr::new(0, 5, 0x42));
        assert_eq!(ptr7, GpuPtr::new(5, 5, 0x84));
        
        // Test normal add, but now for usizes
        assert_eq!(GpuPtr::new(0, 0, 0   ) + 0   , GpuPtr::new(0, 0, 0   ));
        assert_eq!(GpuPtr::new(0, 0, 0x42) + 0   , GpuPtr::new(0, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 0, 0   ) + 0x42, GpuPtr::new(0, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 0, 0x42) + 0x42, GpuPtr::new(0, 0, 0x84));
        assert_eq!(GpuPtr::new(5, 0, 0x42) + 0   , GpuPtr::new(5, 0, 0x42));
        assert_eq!(GpuPtr::new(0, 5, 0   ) + 0x42, GpuPtr::new(0, 5, 0x42));
        assert_eq!(GpuPtr::new(5, 5, 0x42) + 0x42, GpuPtr::new(5, 5, 0x84));
        
        // Test assign add, but now for usizes
        let mut ptr1 = GpuPtr::new(0, 0, 0   ); ptr1 += 0;
        let mut ptr2 = GpuPtr::new(0, 0, 0x42); ptr2 += 0;
        let mut ptr3 = GpuPtr::new(0, 0, 0   ); ptr3 += 0x42;
        let mut ptr4 = GpuPtr::new(0, 0, 0x42); ptr4 += 0x42;
        let mut ptr5 = GpuPtr::new(5, 0, 0x42); ptr5 += 0;
        let mut ptr6 = GpuPtr::new(0, 5, 0   ); ptr6 += 0x42;
        let mut ptr7 = GpuPtr::new(5, 5, 0x42); ptr7 += 0x42;
        assert_eq!(ptr1, GpuPtr::new(0, 0, 0   ));
        assert_eq!(ptr2, GpuPtr::new(0, 0, 0x42));
        assert_eq!(ptr3, GpuPtr::new(0, 0, 0x42));
        assert_eq!(ptr4, GpuPtr::new(0, 0, 0x84));
        assert_eq!(ptr5, GpuPtr::new(5, 0, 0x42));
        assert_eq!(ptr6, GpuPtr::new(0, 5, 0x42));
        assert_eq!(ptr7, GpuPtr::new(5, 5, 0x84));
    }
}





/***** HELPER MACROS *****/
/// Checks if an u8 overflows for a type index.value.0 as Self
macro_rules! assert_type_idx_overflow {
    ($type_idx:expr) => {
        assert_type_idx_overflow!($type_idx, false)
    };

    ($type_idx:expr, $err:expr) => {
        if $type_idx & !0x1F != 0 {
            if $err { panic!("Type index '{:#X}' ({}) overflows for a 5-bit integer", $type_idx, $type_idx); }
            else { warn!("Given type index '{:#X}' ({}) overflows for a 5-bit integer", $type_idx, $type_idx); }
        }
    };
}

/// Checks if an u16 overflows for a pool index.
macro_rules! assert_pool_idx_overflow {
    ($pool_idx:expr) => {
        assert_pool_idx_overflow!($pool_idx, false)
    };

    ($pool_idx:expr, $err:expr) => {
        if $pool_idx & !0x7FF != 0 {
            if $err { panic!("Pool index '{:#X}' ({}) overflows for an 11-bit integer", $pool_idx, $pool_idx); }
            else { warn!("Given pool index '{:#X}' ({}) overflows for an 11-bit integer", $pool_idx, $pool_idx); }
        }
    }
}

/// Checks if an u16 overflows for a potype_idxinter value.
macro_rules! assert_ptr_overflow {
    ($ptr:expr) => {
        assert_ptr_overflow!($ptr, false)
    };

    ($ptr:expr, $err:expr) => {
        if $ptr & !0xFFFFFFFFFFFF != 0 {
            if $err { panic!("Given pointer value '{:#X}' ({}) overflows for an 48-bit integer", $ptr, $ptr); }
            else { warn!("Given pointer value '{:#X}' ({}) overflows for an 48-bit integer", $ptr, $ptr); }
        }
    };
}




/***** POPULATE FUNCTIONS *****/
/// Populates the given VkBufferCopy struct.
/// 
/// # Arguments
/// - `src_offset`: The offset of the region in the source buffer.
/// - `dst_offset`: The offset of the region in the destination buffer.
/// - `size`: The size of the region (in bytes).
#[inline]
fn populate_buffer_copy(src_offset: vk::DeviceSize, dst_offset: vk::DeviceSize, size: vk::DeviceSize) -> vk::BufferCopy {
    vk::BufferCopy {
        src_offset,
        dst_offset,
        size,
    }
}

/// Populates a new VkMappedMemoryRange struct with the given values.
/// 
/// # Arguments
/// - `memory`: The VkDeviceMemory where the range to flush is mapped to.
/// - `offset`: The offset of the range to flush.
/// - `size`: The size of the range to flush.
#[inline]
fn populate_mapped_memory_range(memory: vk::DeviceMemory, offset: vk::DeviceSize, size: vk::DeviceSize) -> vk::MappedMemoryRange {
    vk::MappedMemoryRange {
        s_type : vk::StructureType::MAPPED_MEMORY_RANGE,
        p_next : ptr::null(),

        // Set the range properties
        memory,
        offset,
        size,
    }
}





/***** LIBRARY *****/
/// The type of pointers used across the pools.
/// 
/// We current use 64-bit pointers, which we split into one number of 5-bit, one of 11 bits and one of 48 bits:
/// - The first number determines the memory type used (in the case of a non-meta pool, always 0's)
/// - The second number determines the block pool used within that type (in the case of a non-meta pool, always 0's)
/// - The third number determines the pointer within that pool.
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GpuPtr(u64);

impl GpuPtr {
    /// Constructs a new GpuPtr with the appropriate values set
    /// 
    /// # Arguments
    /// - `type_idx`: The index of the memory type (only 5 rightmost bits will be used).
    /// - `pool_idx`: The index of the memory pool (only 11 rightmost bits will be used).
    /// - `ptr`: The actual 48-bit pointer value (only 48 rightmost bits will be used).
    /// 
    /// # Returns
    /// A new GpuPtr encoding the given values.
    /// 
    /// # Warnings
    /// This function may throw `log` warnings to indicate passing values that have unused bits set.
    pub fn new(type_idx: u8, pool_idx: u16, ptr: u64) -> Self {
        // Sanity checks for the values
        assert_type_idx_overflow!(type_idx);
        assert_pool_idx_overflow!(pool_idx);
        assert_ptr_overflow!(ptr);

        // Combine them all in a new Self
        Self(
            (((type_idx as u64) & 0x1F) << (64 - 5)) |
            (((pool_idx as u64) & 0x7FF) << (64 - 16)) |
            ((ptr as u64) & 0xFFFFFFFFFFFF)
        )
    }

    /// Creates a new GpuPtr that is the NULL pointer (the `ptr`-part is all 1's).
    /// 
    /// # Returns
    /// A new GpuPtr that represents the NULL pointer.
    #[inline]
    pub fn null() -> Self {
        Self::new(0, 0, 0xFFFFFFFFFFFF)
    }

    /// Creates an aligned version of the given pointer.
    /// 
    /// Shortcut for using `GpuPtr::new()` and then `GpuPtr::align()`.
    /// 
    /// # Arguments
    /// - `type_idx`: The index of the memory type (only 5 rightmost bits will be used).
    /// - `pool_idx`: The index of the memory pool (only 11 rightmost bits will be used).
    /// - `ptr`: The actual 48-bit pointer value (only 48 rightmost bits will be used).
    /// - `align`: The alignment (as a power of 2) of the `ptr`.
    /// 
    /// # Returns
    /// A new GpuPtr encoding the given values, where `ptr` is aligned to the given alignment.
    /// 
    /// # Warnings
    /// This function may throw `log` warnings to indicate passing values that have unused bits set.
    /// 
    /// # Panics
    /// This function panics if the `align` is not a power of 2.
    #[inline]
    pub fn aligned(type_idx: u8, pool_idx: u16, ptr: u64, align: u8) -> Self {
        Self::new(type_idx, pool_idx, ptr).align(align)
    }



    /// Returns an aligned version of the pointer.
    /// 
    /// # Arguments
    /// - `align`: The number to align the pointer to. Must be a power of 2.
    /// 
    /// # Returns
    /// The value of this GpuPtr rounded up on the given boundry.
    /// 
    /// # Panics
    /// This function panics if `align` is not a power of 2.
    #[inline]
    pub fn align(&self, align: u8) -> Self {
        if align != 0 {
            if (align & (align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", align); }
            Self((self.0 + ((align as u64) - 1)) & ((!(align as u64)) + 1))
        } else {
            Self(self.0)
        }
    }

    /// Returns a copy of the GpuPtr, but without any type or pool indices set.
    #[inline]
    pub fn agnostic(&self) -> Self { Self(self.0 & 0xFFFFFFFFFFFF) }



    /// Sets the value of the type_idx.
    /// 
    /// # Arguments
    /// - `type_idx`; The new `type_idx` for this GpuPtr. Only the last 5 bits are used.
    /// 
    /// # Warnings
    /// This function may throws `log::warn` if the given `type_idx` would overflow for a 5-bit unsigned integer.
    pub fn set_type_idx(&mut self, type_idx: u8) {
        // Sanity check
        assert_type_idx_overflow!(type_idx);

        // Set the value
        self.0 = (self.0 & (!(0x1F << (64 - 5)))) | (((type_idx as u64) & 0x1F) << (64 - 5));
    }

    /// Sets the value of the pool_idx.
    /// 
    /// # Arguments
    /// - `pool_idx`; The new `pool_idx` for this GpuPtr. Only the last 11 bits are used.
    /// 
    /// # Warnings
    /// This function may throw `log::warn` if the given `pool_idx` would overflow for an 11-bit unsigned integer.
    pub fn set_pool_idx(&mut self, pool_idx: u16) {
        // Sanity check
        assert_pool_idx_overflow!(pool_idx);

        // Set the value
        self.0 = (self.0 & (!(0x7FF << (64 - 16)))) | (((pool_idx as u64) & 0x7FF) << (64 - 16));
    }

    /// Sets the value of the ptr.
    /// 
    /// # Arguments
    /// - `ptr`; The new `ptr` for this GpuPtr. Only the last 48 bits are used.
    /// 
    /// # Warnings
    /// This function may throw `log::warn` if the given `ptr` would overflow for a 48-bit unsigned integer.
    pub fn set_ptr(&mut self, ptr: u64) {
        // Sanity check
        assert_ptr_overflow!(ptr);

        // Set the value
        self.0 = (self.0 & (!0xFFFFFFFFFFFF)) | (ptr & 0xFFFFFFFFFFFF);
    }



    /// Returns the type index of the GpuPtr.
    #[inline]
    pub fn type_idx(&self) -> u8 { ((self.0 >> (64 - 5)) & 0x1F) as u8 }

    /// Returns the pool index of the GpuPtr.
    #[inline]
    pub fn pool_idx(&self) -> u16 { ((self.0 >> (64 - 16)) & 0x7FF) as u16 }

    /// Returns the actual pointer value of the GpuPtr.
    #[inline]
    pub fn ptr(&self) -> u64 { self.0 & 0xFFFFFFFFFFFF }

    /// Returns whether or not this GpuPtr represents the NULL-pointer.
    /// 
    /// This is the case iff `ptr` (the last 48-bits) is all 1's, which implies that NULL-pointers are still type & pool specific.
    #[inline]
    pub fn is_null(&self) -> bool { self.0 & 0xFFFFFFFFFFFF == 0xFFFFFFFFFFFF }

    /// Returns the raw number inside the GpuPtr.
    #[inline]
    pub fn as_raw(&self) -> u64 { self.0 }
}

impl Default for GpuPtr {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl Debug for GpuPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Split the value into the type index, pool index & the actual pointer
        let type_idx: u8  = self.type_idx();
        let pool_idx: u16 = self.pool_idx();
        let ptr: u64      = self.ptr();

        // Only print the indices if non-zero, print the pointer always
        if type_idx > 0 { write!(f, "T{}", type_idx)?; }
        if pool_idx > 0 { write!(f, "P{}", pool_idx)?; }
        write!(f, "{:#X}", ptr)
    }
}

impl Add for GpuPtr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Sanity check
        if self.0 & (0xFFFF << (64 - 16)) != rhs.0 & (0xFFFF << (64 - 16)) { warn!("Attempting to add two GpuPtr's with differing type/pool indices (T{}P{} + T{}P{})", self.type_idx(), self.pool_idx(), rhs.type_idx(), rhs.pool_idx()); }

        // Fetch the ptr-parts
        let lhs_ptr: u64 = self.0 & 0xFFFFFFFFFFFF;
        let rhs_ptr: u64 = rhs.0  & 0xFFFFFFFFFFFF;

        // Update with a sanity check
        let res_ptr: u64 = lhs_ptr + rhs_ptr;
        assert_ptr_overflow!(res_ptr, true);

        // Construct the new self
        Self(
            (self.0 & (0xFFFF << (64 - 16))) |
            res_ptr
        )
    }
}

impl AddAssign for GpuPtr {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Add<usize> for GpuPtr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        // Fetch the ptr-part
        let lhs_ptr: u64 = self.0 & 0xFFFFFFFFFFFF;
        let rhs_ptr: u64 = rhs as u64;

        // Update with a sanity check
        let res_ptr: u64 = lhs_ptr + rhs_ptr;
        assert_ptr_overflow!(res_ptr, true);

        // Construct the new self
        Self(
            (self.0 & (0xFFFF << (64 - 16))) |
            res_ptr
        )
    }
}

impl AddAssign<usize> for GpuPtr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs)
    }
}

impl From<usize> for GpuPtr {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(0, 0, value as u64)
    }
}

impl From<GpuPtr> for usize {
    #[inline]
    fn from(value: GpuPtr) -> Self {
        value.ptr() as Self
    }
}

impl From<vk::DeviceSize> for GpuPtr {
    #[inline]
    fn from(value: vk::DeviceSize) -> Self {
        Self::new(0, 0, value as u64)
    }
}

impl From<GpuPtr> for vk::DeviceSize {
    #[inline]
    fn from(value: GpuPtr) -> Self {
        value.ptr() as Self
    }
}





/// The MemoryPool trait which we use to define common access to a MemoryPool.
pub trait MemoryPool {
    /// Returns a newly allocated area of (at least) the requested size.
    /// 
    /// # Arguments
    /// - `reqs`: The memory requirements of the new memory block.
    /// - `props`: Any desired memory properties for this memory block.
    /// 
    /// # Returns
    /// A tuple with the VkDeviceMemory where the new block of memory is allocated on `.0`, and the index in this memory block on `.1`.
    /// 
    /// # Errors
    /// This function errors if the MemoryPool failed to allocate new memory.
    fn allocate(&mut self, reqs: &MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, GpuPtr), Error>;

    /// Frees an allocated bit of memory.
    /// 
    /// Note that not all types of pools may actually do anything with this. A LinearPool, for example, might deallocate but will never re-use that memory until reset anyway.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer to the block that was allocated.
    /// 
    /// # Panics
    /// This function may panic if the given pointer was never allocated with this pool.
    fn free(&mut self, pointer: GpuPtr);

    /// Resets the memory pool back to its initial, empty state.
    fn reset(&mut self);



    /// Returns the device of the pool.
    fn device(&self) -> &Rc<Device>;

    /// Returns the used space in the pool.
    fn size(&self) -> usize;

    /// Returns the total space in the pool.
    fn capacity(&self) -> usize;
}





/// The Buffer trait, which unifies the interface to various types of Buffers.
pub trait Buffer {
    /// Returns the Device where the Buffer lives.
    fn device(&self) -> &Rc<Device>;
    
    /// Returns the MemoryPool where the Buffer's memory is allocated.
    fn pool(&self) -> &Arc<RwLock<dyn MemoryPool>>;



    /// Returns the Vulkan vk::Buffer which we wrap.
    fn vk(&self) -> vk::Buffer;

    /// Returns the Vulkan vk::DeviceMemory which we also wrap.
    fn vk_mem(&self) -> vk::DeviceMemory;

    /// Returns the offset of this Buffer in the DeviceMemory.
    fn vk_offset(&self) -> vk::DeviceSize;



    /// Returns the usage flags for this Buffer.
    fn usage(&self) -> BufferUsageFlags;

    /// Returns the usage flags for this Buffer.
    fn sharing_mode(&self) -> &SharingMode;

    /// Returns the memory requirements for this Buffer.
    fn requirements(&self) -> &MemoryRequirements;

    /// Returns the memory properties of the memory underlying this Buffer.
    fn properties(&self) -> MemoryPropertyFlags;

    /// Returns the actually allocated size of the buffer.
    fn capacity(&self) -> usize;
}



/// The TransferBuffer trait implements functions for a Buffer that may transfer data to or from it on the GPU.
pub trait TransferBuffer: Buffer {
    /// Schedules a copy of a part of this Buffer's contents to the given Buffer.
    /// 
    /// Note that the command buffer is not yet submitted; that should be done manually.
    /// 
    /// # Arguments
    /// - `cmd`: The CommandBuffer on which the memory transfer is scheduled.
    /// - `target`: The Buffer to write this Buffer's contents to.
    /// - `src_offset`: The offset (in bytes) of the range in the _source_ buffer which we should actually copy.
    /// - `dst_offset`: The offset (in bytes) of the range in the _destination_ buffer which we should actually copy.
    /// - `size`: The size (in bytes) of the range which we should actually copy.
    /// 
    /// # Returns
    /// This function returns nothing on success, except that the CommandBuffer has been appropriately recorded.
    /// 
    /// # Errors
    /// This function may error if the transfer somehow failed.
    /// 
    /// # Panics
    /// This function panics if the given Buffer is not large enough.
    fn schedule_copyto_range(&self, cmd: &Rc<CommandBuffer>, target: &Rc<dyn TransferBuffer>, src_offset: usize, dst_offset: usize, size: usize) {
        // Prepare the region to copy
        let buffer_copy: vk::BufferCopy = populate_buffer_copy(src_offset as vk::DeviceSize, dst_offset as vk::DeviceSize, size as vk::DeviceSize);

        // Schedule the copy on the command buffer
        self.device().cmd_copy_buffer(cmd.vk(), self.vk(), target.vk(), &[ buffer_copy ]);
    }

    /// Copies a part of this Buffer's contents to the given Buffer.
    /// 
    /// # Arguments
    /// - `pool`: The CommandPool that is used to get command buffer(s) to transfer the memory around. The resulting buffers will be recorded and submitted, and the thread then blocks until the copy is complete.
    /// - `target`: The Buffer to write this Buffer's contents to.
    /// - `src_offset`: The offset (in bytes) of the range in the _source_ buffer which we should actually copy.
    /// - `dst_offset`: The offset (in bytes) of the range in the _destination_ buffer which we should actually copy.
    /// - `size`: The size (in bytes) of the range which we should actually copy.
    /// 
    /// # Returns
    /// This function returns nothing on success, except that the target's Buffer's contents will have changed.
    /// 
    /// # Errors
    /// This function may error if the transfer somehow failed.
    /// 
    /// # Panics
    /// This function panics if the given Buffer is not large enough.
    fn copyto_range(&self, pool: &Arc<RwLock<CommandPool>>, target: &Rc<dyn TransferBuffer>, src_offset: usize, dst_offset: usize, size: usize) -> Result<(), Error> {
        // Allocate a new command buffer
        let cmd: Rc<CommandBuffer> = match CommandBuffer::new(self.device().clone(), pool.clone(), self.device().families().memory, CommandBufferFlags::TRANSIENT) {
            Ok(cmd)  => cmd,
            Err(err) => { return Err(Error::CommandBufferError{ what: "transfer", err }); }
        };

        // Schedule the copy
        cmd.begin(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.schedule_copyto_range(&cmd, target, src_offset, dst_offset, size);
        cmd.end();

        // Submit the command buffer and wait until it is completed
        self.device().queues().memory.submit(&cmd, &[], &[], None);
        self.device().queues().memory.drain();

        // Done
        Ok(())
    }



    /// Schedules a copy of this Buffer's (entire) contents to the given Buffer.
    /// 
    /// Note that the command buffer is not yet submitted; that should be done manually.
    /// 
    /// # Arguments
    /// - `cmd`: The CommandBuffer on which the memory transfer is scheduled.
    /// - `target`: The Buffer to write this Buffer's contents to.
    /// 
    /// # Returns
    /// This function returns nothing on success, except that the CommandBuffer has been appropriately recorded.
    /// 
    /// # Errors
    /// This function may error if the transfer somehow failed.
    /// 
    /// # Panics
    /// This function panics if the given Buffer is not large enough.
    #[inline]
    fn schedule_copyto(&self, cmd: &Rc<CommandBuffer>, target: &Rc<dyn TransferBuffer>) {
        // Call the `schedule_copyto_range()` with the entire range
        self.schedule_copyto_range(cmd, target, 0, 0, self.capacity())
    }

    /// Copies this Buffer's (entire) contents to the given Buffer.
    /// 
    /// # Arguments
    /// - `pool`: The CommandPool that is used to get command buffer(s) to transfer the memory around. The resulting buffers will be recorded and submitted, and the thread then blocks until the copy is complete.
    /// - `target`: The Buffer to write this Buffer's contents to.
    /// 
    /// # Returns
    /// This function returns nothing on success, except that the target's Buffer's contents will have changed.
    /// 
    /// # Errors
    /// This function may error if the transfer somehow failed.
    /// 
    /// # Panics
    /// This function panics if the given Buffer is not large enough.
    #[inline]
    fn copyto(&self, pool: &Arc<RwLock<CommandPool>>, target: &Rc<dyn TransferBuffer>) -> Result<(), Error> {
        // Call the `copyto_range()` with the entire range
        self.copyto_range(pool, target, 0, 0, self.capacity())
    }
}



/// The HostBuffer trait implements functions for a Buffer that are possible when the Buffer memory is host-visible.
pub trait HostBuffer: Buffer {
    /// Maps the Buffer memory to host memory.
    /// 
    /// # Returns
    /// A pointer to the new Host-memory area.
    /// 
    /// # Errors
    /// This function may error if we failed to map the Buffer memory.
    #[inline]
    fn map(&self) -> Result<*mut c_void, Error> {
        // Simply call the map function
        match self.device().map_memory(self.vk_mem(), self.vk_offset(), self.capacity() as vk::DeviceSize, vk::MemoryMapFlags::empty()) {
            Ok(ptr)  => Ok(ptr),
            Err(err) => Err(Error::BufferMapError{ err }),
        }
    }

    /// Maps the Buffer memory to host memory.
    /// 
    /// This variant already casts the pointer to a slice of the given object type.
    /// 
    /// # Arguments
    /// - `size`: The number of elements we already expect to be allocated in the range.
    /// 
    /// # Returns
    /// A slice to the new Host-memory area.
    /// 
    /// # Errors
    /// This function may error if we failed to map the Buffer memory.
    /// 
    /// # Panics
    /// This function may panic if the size overflows the mapped memory area's size.
    fn map_slice<T>(&self, size: usize) -> Result<&mut [T], Error> {
        // Check if the size does not overflow
        if size * std::mem::size_of::<T>() > self.capacity() { panic!("A buffer of {} bytes is too small to fit {} elements of {} bytes each ({} bytes total)", self.capacity(), size, std::mem::size_of::<T>(), size * std::mem::size_of::<T>()); }

        // Simply call the map function
        Ok(unsafe { slice::from_raw_parts_mut(self.map()? as *mut T, size) })
    }

    /// Flushes the host-mapped memory area.
    /// 
    /// Note that, if the underlying memory is actually coherent, this function does nothing significant.
    /// 
    /// # Errors
    /// This function may error if there was not enough host memory to perform the flush.
    /// 
    /// # Panics
    /// This function may panic if the memory was not actually mapped.
    #[inline]
    fn flush(&self) -> Result<(), Error> {
        // Call the flush function
        match self.device().flush_mapped_memory_ranges(&[
            populate_mapped_memory_range(self.vk_mem(), self.vk_offset(), self.capacity() as vk::DeviceSize),
        ]) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Error::BufferFlushError{ err }),
        }
    }

    /// Unmaps the Buffer's memory area.
    /// 
    /// # Panics
    /// This function may panic if the memory was not actually mapped.
    #[inline]
    fn unmap(&self) {
        // Simply call unmap
        unsafe { self.device().unmap_memory(self.vk_mem()); }
    }
}



/// The LocalBuffer trait implements functions for a Buffer that lives solely on the GPU-side.
pub trait LocalBuffer: Buffer {}
