
use clap::Parser;

mod cli;
mod tui;
mod game_rules;

#[derive(Parser, Clone, Copy)]
#[command(author = "Bronson Jordan")]
#[command(about = "Play Blackjack in a CLI or TUI")]
pub struct Config {

    /// Only use ASCII text
    /// (By default UTF-8 characters are used for card suits)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    ascii: bool,

    /// Run the game as a CLI instead of a TUI
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    cli: bool
}

fn main() {

    let config = Config::parse();

    if config.cli {
        cli::run_game(config);
    } else {
        tui::run_game(config);
    }
}