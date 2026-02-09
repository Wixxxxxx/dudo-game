use crate::events::{ClientEvent, DudoEvent};
use crate::systems::challenge::ResolveChallengeSystem;
use crate::systems::place_bid::PlaceBidSystem;
use crate::systems::roll_dice::RollDiceSystem;
use anyhow::Result;
use game_engine::{EventQueue, World};

pub fn process_events(world: &mut World) -> Result<()> {
    while let Some(event) = world.pop_event::<DudoEvent>()?.map(|e| e.event) {
        match event {
            DudoEvent::BidMade {
                player,
                player_name,
                quantity,
                face,
            } => {
                PlaceBidSystem::run(world, player, player_name, quantity, face)?;
            }
            DudoEvent::RollDice => {
                RollDiceSystem::run(world)?;
            }
            DudoEvent::ChallengeMade { challenger } => {
                ResolveChallengeSystem::run(world, challenger)?;
            }
        }
    }
    Ok(())
}

pub fn process_client_events(world: &mut World) -> Result<Vec<ClientEvent>> {
    let queue = world.resource_mut::<EventQueue<ClientEvent>>()?;
    Ok(queue.drain().into_iter().map(|e| e.event).collect())
}
