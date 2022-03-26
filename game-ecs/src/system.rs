/* SYSTEM.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:31:26
 * Last edited:
 *   26 Mar 2022, 10:51:47
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Implements the base system itself.
**/

use std::any::TypeId;
use std::collections::{HashMap, HashSet};

#[cfg(feature = "log")]
use log::debug;

use crate::{to_component_list, to_component_list_mut};
use crate::spec::{Component, Entity};
use crate::list::{ComponentList, ComponentListBase};


/***** LIBRARY *****/
/// The Entity Component System (ECS) manages all entiteis that exist in the engine (both renderable as non-renderable).
#[derive(Debug)]
pub struct Ecs {
    /// The last entity added to the ECS.
    next_id    : u64,
    /// The list of entities in the ECS
    entities   : HashSet<Entity>,
    /// The list of Window components
    components : HashMap<TypeId, Box<dyn ComponentListBase>>,
}

impl Ecs {
    /// Constructor for the ECS.
    /// 
    /// **Arguments**
    ///  * `initial_capacity`: The initial size of the internal vector (might be used to optimize)
    pub fn new(initial_capacity: usize) -> Self {
        Ecs {
            next_id    : 0,
            entities   : HashSet::with_capacity(initial_capacity),
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
        self.components.insert(ComponentList::<T>::id(), Box::new(
            ComponentList::<T>::default()
        ));

        // Also log the new component registration, but only if compiled with log support
        #[cfg(feature = "log")]
        debug!("Registered new Component type '{:?}'", ComponentList::<T>::id());
    }



    /// Pushes a new entity onto the ECS. Returns the ID of that entity.
    /// 
    /// **Returns**  
    /// The identifier of that entity, as an Entity.
    pub fn add_entity(&mut self) -> Entity {
        // Get the next id
        let id: Entity = self.next_id.into();
        self.next_id += 1;

        // Add the ID to the list
        self.entities.insert(id);

        // Done
        id
    }

    /// Removes the given entity from the internal list.
    /// 
    /// **Arguments**
    ///  * `entity`: The Entity to remove.
    /// 
    /// **Returns**  
    /// True if we removed something, or false if that entity did not exist anymore.
    #[inline]
    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        self.entities.remove(&entity)
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
        // Check if the entity exists
        if !self.entities.contains(&entity) { return false; }

        // Try to get the list to insert it into
        let list = self.components.get_mut(&ComponentList::<T>::id())
            .expect(&format!("Unregistered Component type '{:?}'", ComponentList::<T>::id()));
        to_component_list_mut!(list, T).insert(entity, data);

        // Done
        true
    }

    /// Returns the component of the given Entity.
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
