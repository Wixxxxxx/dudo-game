use pinocchio::pubkey::Pubkey;

pub struct Game {
    pub host: Pubkey,
    pub players: [Pubkey; 5],
    pub max_players: u8,
    pub min_players: u8,
    pub buyin: u64,
    pub dice: [[u8; 5]; 5],
    pub game_status: GameStatus,
    pub mint: Pubkey,
    pub pot: u64,
}

impl Game {}

pub enum GameStatus {
    Lobby,
    RoundStart,
    RoundOngoing,
    RoundEnd,
    GameEnd,
}
