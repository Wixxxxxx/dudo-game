use crate::components::player::Gamertag;
use crate::events::ClientEvent;
use crate::resources::TurnOrder;
use anyhow::Result;
use game_engine::World;

/// Emits a ClientEvent to display the current player's turn
pub fn emit_current_turn_display(world: &mut World) -> Result<()> {
    let turn_order = world.resource::<TurnOrder>()?;
    let gamertag = world.component::<Gamertag>(turn_order.current_player())?;
    world.emit_now(ClientEvent::DisplayCurrentTurn {
        player_name: gamertag.name.clone(),
    })?;
    Ok(())
}

/// Gets the current player's name without emitting an event
pub fn get_current_player_name(world: &World) -> Result<String> {
    let turn_order = world.resource::<TurnOrder>()?;
    let gamertag = world.component::<Gamertag>(turn_order.current_player())?;
    Ok(gamertag.name.clone())
}
