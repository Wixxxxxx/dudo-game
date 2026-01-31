use crate::DudoEvent;
use anyhow::Result;
use game_engine::World;

pub struct GameReady;

impl GameReady {
    pub fn run(world: &mut World) -> Result<()> {
        world.emit_now(DudoEvent::RollDice)?;
        Ok(())
    }
}
