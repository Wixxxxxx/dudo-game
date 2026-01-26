pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub use components::*;
pub use events::DudoEvent;
pub use systems::*;

use anyhow::Result;
use game_engine::{Entity, EventQueue, World};

use crate::components::dice::Hand;
use crate::components::player::{Gamertag, Player};
use crate::resources::{BidHistory, GameState, TurnOrder};

pub fn setup_game(player_names: Vec<String>) -> Result<World> {
    let mut world = World::new();
    world.insert_resource(EventQueue::<DudoEvent>::new());
    world.insert_resource(GameState::new());
    world.insert_resource(BidHistory::new());

    let players = add_players(&mut world, player_names)?;
    world.insert_resource(TurnOrder::new(players));
    Ok(world)
}

fn add_players(world: &mut World, player_names: Vec<String>) -> Result<Vec<Entity>> {
    let mut players = Vec::new();

    for name in player_names.iter() {
        let player = world
            .spawn()
            .with(Player)?
            .with(Gamertag::new(name))?
            .with(Hand::new())?
            .build();

        players.push(player);
    }

    Ok(players)
}
