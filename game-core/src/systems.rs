use crate::{
    components::{Hand, Player},
    helpers::roll_hand,
};
use game_engine::World;

pub struct RollDiceSystem;

impl RollDiceSystem {
    pub fn run(world: &mut World) {
        let players_to_roll = world.query::<(Player, Hand)>();

        for entity in players_to_roll {
            if let Ok(hand) = world.get_component_mut::<Hand>(entity) {
                roll_hand(hand);
            }
        }
    }
}
