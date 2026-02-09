use game_engine::{Entity, GameEvent};
use serde::{Deserialize, Serialize};

impl GameEvent for DudoEvent {}
impl GameEvent for ClientEvent {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DudoEvent {
    BidMade {
        player: Entity,
        player_name: String,
        quantity: u8,
        face: u8,
    },
    ChallengeMade {
        challenger: Entity,
    },
    RollDice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientEvent {
    DisplayCurrentTurn {
        player_name: String,
    },
    DisplayChallenge {
        challenger_name: String,
    },
    DisplayChallengeLoser {
        loser: String,
    },
    DisplayBid {
        player_name: String,
        quantity: u8,
        face: u8,
    },
    DisplayGameOver {
        winner_name: String,
    },
    DisplayPlayerEliminated {
        player_name: String,
    },
}
