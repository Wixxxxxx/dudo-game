use crate::{components::dice::Hand, resources::GameState};
use game_engine::{Entity, World};

use crate::resources::TurnOrder;
use anyhow::Result;
pub struct ResolveChallengeSystem;

impl ResolveChallengeSystem {
    pub fn run(world: &mut World, challenger: Entity) -> Result<Entity> {
        let game_state = world.resource::<GameState>()?;

        let current_bid = game_state
            .current_bid
            .ok_or_else(|| anyhow::anyhow!("No current bid"))?;

        let challenged = current_bid.player;

        let total = Self::count_total_dice(world, current_bid.face)?;

        let loser = if total >= current_bid.quantity as usize {
            challenger
        } else {
            challenged
        };

        Self::remove_die_from_player(world, loser)?;

        Ok(loser)
    }

    fn count_total_dice(world: &World, face: u8) -> Result<usize> {
        let turn_order = world.resource::<TurnOrder>()?;
        let mut count = 0;

        for &player in &turn_order.players {
            let hand = world.component::<Hand>(player)?;
            count += hand.dice.iter().filter(|d| d.face == Some(face)).count();
        }

        Ok(count)
    }

    fn remove_die_from_player(world: &mut World, player: Entity) -> Result<()> {
        let hand = world.component_mut::<Hand>(player)?;
        hand.remove_random();
        Ok(())
    }
}
