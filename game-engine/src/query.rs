use crate::{Component, Entity, World};

pub trait Query {
    fn query(world: &World) -> Vec<Entity>;
}

impl<'a, A: Component> Query for &'a A {
    fn query(world: &World) -> Vec<Entity> {
        world
            .query_component::<A>()
            .map(|storage| storage.keys().copied().collect())
            .unwrap_or_default()
    }
}

macro_rules! impl_query {
    // recurse: emit an impl for the full list, then peel one off
    ($head:ident, $($tail:ident),+) => {
        impl_query!(@impl $head, $($tail),+);
        impl_query!($($tail),+);
    };
    // base case: single component — covered by the blanket &A impl above
    ($last:ident) => {};
    (@impl $first:ident, $($rest:ident),+) => {
        impl<'a, $first: Component, $($rest: Component),+> Query for (&'a $first, $(&'a $rest),+) {
            fn query(world: &World) -> Vec<Entity> {
                #[allow(non_snake_case)]
                let $first = match world.query_component::<$first>() {
                    Ok(s) => s,
                    Err(_) => return vec![],
                };
                $(
                    #[allow(non_snake_case)]
                    let $rest = match world.query_component::<$rest>() {
                        Ok(s) => s,
                        Err(_) => return vec![],
                    };
                )+

                $first
                    .keys()
                    .filter(|entity| $($rest.contains_key(entity))&&+)
                    .copied()
                    .collect()
            }
        }
    };
}

impl_query!(A, B, C, D, E, F, G, H);

#[cfg(test)]
mod tests {
    use super::*;

    struct Position;
    struct Velocity;
    struct Health;

    #[test]
    fn query_single_component() {
        let mut world = World::new();

        let a = world.create_entity();
        let b = world.create_entity();
        world.insert_component(a, Position).unwrap();
        world.insert_component(b, Position).unwrap();
        world.insert_component(b, Velocity).unwrap();

        let mut entities = world.query::<&Position>();
        entities.sort_by_key(|entity| entity.id);
        assert_eq!(entities, vec![a, b]);

        assert_eq!(world.query::<&Velocity>(), vec![b]);
    }

    #[test]
    fn query_multiple_components_intersects() {
        let mut world = World::new();

        let a = world.create_entity();
        let b = world.create_entity();
        let c = world.create_entity();
        world.insert_component(a, Position).unwrap();
        world.insert_component(b, Position).unwrap();
        world.insert_component(b, Velocity).unwrap();
        world.insert_component(c, Velocity).unwrap();
        world.insert_component(b, Health).unwrap();

        assert_eq!(world.query::<(&Position, &Velocity)>(), vec![b]);
        assert_eq!(world.query::<(&Position, &Velocity, &Health)>(), vec![b]);
    }

    #[test]
    fn query_missing_storage_returns_empty() {
        let mut world = World::new();

        let a = world.create_entity();
        world.insert_component(a, Position).unwrap();

        assert!(world.query::<&Health>().is_empty());
        assert!(world.query::<(&Position, &Health)>().is_empty());
    }
}
