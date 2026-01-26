use crate::components::bid::{Bid, BidEntry, BidHistory};
use std::time::{SystemTime, UNIX_EPOCH};

// use game_core::systems::RollDiceSystem;
use game_engine::{Entity, World};

use anyhow::Result;
use colored::Colorize;
use inquire::{Select, Text};

use dudo::{DudoEvent, event_systems::process_events, resources::GamePhase, setup_game};
use rand::random_range;

fn main() -> Result<()> {
    loop {
        show_title();
        if !main_menu()? {
            break;
        }
    }

    Ok(())
}

fn main_menu() -> Result<bool> {
    let menu = vec!["Start", "Rules", "Quit"];
    let menu_choice = Select::new("Main Menu", menu).prompt()?;

    match menu_choice {
        "Start" => {
            game_loop()?;
            Ok(true)
        }
        "Rules" => {
            show_rules()?;
            Ok(true)
        }
        "Quit" => {
            quit()?;
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn game_loop() -> Result<()> {
    let players = get_player_names()?;
    let mut world = setup_game(players)?;

    loop {
        let phase = world.resource::<GameState>()?.phase;
        if phase == GamePhase::RoundStart {
            println!("\n{}", "🎲 Rolling dice...".bright_yellow());
            emit(world, DudoEvent::RollDice)?;
        }

        process_events(&mut world);
    }

    world.insert_resource(BidHistory { bids: Vec::new() })?;
    let starting_idx = random_range(0..players.len());
    let starting_name = world.get_component::<Gamertag>(players[starting_idx])?;
    println!(
        "{}",
        format!("🎯 {} starts!", starting_name.name).bright_green()
    );

    // game_loop(&mut world, &mut players, starting_idx)?;
    Ok(())
}

// fn game_loop(world: &mut World, players: &mut Vec<Entity>, starting_idx: usize) -> Result<()> {
//     let mut current_idx = starting_idx;
//     let mut current_bid: Option<Bid> = None;

//     loop {
//         let player = players[current_idx];
//         let gamertag = world.get_component::<Gamertag>(player)?;

//         println!(
//             "\n{}",
//             format!("─── {}'s Turn ───", gamertag.name)
//                 .bright_green()
//                 .bold()
//         );
//         if let Some(bid) = &current_bid {
//             println!("Current bid: {} dice showing {}", bid.quantity, bid.face);
//         }

//         let action = get_player_action(current_bid.is_some())?;

//         match action {
//             PlayerAction::InspectDice => {
//                 println!("{}", world.get_component::<Hand>(player)?);
//                 continue;
//             }
//             PlayerAction::MakeBid(bid) => {
//                 if let Some(prev_bid) = &current_bid {
//                     if !is_higher_bid(&bid, prev_bid) {
//                         println!("{}", "Bid must be higher!".red());
//                         continue;
//                     }
//                 }
//                 current_bid = Some(bid);
//                 println!(
//                     "{}",
//                     format!("✅ {} bids {} × {}", gamertag.name, bid.quantity, bid.face).green()
//                 );
//             }
//             PlayerAction::CallBluff => {
//                 if current_bid.is_none() {
//                     println!("{}", "No bid to challenge!".red());
//                     continue;
//                 }
//                 resolve_challenge(world, players, &current_bid.unwrap())?;
//                 break;
//             }
//         }

//         current_idx = (current_idx + 1) % players.len();
//     }

//     Ok(())
// }

fn get_player_action(has_bid: bool) -> Result<PlayerAction> {
    let actions = if has_bid {
        vec!["Inspect Dice", "Raise Bid", "Call Bluff"]
    } else {
        vec!["Inspect Dice", "Make First Bid"]
    };

    let choice = Select::new("Choose action:", actions).prompt()?;

    match choice {
        "Inspect Dice" => Ok(PlayerAction::InspectDice),
        "Raise Bid" | "Make First Bid" => {
            let bid = get_bid_from_player()?;
            Ok(PlayerAction::MakeBid(bid))
        }
        "Call Bluff" => Ok(PlayerAction::CallBluff),
        _ => Ok(PlayerAction::InspectDice),
    }
}

fn get_bid_from_player() -> Result<Bid> {
    let quantity = Text::new("How many dice?").prompt()?.parse::<u8>()?;

    let face = Text::new("What face value (1-6)?")
        .prompt()?
        .parse::<u8>()?;

    if face < 1 || face > 6 {
        println!("{}", "Face must be 1-6!".red());
        return get_bid_from_player();
    }

    Ok(Bid { quantity, face })
}

fn is_higher_bid(new_bid: &Bid, prev_bid: &Bid) -> bool {
    if new_bid.quantity > prev_bid.quantity {
        return true;
    }
    if new_bid.quantity == prev_bid.quantity && new_bid.face > prev_bid.face {
        return true;
    }
    false
}

fn resolve_challenge(world: &World, players: &[Entity], bid: &Bid) -> Result<()> {
    println!(
        "\n{}",
        "⚔️  CHALLENGE! Revealing all dice...".bright_red().bold()
    );

    let mut total_count = 0;

    for &player in players {
        let gamertag = world.get_component::<Gamertag>(player)?;
        let hand = world.get_component::<Hand>(player)?;

        let count = hand
            .dice
            .iter()
            .filter(|d| d.face == Some(bid.face))
            .count();

        total_count += count;
        println!(
            "{}: {} ({} × {})",
            gamertag.name.yellow(),
            hand,
            count,
            bid.face
        );
    }

    println!(
        "\n{}",
        format!("Total: {} dice showing {}", total_count, bid.face)
            .bright_cyan()
            .bold()
    );
    println!("Bid was: {} dice showing {}", bid.quantity, bid.face);

    if total_count >= bid.quantity as usize {
        println!("{}", "✅ Bid was correct! Challenger loses.".bright_green());
        // TODO: Remove die from challenger
    } else {
        println!("{}", "❌ Bid was too high! Bidder loses.".bright_red());
        // TODO: Remove die from previous bidder
    }

    println!("\n{}", "Press Enter to continue...".dimmed());
    Text::new("").prompt()?;

    Ok(())
}

fn add_players(world: &mut World, players: &mut Vec<Entity>) -> Result<()> {
    let player_count = Text::new("How many players (2-6)?")
        .with_default("3")
        .prompt()?
        .parse::<usize>()?;

    if !(2..7).contains(&player_count) {
        println!("{}", "Must be 2-6 players!".red());
        return play_game();
    }
    for i in 0..player_count {
        let name = Text::new(&format!("Player {} name:", i + 1))
            .with_default(&format!("Player{}:", i + 1))
            .prompt()?;

        let player = world
            .spawn()
            .with(Player)?
            .with(Gamertag::new(name))?
            .with(Hand::new())?
            .build();

        players.push(player);
    }

    println!("\n{}", "✅ All players added!".bright_green());

    Ok(())
}

// This should be in main.rs (UI/input gathering)
fn get_player_names() -> Result<Vec<String>> {
    let player_count = Text::new("How many players (2-6)?")
        .with_default("3")
        .prompt()?
        .parse::<usize>()?;

    if !(2..7).contains(&player_count) {
        println!("{}", "Must be 2-6 players!".red());
        return get_player_names(); // Retry on invalid input
    }

    let mut names = Vec::new();
    for i in 0..player_count {
        let name = Text::new(&format!("Player {} name:", i + 1))
            .with_default(&format!("Player {}", i + 1))
            .prompt()?;
        names.push(name);
    }

    println!("\n{}", "✅ All players added!".bright_green());
    Ok(names)
}

fn show_title() {
    println!("\n{}", "═══════════════════════".bright_cyan());
    println!("{}", "   🎲 DUDO 🎲   ".red().bold());
    println!("{}", "═══════════════════════".bright_cyan());
}

fn show_rules() -> Result<()> {
    println!("\n{}", "📖 DUDO (Liar’s Dice) Rules 🎲🤥".blue().bold());

    println!("\n{}", "🎲 SETUP".yellow().bold());
    println!("  • Each player rolls 5 dice in secret and keeps them hidden");

    println!("\n{}", "🎯 GAMEPLAY".yellow().bold());
    println!("  • Players take turns making bids about total dice on the table");
    println!("  • Example bid: \"Five 3s\" (claiming there are at least five 3s total)");
    println!("  • Each bid must be HIGHER than the previous:");
    println!("    - More dice with same face (\"Six 3s\" beats \"Five 3s\")");
    println!("    - Same dice with higher face (\"Five 4s\" beats \"Five 3s\")");

    println!("\n{}", "⚔️  YOUR TURN".yellow().bold());
    println!(
        "  • {} Raise the bid (push it higher)",
        "OPTION 1:".bright_green()
    );
    println!(
        "  • {} Call DUDO! (challenge the bid)",
        "OPTION 2:".bright_red()
    );

    println!("\n{}", "🔍 WHEN DUDO IS CALLED".yellow().bold());
    println!("  • All players reveal their dice");
    println!("  • Count the total matching dice");
    println!("  • {} → Caller loses a die", "Bid was TRUE".green());
    println!("  • {} → Bidder loses a die", "Bid was FALSE".red());

    println!("\n{}", "🏆 WINNING".yellow().bold());
    println!("  • Lose all your dice → You're out!");
    println!("  • Last player with dice wins");

    println!("\n{}", "Press Enter to return...".dimmed());
    Text::new("").prompt()?;
    Ok(())
}

fn quit() -> Result<()> {
    println!("{}", "Thanks for playing! 👋".bright_green());
    Ok(())
}
