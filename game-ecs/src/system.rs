/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:31:26
 * Last edited:
 *   26 Jul 2022, 00:27:09
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base system itself.
**/

use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use log::debug;

use crate::{to_component_list, to_component_list_mut};
use crate::spec::{Component, Entity};
use crate::list::{ComponentList, ComponentListBase};


/***** LIBRARY *****/
/// The Entity Component System (ECS) manages all entiteis that exist in the engine (both renderable as non-renderable).
pub struct Ecs {
    /// Data related to the entities in the ECS.
    /// 
    /// # Layout
    /// - `.0`: The last entity ID used.
    /// - `.1`: The list of currently active entities.
    entities   : RwLock<(u64, HashSet<Entity>)>,
    /// The list of Window components
    components : HashMap<TypeId, (&'static str, RwLock<Box<dyn ComponentListBase>>)>,
}

impl Ecs {
    /// Constructor for the ECS.
    /// 
    /// **Arguments**
    ///  * `initial_capacity`: The initial size of the internal vector (might be used to optimize)
    pub fn new(initial_capacity: usize) -> Self {
        debug!("Initialized Entity Component System v{}", env!("CARGO_PKG_VERSION"));
        Ecs {
            entities   : RwLock::new((0, HashSet::with_capacity(initial_capacity))),
            components : HashMap::with_capacity(16),
        }
    }



    /// Registers a new component type in the ECS.
    /// 
    /// **Generic Types**
    ///  * `T`: The new Component type to register.
    pub fn register<T: 'static + Component>(&mut self) {
        // Insert the new component type if it does not exist yet
        if self.components.contains_key(&ComponentList::<T>::id()) { panic!("A component with ID {:?} already exists", ComponentList::<T>::id()); }
        self.components.insert(ComponentList::<T>::id(), (
            std::any::type_name::<T>(),
            RwLock::new(Box::new(ComponentList::<T>::default())),
        ));

        // Also log the new component registration, but only if compiled with log support
        debug!("Registered new Component type '{:?}'", ComponentList::<T>::id());
    }



    /// Pushes a new entity onto the ECS. Returns the ID of that entity.
    /// 
    /// **Returns**  
    /// The identifier of that entity, as an Entity.
    pub fn add_entity(&mut self) -> Entity {
        // Get a lock first
        let entities: RwLockWriteGuard<(u64, HashSet<_>)> = self.entities.write().expect("Could not get write lock on entity data");

        // Get the next id
        let id: Entity = entities.0.into();
        entities.0 += 1;
        // Insert it into the list of active entities
        entities.1.insert(id);

        // Done
        id
    }

    /// Removes the given entity from the internal list.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to remove.
    /// 
    /// **Returns**  
    /// True if we removed something, or false if that entity did not exist already.
    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        // Remove the entity in question
        {
            let entities: RwLockWriteGuard<(u64, HashSet<_>)> = self.entities.write().expect("Could not get write lock on entity data");
            if !entities.1.remove(&entity) { return false; }
        }

        // Also remove its components from all relevant lists
        for (name, list) in self.components.values() {
            // Get a lock on this list
            let list: RwLockWriteGuard<Box<dyn ComponentListBase>> = list.write().unwrap_or_else(|_| format!("Could not get write lock on component list for {}", name));

            // Remove the entity from it if it exists
            list.delete(entity);
        }

        // Done
        true
    }



    /// Adds the given component to the given entity.  
    /// Overwrites any existing component.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to add.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to add a component for.
    ///  * `data`: The data to set the component value to.
    /// 
    /// **Returns**  
    /// 'true' if the component was added, or 'false' otherwise. It can only fail to be added if the Entity does not exist.
    pub fn add_component<T: 'static + Component>(&mut self, entity: Entity, data: T) -> bool {
        // Get a read lock on the entity list
        let entities: RwLockReadGuard<(_, HashSet<_>)> = self.entities.read().expect("Could not get read lock on entity data");

        // Check if the entity exists
        if !entities.1.contains(&entity) { return false; }

        // Try to get the list to insert it into
        let (name, list) = self.components.get_mut(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        let list: RwLockWriteGuard<Box<dyn ComponentListBase>> = list.write().unwrap_or_else(|_| format!("Could not get write lock on component list for {}", name));

        // Perform the insert
        to_component_list_mut!(list, T).insert(entity, data);

        // Done
        true
    }

    /// Returns the component of the given Entity.
    /// 
    /// The lock returned is actually a lock to the parent ComponentList, so try to keep access to a minimum.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to get.
    /// 
    /// **Returns**  
    /// An immuteable reference to the Component, or else None if the given entity does not exist or does not have such a Component.
    pub fn get_component<T: 'static + Component>(&self, entity: Entity) -> Option<&T> {
        // Check if the entity exists
        if !self.entities.contains(&entity) { return None; }

        // Try to get the list to get from
        let list = self.components.get(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        to_component_list!(list, T).get(entity)
    }

    /// Returns the component of the given Entity.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to get.
    /// 
    /// **Returns**  
    /// A muteable reference to the Component, or else None if the given entity does not exist or does not have such a Component.
    pub fn get_component_mut<T: 'static + Component>(&mut self, entity: Entity) -> Option<&mut T> {
        // Check if the entity exists
        if !self.entities.contains(&entity) { return None; }

        // Try to get the list to get from
        let list = self.components.get_mut(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        to_component_list_mut!(list, T).get_mut(entity)
    }

    /// Returns all entities with the given component type.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to list.
    /// 
    /// **Returns**  
    /// An immuteable reference to the list of components.
    #[inline]
    pub fn list_component<T: 'static + Component>(&self) -> &ComponentList<T> {
        // Simply try to return the correct list
        to_component_list!(
            self.components.get(&ComponentList::<T>::id())
                .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id())),
            T
        )
    }

    /// Returns all entities with the given component type.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to list.
    /// 
    /// **Returns**  
    /// A muteable reference to the list of components.
    #[inline]
    pub fn list_component_mut<T: 'static + Component>(&mut self) -> &mut ComponentList<T> {
        // Try to get the list
        let list = self.components.get_mut(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        // Simply try to return the correct list
        to_component_list_mut!(list, T)
    }

    /// Removes a component for the given entity.
    /// 
    /// **Generic Types**
    ///  * `T`: The Component type we want to remove.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to remove the component of.
    /// 
    /// **Returns**  
    /// Returns the removed component if it existed, or else None.
    #[inline]
    pub fn remove_component<T: 'static + Component>(&mut self, entity: Entity) -> Option<T> {
        // Simply pass it to the correct list
        let list = self.components.get_mut(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        to_component_list_mut!(list, T).remove(entity)
    }
}

impl Default for Ecs {
    /// Default constructor for the ECS.
    fn default() -> Self {
        Ecs::new(2048)
    }
}
