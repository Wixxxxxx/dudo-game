use game_engine::Entity;

#[derive(Debug, Clone)]
pub struct Bid {
    pub player: Entity,
    pub player_name: String,
    pub quantity: u8,
    pub face: u8,
}

impl Bid {
    pub fn new(player: Entity, player_name: String, quantity: u8, face: u8) -> Self {
        Self {
            player,
            player_name,
            quantity,
            face,
        }
    }
}
