use std::{
    any::{Any, TypeId, type_name},
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{EntityBuilder, Query, error::WorldStorageError};

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq)]
pub struct Entity(u64);

pub trait Component: 'static {}
impl<T: Any + 'static> Component for T {}

pub trait Resource: 'static {}
impl<T: Any + 'static> Resource for T {}

pub struct World {
    storages: HashMap<TypeId, Box<dyn Any>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
    entities: HashSet<Entity>,
    next_entity_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
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
        let entity = Entity(id);
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
        let typename = type_name::<T>();

        let storage_any = self
            .storages
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));

        let storage = storage_any
            .downcast_mut::<HashMap<Entity, T>>()
            .ok_or(WorldStorageError::ComponentStorageTypeMismatch(typename))?;

        storage.insert(entity, component);
        Ok(())
    }

    pub fn query_component<T: Component>(&self) -> Result<&HashMap<Entity, T>, WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let typename = type_name::<T>();

        let storage_any = self
            .storages
            .get(&type_id)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))?;

        let storage = storage_any
            .downcast_ref::<HashMap<Entity, T>>()
            .ok_or(WorldStorageError::ComponentStorageTypeMismatch(typename))?;

        Ok(storage)
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Result<&T, WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let typename = type_name::<T>();

        let storage_any = self
            .storages
            .get(&type_id)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))?;

        let storage = storage_any
            .downcast_ref::<HashMap<Entity, T>>()
            .ok_or(WorldStorageError::ComponentStorageTypeMismatch(typename))?;

        storage
            .get(&entity)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))
    }

    pub fn get_component_mut<T: Component>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut T, WorldStorageError> {
        let type_id = TypeId::of::<T>();
        let typename = type_name::<T>();

        let storage_any = self
            .storages
            .get_mut(&type_id)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))?;

        let storage = storage_any
            .downcast_mut::<HashMap<Entity, T>>()
            .ok_or(WorldStorageError::ComponentStorageTypeMismatch(typename))?;

        storage
            .get_mut(&entity)
            .ok_or(WorldStorageError::ComponentStorageDoesNotExist(typename))
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> Result<(), WorldStorageError> {
        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(resource));
        Ok(())
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get(&type_id)
            .and_then(|resource| resource.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get_mut(&type_id)
            .and_then(|resource| resource.downcast_ref::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Position {
        x: f64,
        y: f64,
    }

    struct Velocity {
        dx: f64,
        dy: f64,
    }

    #[test]
    fn query_component_storage_immutable() {
        let entity = Entity(1);
        let pos = Position { x: 2.0, y: 3.5 };
        let vel = Velocity { dx: 1.2, dy: 4.3 };

        let mut world = World::new();

        world
            .insert_component(entity, pos)
            .expect("Failed to insert Position component");
        world
            .insert_component(entity, vel)
            .expect("Failed to insert Velocity component");

        assert!(
            world
                .query_component::<Position>()
                .unwrap()
                .contains_key(&entity)
        );
        assert!(
            world
                .query_component::<Velocity>()
                .unwrap()
                .contains_key(&entity)
        );
    }

    #[test]
    fn query_resource_storage() {}
}
