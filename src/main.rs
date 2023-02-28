
use clap::Parser;

mod cli;
mod game_rules;

#[derive(Parser)]
#[command(author = "Bronson Jordan")]
#[command(about = "Play Blackjack in a CLI or TUI")]
pub struct Config {

    /// Only use ASCII text
    /// (By default UTF-8 characters are used for card suits)
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
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
        println!("Sorry, the TUI version of the game is not implemented yet");
        println!("Run with -c to start the CLI game");
    }
}