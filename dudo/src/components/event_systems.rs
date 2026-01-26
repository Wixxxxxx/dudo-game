use crate::DudoEvent;
use crate::systems::place_bid::PlaceBidSystem;
use crate::systems::roll_dice::RollDiceSystem;
use game_engine::World;

pub fn process_events(world: &mut World) -> anyhow::Result<()> {
    while let Some(event) = world.pop_event::<DudoEvent>()?.map(|e| e.event) {
        match event {
            DudoEvent::BidMade {
                player,
                quantity,
                face,
            } => {
                PlaceBidSystem::run(world, player, quantity, face)?;
            }
            DudoEvent::RollDice => {
                RollDiceSystem::run(world)?;
            }
        }
    }
    Ok(())
}
