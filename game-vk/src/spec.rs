/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   29 Apr 2022, 18:16:49
 * Last edited:
 *   07 Jul 2022, 18:14:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines traits and other public interfaces in the Vulkan crate.
**/

use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr, BitOrAssign};


/***** LIBRARY *****/
/// The Flags-trait is used to define a common interface for all the auxillary Vulkan flag structs.
pub trait Flags: Copy + Clone + Debug {
    /// Determines the type of the internal value where the flags are stored.
    type RawType: BitAnd<Output = Self::RawType> + BitOr<Output = Self::RawType> + Copy + Clone + Debug + PartialEq;


    /// Constructor for the Flags object that creates it without any flags initialized.
    #[inline]
    fn empty() -> Self { Self::from_raw(0) }

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
