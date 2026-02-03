use crate::ClientEvent;
use crate::DudoEvent;
use crate::resources::GamePhase;
use crate::resources::TurnOrder;
use crate::{Gamertag, components::dice::Hand, resources::GameState};
use anyhow::Result;
use game_engine::{Entity, World};
pub struct ResolveChallengeSystem;

impl ResolveChallengeSystem {
    pub fn run(world: &mut World, challenger: Entity) -> Result<()> {
        let game_state = world.resource::<GameState>()?;

        let current_bid = game_state
            .current_bid
            .ok_or_else(|| anyhow::anyhow!("No current bid"))?;

        let challenged = current_bid.player;

        let total = Self::count_total_dice(world, current_bid.face)?;

        let loser = if total >= current_bid.quantity as usize {
            challenger
        } else {
            challenged
        };

        let loser_name = world.component::<Gamertag>(loser)?.name.clone();
        world.emit_now(ClientEvent::DisplayChallengeLoser { loser: loser_name })?;

        Self::remove_die_from_player(world, loser)?;
        Self::remove_eliminated_players(world)?;
        Self::check_game_over(world)?;

        Ok(())
    }

    fn count_total_dice(world: &World, face: u8) -> Result<usize> {
        let turn_order = world.resource::<TurnOrder>()?;
        let mut count = 0;

        for &player in &turn_order.players {
            let hand = world.component::<Hand>(player)?;
            count += hand.dice.iter().filter(|d| d.face == Some(face)).count();
        }

        Ok(count)
    }

    fn remove_die_from_player(world: &mut World, player: Entity) -> Result<()> {
        let hand = world.component_mut::<Hand>(player)?;
        hand.remove_random();
        Ok(())
    }

    fn remove_eliminated_players(world: &mut World) -> Result<()> {
        // clone player vec
        let players = world.resource::<TurnOrder>()?.players.clone();

        // partition players into survivors and eliminated
        let (survivors, eliminated): (Vec<Entity>, Vec<Entity>) =
            players.into_iter().partition(|&player| {
                world
                    .component::<Hand>(player)
                    .map(|hand| !hand.dice.is_empty())
                    .unwrap_or(false)
            });

        // emit events for eliminated players
        for player in eliminated {
            if let Ok(gamertag) = world.component::<Gamertag>(player) {
                let player_name = gamertag.name.clone();
                world.emit_now(ClientEvent::DisplayPlayerEliminated { player_name })?;
            }
        }

        // update turn order with filtered players
        world.resource_mut::<TurnOrder>()?.players = survivors;

        Ok(())
    }

    fn check_game_over(world: &mut World) -> Result<()> {
        let turn_order = world.resource::<TurnOrder>()?;

        // after removing eliminated players, check if only one remains
        if turn_order.players.len() == 1 {
            // Game over - we have a winner!
            let winner = turn_order.players[0];
            let winner_name = world.component::<Gamertag>(winner)?.name.clone();

            let game_state = world.resource_mut::<GameState>()?;
            game_state.phase = GamePhase::GameOver;
            game_state.winner = Some(winner);

            // emit client event to announce winner
            world.emit_now(ClientEvent::DisplayGameOver { winner_name })?;
        }

        Ok(())
    }
}
