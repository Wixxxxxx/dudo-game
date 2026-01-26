use anyhow::Result;
use game_engine::{Entity, World};

use crate::{components::bid::Bid, resources::GameState};

pub struct PlaceBidSystem;

impl PlaceBidSystem {
    pub fn run(world: &mut World, player: Entity, quantity: u8, face: u8) -> Result<()> {
        let game_state = world.resource_mut::<GameState>()?;
        game_state.current_bid = Some(Bid::new(player, quantity, face));
        Ok(())
    }
}
