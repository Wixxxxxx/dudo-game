use anyhow::Result;
use game_engine::{Entity, GameEvent, World};
use serde::{Deserialize, Serialize};

impl GameEvent for DudoEvent {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DudoEvent {
    BidMade {
        player: Entity,
        quantity: u8,
        face: u8,
    },
    ChallengeMade {
        challenger: Entity,
    },
    GameReady,
    RollDice,
}

fn emit(world: &mut World, event: DudoEvent) -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let t = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    world.emit_event(event, t)?;
    Ok(())
}
