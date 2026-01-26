pub struct Player;

pub struct Gamertag {
    pub name: String,
}

impl Gamertag {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
