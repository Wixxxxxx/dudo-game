use crate::{Component, Entity, World, WorldStorageError};

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World, entity: Entity) -> Self {
        Self { world, entity }
    }

    pub fn with<T: Component>(self, component: T) -> Result<Self, WorldStorageError> {
        self.world.insert_component(self.entity, component)?;
        Ok(self)
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}
