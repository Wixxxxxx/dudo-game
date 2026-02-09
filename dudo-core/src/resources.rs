use crate::components::bid::Bid;
use game_engine::Entity;
use rand::rng;
use rand::seq::SliceRandom;
use std::fmt;

// ============================================================================
// Game State
// ============================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum GamePhase {
    InPlay,
    GameOver,
}

pub struct GameState {
    pub round: u32,
    pub phase: GamePhase,
    pub current_bid: Option<Bid>,
    pub winner: Option<Entity>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            round: 1,
            phase: GamePhase::InPlay,
            current_bid: None,
            winner: None,
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

    pub fn shuffle(&mut self) {
        self.players.shuffle(&mut rng());
        self.current_index = 0;
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

    pub fn add_bid(&mut self, new_bid: Bid) {
        self.bids.push(new_bid);
    }

    pub fn is_empty(&self) -> bool {
        self.bids.is_empty()
    }
}

impl fmt::Display for BidHistory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bids.is_empty() {
            return write!(f, "No bids yet");
        }

        writeln!(f, "Bid History:")?;
        for (i, bid) in self.bids.iter().enumerate() {
            writeln!(
                f,
                "  {}. {}: {} × {}",
                i + 1,
                bid.player_name,
                bid.quantity,
                bid.face
            )?;
        }
        Ok(())
    }
}
