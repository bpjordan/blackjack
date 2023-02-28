use std::io::{stdin, Read, stdout, Write};

use crate::game_rules::round::{
    BlackjackTable,
    GameStartResult,
    BlackjackRoundError,
    PlayerTurnResult,
    DealerTurnResult,
    GameResult
};

pub fn run_game() {

    let mut user_input = [0u8];

    'game_loop: loop {

        print!("Press [s] to start or [q] to quit: ");

        'start_loop: loop {
            stdout().flush().unwrap();

            let choice = match stdin().read(&mut user_input[..]) {
                Ok(1) => user_input[0],
                Err(_) | Ok(_) => {
                    eprintln!("Encountered an error reading from stdin, stopping game...");

                    break 'game_loop;
                },
            };

            match choice {
                b's' | b'S' => {
                    break 'start_loop;
                }

                b'q' | b'Q' => {
                    println!("Goodbye!");

                    break 'game_loop;
                }

                b'\n' => { }

                s => {
                    println!("{} is not a choice, try again", s as char)
                }
            }
        }

        println!("Starting a new blackjack game...");
        println!("Dealing...");

        let mut players_turn = match BlackjackTable::default().shuffle().deal() {
            Ok(GameStartResult::Normal(s)) => s,
            Ok(GameStartResult::Natural(s)) => {
                let result_str = match s.result() {
                    GameResult::DealerWin => "The Dealer got a natural! Better luck next time!",
                    GameResult::PlayerWin => "You got a natural! Great job!",
                    GameResult::StandOff => "You and the dealer both got a Natural! It's a stand off!",
                    _ => "An unexpected game result occured"
                };

                println!("{}", result_str);

                continue;
            }

            Err(BlackjackRoundError::DeckEmpty) => {
                println!("There weren't enough cards in the deck to deal.");
                break;
            },
        };

        match players_turn.showing_card() {
            Some(c) => println!("The dealer is showing a {}", c),
            None => unreachable!(),
        }

        println!("Your hand is:");

        for c in players_turn.player_hand().cards() {
            println!("{c}")
        }

        let mut dealers_turn = 'player_turn_loop: loop {


            println!("Your hand's value is {}", players_turn.player_hand().total_value());

            print!("Would you like to [h]it or [s]tand? > ");
            stdout().flush().unwrap();

            let choice = match stdin().read(&mut user_input[..]) {
                Ok(1) => user_input[0],
                Err(_) | Ok(_) => {
                    println!("Failed to read user input, exiting");

                    break 'game_loop;
                },
            };

            match choice {
                b'h' | b'H' => {
                    players_turn = match players_turn.hit() {
                        Ok(PlayerTurnResult::Hit(s)) => {
                            println!("You drew a {}", s.player_hand().cards().last().unwrap());

                            s
                        },
                        Ok(PlayerTurnResult::Bust(s)) => {

                            println!("You drew a {} and went bust!", s.player_hand().cards().last().unwrap());

                            continue 'game_loop;
                        }
                        Err(_) => todo!(),
                    };
                }

                b's' | b'S' => {
                    break 'player_turn_loop players_turn.stand();
                }

                b'q' | b'Q' => {
                    println!("Goodbye!");

                    break 'game_loop;
                }

                b'\n' => {}

                s => {
                    println!("Unexpected response {s}")
                }
            };

        };

        println!("The dealer's hand is:");

        for c in dealers_turn.dealer_hand().cards() {
            println!("{c}");
        }

        let round_result = loop {

            match dealers_turn.hit() {
                Ok(DealerTurnResult::Hit(s)) => {
                    println!("Dealer drew a {}", s.dealer_hand().cards().last().unwrap());

                    dealers_turn = s;
                },
                Ok(DealerTurnResult::Stand(s)) => {
                    println!("Dealer drew a {}", s.dealer_hand().cards().last().unwrap());

                    break s;
                },
                Err(_) => todo!(),
            }

        };

        println!(
            "The dealer's score is {} and your score is {}",
            round_result.dealer_hand().total_value(),
            round_result.player_hand().total_value()
        );

        match round_result.result() {
            GameResult::DealerWin => println!("The dealer wins"),
            GameResult::PlayerWin => println!("You win!"),
            GameResult::DealerBust => println!("The dealer went bust! You win!"),
            GameResult::PlayerBust => println!("You went bust! The dealer wins"),
            GameResult::StandOff => println!("You and the dealer are in a stand off!"),
        }

    }
}
