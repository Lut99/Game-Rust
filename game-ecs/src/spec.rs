/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:32:55
 * Last edited:
 *   18 Jul 2022, 18:32:16
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the interfaces to the library: common types, structs, etc.
**/

use std::fmt::Debug;
use std::hash::{Hash, Hasher};


/***** CUSTOM TYPES *****/
/// Defines the type used for all entitites.
#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity(u64);

impl Hash for Entity {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<u64> for Entity {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Entity> for u64 {
    #[inline]
    fn from(value: Entity) -> Self {
        value.0
    }
}



/// Defines the base Component trait.
pub trait Component {}
