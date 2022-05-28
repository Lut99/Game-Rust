/* TRAITS.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 15:12:04
 * Last edited:
 *   28 May 2022, 16:50:20
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements common traits that are a bit miscellaneous in type.
**/

use std::any::Any;


/***** AS ANY *****/
/// Allows any trait requiring this to be downcasted to its original type via Any
pub trait AsAny {
    /// Casts this trait object to a(n immuteable) Any reference.
    fn as_any(&self) -> &dyn Any;

    /// Casts this trait object to a (muteable) Any reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    /// Casts this trait object to an (immuteable) Any reference.
    #[inline]
    fn as_any(&self) -> &dyn Any { self }

    /// Casts this trait object to a (muteable) Any reference.
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
