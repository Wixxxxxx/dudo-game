use crate::bid::Bid;
use game_engine::Entity;
use serde::{Deserialize, Serialize};

// ============================================================================
// Game State
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    RoundStart,
    Bidding,
    Challenge,
    RoundEnd,
    GameOver,
}

pub struct GameState {
    pub round: u32,
    pub current_bid: Option<Bid>,
    pub phase: GamePhase,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            round: 1,
            current_bid: None,
            phase: GamePhase::Bidding,
        }
    }
}

// ============================================================================
// Turn Order
// ============================================================================

pub struct TurnOrder {
    pub players: Vec<Entity>,
    pub current_index: usize,
}

impl TurnOrder {
    pub fn new(players: Vec<Entity>) -> Self {
        Self {
            players,
            current_index: 0,
        }
    }

    pub fn current_player(&self) -> Entity {
        self.players[self.current_index]
    }

    pub fn previous_player(&self) -> Entity {
        let prev_idx = if self.current_index == 0 {
            self.players.len() - 1
        } else {
            self.current_index - 1
        };
        self.players[prev_idx]
    }

    pub fn advance(&mut self) -> Entity {
        self.current_index = (self.current_index + 1) % self.players.len();
        self.current_player()
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}

// ============================================================================
// Bid History
// ============================================================================

pub struct BidHistory {
    pub bids: Vec<Bid>,
}

impl BidHistory {
    pub fn new() -> Self {
        Self { bids: Vec::new() }
    }

    pub fn last_bid(&self) -> Option<&Bid> {
        self.bids.last()
    }

    pub fn clear_round(&mut self) {
        self.bids.clear();
    }
}
