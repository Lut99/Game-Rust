/* LIST.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:32:36
 * Last edited:
 *   18 Jul 2022, 18:33:54
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the ComponentList container, that is basically an array but
 *   with entity-to-index mappings.
**/

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::spec::{Component, Entity};


/***** LIBRARY *****/
/// Defines the ComponentListBase trait that functions as the typeless base of the ComponentList
pub trait ComponentListBase {
    /// Allows the ComponentListBase to be downcasted.
    fn as_any(&self) -> &dyn Any;

    /// Allows the ComponentListBase to be (muteable) downcasted.
    fn as_any_mut(&mut self) -> &mut dyn Any;



    /// Returns the identifier for this specific generic type
    fn id(&self) -> TypeId;



    /// Get the index from an entity.  
    /// This is useful to iterate more easily through the list.
    /// 
    /// **Arguments**
    ///  * `entity`: The entity to get the index of.
    /// 
    /// **Returns**  
    /// The index of the given entity in the list if the entity has a component in this list, or None otherwise.
    fn get_index(&self, entity: Entity) -> Option<usize>;

    /// Get the entity from an index.  
    /// This is useful when iterating through the list.
    /// 
    /// **Arguments**
    ///  * `index`: The index to match an entity with.
    /// 
    /// **Returns**  
    /// The entity that resides at the given index if the index is in range, or None otherwise.
    fn get_entity(&self, index: usize) -> Option<Entity>;
}





/***** HELPER STRUCTS *****/
/// Defines an iterable vector (good for caching) which is indexable by entity ID.
/// 
/// **Generic Types**
///  * `T`: The Component for this list.
#[derive(Clone)]
pub struct ComponentList<T>
where
    T: Component,
{
    /// Maps entity names to indices
    e_to_i : HashMap<Entity, usize>,
    /// Maps indices to entity names
    i_to_e : HashMap<usize, Entity>,
    /// Stores the components
    data   : Vec<T>,
}

impl<T: Component> ComponentList<T> {
    /// Constructor for the ComponentList.
    /// 
    /// **Arguments**
    ///  * `initial_capacity`: The initial capacity of the list. Used for optimization purposes.
    pub(crate) fn new(initial_capacity: usize) -> Self {
        ComponentList {
            e_to_i : HashMap::with_capacity(initial_capacity),
            i_to_e : HashMap::with_capacity(initial_capacity),
            data   : Vec::with_capacity(initial_capacity),
        }
    }

    /// Returns the identifier for this ComponentList type.
    #[inline]
    pub(crate) fn id() -> TypeId
    where
        T: 'static
    {
        TypeId::of::<T>()
    }



    /// Inserts a new set of component data.  
    /// Overwrites the value for the entity if it already exists.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to add the data for.
    ///  * `data`: The data to add for the Entity.
    #[inline]
    pub(crate) fn insert(&mut self, entity: Entity, data: T) {
        // Do two different things depending on if it exists or not
        match self.e_to_i.get(&entity) {
            Some(index) => {
                // Overwrite the value
                self.data[*index] = data;
            },
            None => {
                // Add the mapping
                let index = self.data.len();
                self.e_to_i.insert(entity, index);
                self.i_to_e.insert(index, entity);

                // Add the data itself
                if self.data.len() >= self.data.capacity() { self.data.reserve(self.data.capacity()); }
                self.data.push(data);
            }
        }
    }

    /// Gets the component for the given entity (as immuteable).
    /// 
    /// **Arguments**
    ///  * `entity`: The entity to get the component of.
    /// 
    /// **Returns**  
    /// An immuteable reference to the component if it exists, or None otherwise.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        match self.e_to_i.get(&entity) {
            Some(index) => Some(&self.data[*index]),
            None => None,
        }
    }

    /// Gets the component for the given entity (as muteable)
    /// 
    /// **Arguments**
    ///  * `entity`: The entity to get the component of.
    /// 
    /// **Returns**  
    /// A muteable reference to the component if it exists, or None otherwise.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        match self.e_to_i.get(&entity) {
            Some(index) => Some(&mut self.data[*index]),
            None => None,
        }
    }

    /// Removes the component for an entity.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to remove the data of.
    /// 
    /// **Returns**  
    /// Returns the removed value if it existed, or else None.
    #[inline]
    pub(crate) fn remove(&mut self, entity: Entity) -> Option<T> {
        // Check if the entity exists
        match self.e_to_i.get(&entity).map(|index| *index) {
            Some(index) => {
                // Remove from the mappings
                self.e_to_i.remove(&entity);
                self.i_to_e.remove(&index);

                // Next, remove the data itself
                let to_return = self.data.swap_remove(index);

                // If there is a last value that was swapped, update the value of the last element to point to this element instead
                if self.data.len() > 0 {
                    let last_entity = self.i_to_e.remove(&self.data.len()).expect("Last element in list is not mapped in index-to-entity map");
                    self.i_to_e.insert(index, last_entity);
                    *self.e_to_i.get_mut(&last_entity).expect("Last element in list is not mapped in entity-to-index map") = index;
                }

                // Done
                Some(to_return)
            },
            None => None,
        }
    }
}

impl<T> ComponentListBase for ComponentList<T>
where
    T: 'static + Component
{
    /// Allows the ComponentListBase to be downcasted.
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Allows the ComponentListBase to be (muteable) downcasted.
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }



    /// Returns the identifier for this specific generic type
    #[inline]
    fn id(&self) -> TypeId {
        ComponentList::<T>::id()
    }



    /// Get the index from an entity.  
    /// This is useful to iterate more easily through the list.
    /// 
    /// **Arguments**
    ///  * `entity`: The entity to get the index of.
    /// 
    /// **Returns**  
    /// The index of the given entity in the list if the entity has a component in this list, or None otherwise.
    #[inline]
    fn get_index(&self, entity: Entity) -> Option<usize> {
        self.e_to_i.get(&entity).map(|index| *index)
    }

    /// Get the entity from an index.  
    /// This is useful when iterating through the list.
    /// 
    /// **Arguments**
    ///  * `index`: The index to match an entity with.
    /// 
    /// **Returns**  
    /// The entity that resides at the given index if the index is in range, or None otherwise.
    #[inline]
    fn get_entity(&self, index: usize) -> Option<Entity> {
        self.i_to_e.get(&index).map(|entity| *entity)
    }
}

impl<T: Component> Default for ComponentList<T> {
    /// Default constructor for the ComponentList
    fn default() -> Self {
        ComponentList::new(2048)
    }
}

impl<T: Component> Index<usize> for ComponentList<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Component> IndexMut<usize> for ComponentList<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Component> IntoIterator for ComponentList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
}

impl<'a, T: Component> IntoIterator for &'a ComponentList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.data.iter() }
}

impl <'a, T: Component> IntoIterator for &'a mut ComponentList<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
}
