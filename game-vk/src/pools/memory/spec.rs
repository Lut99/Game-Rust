/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   02 Jul 2022, 10:37:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and definitions for the MemoryPools.
**/

use std::fmt::{Debug, Formatter, Result as FResult};
use std::ops::{Add, AddAssign};
use std::rc::Rc;

use ash::vk;
use log::warn;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{MemoryPropertyFlags, MemoryRequirements};
use crate::device::Device;


/***** UNIT TESTS *****/
#[cfg(test)]
mod test {
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

// impl BitAnd for GpuPtr {
//     type Output = Self;

//     #[inline]
//     fn bitand(self, rhs: Self) -> Self::Output {
//         Self(self.0 & rhs.0)
//     }
// }

// impl BitAndAssign for GpuPtr {
//     #[inline]
//     fn bitand_assign(&mut self, rhs: Self) {
//         self.0 &= rhs.0;
//     }
// }

// impl BitAnd<usize> for GpuPtr {
//     type Output = Self;

//     #[inline]
//     fn bitand(self, rhs: usize) -> Self::Output {
//         Self(self.0 & rhs as u64)
//     }
// }

// impl BitAndAssign<usize> for GpuPtr {
//     #[inline]
//     fn bitand_assign(&mut self, rhs: usize) {
//         self.0 &= rhs as u64;
//     }
// }

// impl BitOr for GpuPtr {
//     type Output = Self;

//     #[inline]
//     fn bitor(self, rhs: Self) -> Self::Output {
//         Self(self.0 | rhs.0)
//     }
// }

// impl BitOrAssign for GpuPtr {
//     #[inline]
//     fn bitor_assign(&mut self, rhs: Self) {
//         self.0 |= rhs.0;
//     }
// }

// impl BitOr<usize> for GpuPtr {
//     type Output = Self;

//     #[inline]
//     fn bitor(self, rhs: usize) -> Self::Output {
//         Self(self.0 | rhs as u64)
//     }
// }

// impl BitOrAssign<usize> for GpuPtr {
//     #[inline]
//     fn bitor_assign(&mut self, rhs: usize) {
//         self.0 |= rhs as u64;
//     }
// }

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
