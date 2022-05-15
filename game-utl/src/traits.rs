/* TRAITS.rs
 *   by Lut99
 *
 * Created:
 *   03 Apr 2022, 15:12:04
 * Last edited:
 *   29 Apr 2022, 18:12:20
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
    /// Casts this RenderTarget to an (immuteable) Any reference.
    fn as_any(&self) -> &dyn Any;

    /// Casts this RenderTarget to a (muteable) Any reference.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AsAny for T {
    /// Casts this RenderTarget to an (immuteable) Any reference.
    #[inline]
    fn as_any(&self) -> &dyn Any { self }

    /// Casts this RenderTarget to a (muteable) Any reference.
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
