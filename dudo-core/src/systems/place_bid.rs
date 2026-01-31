use anyhow::Result;
use game_engine::{Entity, World};

use crate::{
    components::bid::Bid,
    resources::{BidHistory, GameState},
};

pub struct PlaceBidSystem;

impl PlaceBidSystem {
    pub fn run(world: &mut World, player: Entity, quantity: u8, face: u8) -> Result<()> {
        let new_bid = Bid::new(player, quantity, face);
        let game_state = world.resource_mut::<GameState>()?;
        game_state.current_bid = Some(new_bid);
        let bid_history = world.resource_mut::<BidHistory>()?;
        bid_history.add_bid(new_bid);
        Ok(())
    }
}
