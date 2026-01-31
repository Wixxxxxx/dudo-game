use game_engine::{Entity, GameEvent};
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
    RollDice,
    GameReady,
}
