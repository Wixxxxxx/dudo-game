pub mod components;
pub mod events;
pub mod helpers;
pub mod resources;
pub mod systems;

pub use events::DudoEvent;
use rand::rng;
use rand::seq::SliceRandom;
pub use systems::*;

use anyhow::Result;
use game_engine::{Entity, EventQueue, World};

use crate::components::dice::Hand;
use crate::components::player::{Gamertag, Player};
use crate::events::ClientEvent;
use crate::resources::GamePhase;
use crate::resources::{BidHistory, GameState, TurnOrder};
use crate::systems::event_systems::process_events;

pub struct GameLoop {
    world: World,
}

impl GameLoop {
    pub fn new(player_names: Vec<String>) -> Result<Self> {
        let world = setup_game(player_names)?;
        Ok(Self { world })
    }

    pub fn tick(&mut self) -> Result<()> {
        process_events(&mut self.world)?;
        Ok(())
    }

    pub fn is_game_over(&self) -> Result<bool> {
        Ok(self.world.resource::<GameState>()?.phase == GamePhase::GameOver)
    }

    pub fn can_challenge(&self) -> Result<bool> {
        Ok(!self.world.resource::<BidHistory>()?.bids.is_empty())
    }

    pub fn submit_action(&mut self, action: PlayerAction) -> Result<()> {
        match action {
            PlayerAction::Bid { quantity, face } => {
                let turn_order = self.world.resource::<TurnOrder>()?;
                let player = turn_order.current_player();
                self.world.emit_now(DudoEvent::BidMade {
                    player,
                    quantity,
                    face,
                })?;
            }
            PlayerAction::Challenge => {
                let turn_order = self.world.resource::<TurnOrder>()?;
                let challenger = turn_order.current_player();
                self.world
                    .emit_now(DudoEvent::ChallengeMade { challenger })?;
            }
        }
        Ok(())
    }
}

pub enum PlayerAction {
    Bid { quantity: u8, face: u8 },
    Challenge,
}

pub fn setup_game(player_names: Vec<String>) -> Result<World> {
    let mut world = World::new();
    world.insert_resource(EventQueue::<DudoEvent>::new());
    world.insert_resource(EventQueue::<ClientEvent>::new());
    world.insert_resource(GameState::new());
    world.insert_resource(BidHistory::new());

    let players = add_players(&mut world, player_names)?;
    world.insert_resource(TurnOrder::new(players));
    world.emit_now(DudoEvent::RollDice)?;
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
        players.shuffle(&mut rng());
    }

    Ok(players)
}
