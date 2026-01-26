use crate::dice::Dice;
use crate::{components::dice::Hand, resources::GameState};
use game_engine::{Entity, World};

use crate::resources::TurnOrder;
use anyhow::Result;
use rand::random_range;

fn resolve_challenge(world: &World, challenger: Entity, challenged: Entity) -> Result<Entity> {
    let game_state = world.resource::<GameState>()?;

    let current_bid = game_state
        .current_bid
        .ok_or_else(|| anyhow::anyhow!("No current bid set in GameState"))?;

    let total = count_total_dice(world, current_bid.face)?;

    Ok(if total >= current_bid.quantity as usize {
        challenger
    } else {
        challenged
    })
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

fn remove_die_from_player(world: &mut World, player: Entity) -> Result<Option<Dice>> {
    let hand = world.component_mut::<Hand>(player)?;
    let idx = random_range((0..hand.dice.iter().len()));
    Ok(Some(hand.dice.swap_remove(idx)))
}
