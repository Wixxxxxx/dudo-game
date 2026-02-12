use anyhow::Result;
use game_engine::{Entity, World};

use crate::{
    components::bid::Bid,
    events::ClientEvent,
    resources::{BidHistory, GameState, TurnOrder},
};

use crate::helpers::emit_current_turn_display;

pub struct PlaceBidSystem;

impl PlaceBidSystem {
    pub fn run(world: &mut World, player: Entity, quantity: u8, face: u8) -> Result<()> {
        // add new bid to history
        let new_bid = Bid::new(player, quantity, face);
        let game_state = world.resource_mut::<GameState>()?;
        game_state.current_bid = Some(new_bid);
        let bid_history = world.resource_mut::<BidHistory>()?;
        bid_history.add_bid(new_bid);

        // advance turn order
        let turn_order = world.resource_mut::<TurnOrder>()?;
        turn_order.advance();

        emit_current_turn_display(world)?;
        world.emit_now(ClientEvent::DisplayActions)?;
        Ok(())
    }
}
