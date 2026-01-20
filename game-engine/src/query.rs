use crate::{Component, Entity, World};

pub trait Query {
    fn query(world: &World) -> Vec<Entity>;
}

impl<A: Component> Query for (A,) {
    fn query(world: &World) -> Vec<Entity> {
        world
            .query_component::<A>()
            .map(|storage| storage.keys().copied().collect())
            .unwrap_or_default()
    }
}

impl<A: Component, B: Component> Query for (A, B) {
    fn query(world: &World) -> Vec<Entity> {
        let storage_a = match world.query_component::<A>() {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let storage_b = match world.query_component::<B>() {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        storage_a
            .keys()
            .filter(|entity| storage_b.contains_key(entity))
            .copied()
            .collect()
    }
}

impl<A: Component, B: Component, C: Component> Query for (A, B, C) {
    fn query(world: &World) -> Vec<Entity> {
        let storage_a = match world.query_component::<A>() {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let storage_b = match world.query_component::<B>() {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let storage_c = match world.query_component::<C>() {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        storage_a
            .keys()
            .filter(|entity| storage_b.contains_key(entity) && storage_c.contains_key(entity))
            .copied()
            .collect()
    }
}
