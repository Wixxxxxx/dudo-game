use anyhow::Result;
use colored::Colorize;
use inquire::{Select, Text};

use dudo_core::{events::ClientEvent, GameLoop, PlayerAction};

fn main() -> Result<()> {
    loop {
        show_title();
        if !main_menu()? {
            break;
        }

        let player_names = get_player_names()?;
        let mut game = GameLoop::new(player_names)?;

        loop {
            // process game logic
            let ui_events = game.tick()?;

            for event in ui_events {
                display_event(event);
            }

            if game.is_game_over()? {
                break;
            }

            let can_challenge = game.can_challenge()?;
            let action = get_player_action(&mut game, can_challenge)?;
            game.submit_action(action)?;
        }
    }

    Ok(())
}

fn display_event(event: ClientEvent) {
    match event {
        ClientEvent::DisplayCurrentTurn { player_name } => {
            println!(
                "\n{}",
                format!("─── {}'s Turn ───", player_name)
                    .bright_green()
                    .bold()
            );
        }
        ClientEvent::DisplayGameOver { winner_name } => {
            println!(
                "\n{}",
                format!("🏆 {} WINS! 🏆", winner_name)
                    .bright_yellow()
                    .bold()
            );
        }
        ClientEvent::DisplayPlayerEliminated { player_name } => {
            println!(
                "\n{}",
                format!("❌ {} has been eliminated!", player_name).red()
            );
        }
        ClientEvent::DisplayChallengeLoser { loser } => {
            println!("\n{}", format!("Challenge lost by {}!", loser).red());
        }
        ClientEvent::DisplayChallenge { challenger_name } => {
            println!(
                "\n{}, by {}",
                "⚔️ CHALLENGE!".bright_red().bold(),
                challenger_name
            );
        }
        ClientEvent::DisplayBid {
            player_name,
            quantity,
            face,
        } => {
            println!(
                "\n{}, player: {}, quantity: {}, face: {}",
                "BID PLACED".bright_green().bold(),
                player_name,
                quantity,
                face
            )
        }
    }
}

fn main_menu() -> Result<bool> {
    let menu = vec!["Start", "Rules", "Quit"];
    let menu_choice = Select::new("Main Menu", menu).prompt()?;

    match menu_choice {
        "Start" => Ok(true),
        "Rules" => {
            show_rules()?;
            main_menu()
        }
        "Quit" => {
            quit()?;
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn get_player_action(game: &mut GameLoop, can_challenge: bool) -> Result<PlayerAction> {
    let actions = if can_challenge {
        vec![
            "Inspect Dice",
            "View Bid History",
            "Raise Bid",
            "Call Bluff",
        ]
    } else {
        vec!["Inspect Dice", "View Bid History", "Make First Bid"]
    };

    let choice = Select::new("Choose action:", actions).prompt()?;

    match choice {
        "Inspect Dice" => {
            // display current player's hand
            let hand = game.get_current_player_hand()?;
            println!("{}", hand);
            get_player_action(game, can_challenge)
        }
        "View Bid History" => {
            let history = game.get_bid_history()?;
            println!("\n{}", history.yellow());
            get_player_action(game, can_challenge)
        }
        "Raise Bid" | "Make First Bid" => {
            let current_bid = game.get_current_bid()?;
            let (quantity, face) = get_bid_from_player(current_bid)?;
            Ok(PlayerAction::Bid { quantity, face })
        }
        "Call Bluff" => Ok(PlayerAction::Challenge),
        _ => get_player_action(game, can_challenge),
    }
}

fn get_bid_from_player(current_bid: Option<(u8, u8)>) -> Result<(u8, u8)> {
    let quantity = Text::new("How many dice?").prompt()?.parse::<u8>()?;

    let face = Text::new("What face value (1-6)?")
        .prompt()?
        .parse::<u8>()?;

    // validate face value
    if !(1..=6).contains(&face) {
        println!("{}", "Face must be 1-6!".red());
        return get_bid_from_player(current_bid);
    }

    // validate new bid is higher than current bid
    if let Some((curr_quantity, curr_face)) = current_bid {
        if !is_higher_bid(quantity, face, curr_quantity, curr_face) {
            println!(
                "{}",
                format!(
                    "Bid must be higher than {} × {}! Try again.",
                    curr_quantity, curr_face
                )
                .red()
            );
            return get_bid_from_player(current_bid);
        }
    }
    Ok((quantity, face))
}

fn is_higher_bid(new_quantity: u8, new_face: u8, curr_quantity: u8, curr_face: u8) -> bool {
    if new_quantity > curr_quantity && new_face == curr_face {
        return true;
    }

    if new_face > curr_face && new_quantity == curr_quantity {
        return true;
    }

    false
}

fn get_player_names() -> Result<Vec<String>> {
    let player_count = Text::new("How many players (2-6)?")
        .with_default("3")
        .prompt()?
        .parse::<usize>()?;

    if !(2..7).contains(&player_count) {
        println!("{}", "Must be 2-6 players!".red());
        return get_player_names();
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
