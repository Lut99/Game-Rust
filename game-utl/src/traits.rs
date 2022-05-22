/* TRAITS.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 15:12:04
 * Last edited:
 *   22 May 2022, 13:44:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements common traits that are a bit miscellaneous in type.
**/

use std::any::{Any, type_name};


/***** AS ANY *****/
/// Allows any trait requiring this to be downcasted to its original type via Any
pub trait AsAny {
    /// Casts this trait object to a(n immuteable) Any reference.
    fn as_any(&self) -> &dyn Any;

    /// Casts this trait object to a (muteable) Any reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;



    /// Casts this trait object to a(n immuteable) T reference.
    #[inline]
    fn as_t<T: 'static>(&self) -> &T { self.as_any().downcast_ref::<T>().unwrap_or_else(|| panic!("Could cast {} to {}", type_name::<Self>(), type_name::<T>())) }

    /// Casts this trait object to a (muteable) T reference.
    #[inline]
    fn as_t_mut<T: 'static>(&mut self) -> &mut T { self.as_any_mut().downcast_mut::<T>().unwrap_or_else(|| panic!("Could cast {} to {}", type_name::<Self>(), type_name::<T>())) }
}

impl<T: 'static> AsAny for T {
    /// Casts this trait object to an (immuteable) Any reference.
    #[inline]
    fn as_any(&self) -> &dyn Any { self }

    /// Casts this trait object to a (muteable) Any reference.
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
