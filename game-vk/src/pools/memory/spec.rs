/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:10:55
 * Last edited:
 *   29 Jun 2022, 19:20:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the interfaces and definitions for the MemoryPools.
**/

use std::fmt::{Debug, Formatter, Result as FResult};
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::rc::Rc;

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{MemoryPropertyFlags, MemoryRequirements};
use crate::device::Device;


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
    /// Returns a pointer with the value 0.
    pub const ZERO: Self = Self(0);

    /// Returns a NULL pointer.
    pub const NULL: Self = Self(!0);



    /// Constructs a new GpuPtr with the appropriate values set
    /// 
    /// # Arguments
    /// - `mem_type`: The index of the memory type.
    /// - `pool`: The index of the memory pool.
    /// - `



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
}

impl Debug for GpuPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Split the value into the block number & the actual pointer
        let blk: u64 = (self.0 >> 48) & 0xFFFF;
        let ptr: u64 =  self.0        & 0xFFFFFFFFFFFF;
        if blk > 0 { write!(f, "B{}-", blk)?; }
        write!(f, "{:#X}", ptr)
    }
}

impl Add for GpuPtr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for GpuPtr {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add<usize> for GpuPtr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl AddAssign<usize> for GpuPtr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs as u64
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
        Self(value as u64)
    }
}

impl From<GpuPtr> for usize {
    #[inline]
    fn from(value: GpuPtr) -> Self {
        value.0 as Self
    }
}

impl From<vk::DeviceSize> for GpuPtr {
    #[inline]
    fn from(value: vk::DeviceSize) -> Self {
        Self(value as u64)
    }
}

impl From<GpuPtr> for vk::DeviceSize {
    #[inline]
    fn from(value: GpuPtr) -> Self {
        value.0 as Self
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
