use game_engine::Entity;

#[derive(Debug, Clone, Copy)]
pub struct Bid {
    pub player: Entity,
    pub quantity: u8,
    pub face: u8,
}

impl Bid {
    pub fn new(player: Entity, quantity: u8, face: u8) -> Self {
        Self {
            player,
            quantity,
            face,
        }
    }
}
