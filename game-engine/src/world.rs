use std::{
    any::{Any, TypeId, type_name},
    collections::{HashMap, HashSet},
};

use crate::{
    Entity, EntityBuilder, Event, EventQueue, GameEvent, Query, WorldResourceError,
    error::WorldStorageError,
};

pub trait Component: 'static {}
impl<T: Any + 'static> Component for T {}

pub trait Resource: 'static {}
impl<T: Any + 'static> Resource for T {}

struct ComponentStorage {
    storage: Box<dyn Any>,
}

impl ComponentStorage {
    fn new<T: Component>() -> Self {
        Self {
            storage: Box::new(HashMap::<Entity, T>::new()),
        }
    }

    fn get<T: Component>(&self) -> Option<&HashMap<Entity, T>> {
        self.storage.downcast_ref::<HashMap<Entity, T>>()
    }

    fn get_mut<T: Component>(&mut self) -> Option<&mut HashMap<Entity, T>> {
        self.storage.downcast_mut::<HashMap<Entity, T>>()
    }

    fn insert<T: Component>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), WorldStorageError> {
        let map = self
            .get_mut::<T>()
            .ok_or_else(|| WorldStorageError::ComponentTypeMismatch {
                expected: type_name::<T>(),
            })?;
        map.insert(entity, component);
        Ok(())
    }
}

struct ResourceStorage {
    resource: Box<dyn Any>,
}

impl ResourceStorage {
    fn new<T: Resource>(resource: T) -> Self {
        Self {
            resource: Box::new(resource),
        }
    }

    fn get<T: Resource>(&self) -> Option<&T> {
        self.resource.downcast_ref::<T>()
    }

    fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resource.downcast_mut::<T>()
    }
}

pub struct World {
    component_storages: HashMap<TypeId, ComponentStorage>,
    resources: HashMap<TypeId, ResourceStorage>,
    entities: HashSet<Entity>,
    next_entity_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_storages: HashMap::new(),
            resources: HashMap::new(),
            entities: HashSet::new(),
            next_entity_id: 0,
        }
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        let entity = self.create_entity();
        EntityBuilder::new(self, entity)
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        let entity = Entity::new(id);
        self.entities.insert(entity);
        entity
    }

    pub fn query<Q: Query>(&self) -> Vec<Entity> {
        Q::query(self)
    }

    pub fn insert_component<T: Component>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let component_storage = self
            .component_storages
            .entry(type_id)
            .or_insert_with(|| ComponentStorage::new::<T>());
        component_storage.insert(entity, component)
    }

    pub fn query_component<T: Component>(&self) -> Result<&HashMap<Entity, T>, WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let typename = type_name::<T>();

        let component_storage = self
            .component_storages
            .get(&type_id)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))?;

        component_storage
            .get::<T>()
            .ok_or_else(|| WorldStorageError::ComponentTypeMismatch { expected: typename })
    }

    pub fn query_component_mut<T: Component>(
        &mut self,
    ) -> Result<&mut HashMap<Entity, T>, WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let typename = type_name::<T>();

        let component_storage = self
            .component_storages
            .get_mut(&type_id)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))?;

        component_storage
            .get_mut::<T>()
            .ok_or_else(|| WorldStorageError::ComponentTypeMismatch { expected: typename })
    }

    pub fn component<T: Component>(&self, entity: Entity) -> Result<&T, WorldStorageError> {
        let component_storage = self.query_component::<T>()?;

        component_storage.get(&entity).ok_or_else(|| {
            WorldStorageError::ComponentNotFoundForEntity {
                component: type_name::<T>(),
                entity: entity.id,
            }
        })
    }

    pub fn component_mut<T: Component>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut T, WorldStorageError> {
        let component_storage = self.query_component_mut::<T>()?;
        component_storage.get_mut(&entity).ok_or_else(|| {
            WorldStorageError::ComponentNotFoundForEntity {
                component: type_name::<T>(),
                entity: entity.id,
            }
        })
    }

    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.resources
            .insert(type_id, ResourceStorage::new(resource));
    }

    pub fn resource<T: Resource>(&self) -> Result<&T, WorldResourceError> {
        let type_id = TypeId::of::<T>();

        let resource_storage = self
            .resources
            .get(&type_id)
            .ok_or_else(|| WorldResourceError::ResourceDoesNotExist(type_name::<T>()))?;

        resource_storage
            .get::<T>()
            .ok_or_else(|| WorldResourceError::ResourceTypeMismatch(type_name::<T>()))
    }

    pub fn resource_mut<T: Resource>(&mut self) -> Result<&mut T, WorldResourceError> {
        let type_id = TypeId::of::<T>();

        let resource_storage = self
            .resources
            .get_mut(&type_id)
            .ok_or_else(|| WorldResourceError::ResourceDoesNotExist(type_name::<T>()))?;

        resource_storage
            .get_mut::<T>()
            .ok_or_else(|| WorldResourceError::ResourceTypeMismatch(type_name::<T>()))
    }

    pub fn emit_event<E: GameEvent + 'static>(
        &mut self,
        event: E,
        timestamp: f64,
    ) -> Result<(), WorldResourceError> {
        let queue = self.resource_mut::<EventQueue<E>>()?;
        queue.push(event, timestamp);
        Ok(())
    }

    pub fn pop_event<E: GameEvent + 'static>(
        &mut self,
    ) -> Result<Option<Event<E>>, WorldResourceError> {
        let queue = self.resource_mut::<EventQueue<E>>()?;
        Ok(queue.pop())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct Position {
//         x: f64,
//         y: f64,
//     }

//     struct Velocity {
//         dx: f64,
//         dy: f64,
//     }

//     #[test]
//     fn query_component_storage_immutable() {
//         let entity = Entity(1);
//         let pos = Position { x: 2.0, y: 3.5 };
//         let vel = Velocity { dx: 1.2, dy: 4.3 };

//         let mut world = World::new();

//         world
//             .insert_component(entity, pos)
//             .expect("Failed to insert Position component");
//         world
//             .insert_component(entity, vel)
//             .expect("Failed to insert Velocity component");

//         assert!(
//             world
//                 .query_component::<Position>()
//                 .unwrap()
//                 .contains_key(&entity)
//         );
//         assert!(
//             world
//                 .query_component::<Velocity>()
//                 .unwrap()
//                 .contains_key(&entity)
//         );
//     }

//     #[test]
//     fn query_resource_storage() {}
// }
