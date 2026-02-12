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
use crate::systems::event_systems::{process_client_events, process_events};

pub struct GameLoop {
    world: World,
}

impl GameLoop {
    pub fn new(player_names: Vec<String>) -> Result<Self> {
        let world = setup_game(player_names)?;
        Ok(Self { world })
    }

    pub fn tick(&mut self) -> Result<Vec<ClientEvent>> {
        process_events(&mut self.world)?;
        process_client_events(&mut self.world)
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
                let player = self.get_current_player()?;
                let player_name = self.world.component::<Gamertag>(player)?.name.clone();
                self.world.emit_now(DudoEvent::BidMade {
                    player,
                    player_name,
                    quantity,
                    face,
                })?;
            }
            PlayerAction::Challenge => {
                let challenger = self.get_current_player()?;
                self.world
                    .emit_now(DudoEvent::ChallengeMade { challenger })?;
            }
        }
        Ok(())
    }

    pub fn get_current_player(&mut self) -> Result<Entity> {
        let turn_order = self.world.resource::<TurnOrder>()?;
        Ok(turn_order.current_player())
    }

    pub fn get_current_player_hand(&mut self) -> Result<&Hand> {
        let player = self.get_current_player()?;
        let hand = self.world.component::<Hand>(player)?;
        Ok(hand)
    }

    pub fn get_current_bid(&self) -> Result<Option<(u8, u8)>> {
        let game_state = self.world.resource::<GameState>()?;
        Ok(game_state
            .current_bid
            .as_ref()
            .map(|bid| (bid.quantity, bid.face)))
    }

    pub fn get_bid_history(&self) -> Result<String> {
        let bid_history = self.world.resource::<BidHistory>()?;
        Ok(format!("{}", bid_history))
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
