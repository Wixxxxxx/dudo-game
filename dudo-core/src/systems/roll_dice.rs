use crate::components::dice::{Dice, Hand};
use crate::components::player::Player;
use crate::helpers::emit_current_turn_display;
use anyhow::Result;
use game_engine::World;
use rand::random_range;

pub struct RollDiceSystem;

impl RollDiceSystem {
    pub fn run(world: &mut World) -> Result<()> {
        let players_to_roll = world.query::<(&Player, &Hand)>();

        for entity in players_to_roll {
            if let Ok(hand) = world.component_mut::<Hand>(entity) {
                roll_hand(hand);
            }
        }

        emit_current_turn_display(world)?;

        Ok(())
    }
}

pub fn roll_dice(dice: &mut Dice) {
    dice.face = Some(random_range(1..7));
}

pub fn roll_hand(hand: &mut Hand) {
    hand.dice.iter_mut().for_each(|die| roll_dice(die));
}
