use super::update_hands::{update_dealer_hand, update_player_hand};
use super::{stand_callback, hit_callback, error_popup, set_message};

use crate::game_rules::round::{GameResult, GameOver, GameStartResult, NotStarted};

use cursive::views::Dialog;

use crate::game_rules::round::BlackjackTable;

use crate::Config;

use cursive::Cursive;

/// Start a new game and prompt the player to deal
/// a new round
pub(crate) fn init_round(s: &mut Cursive, cfg: Config) {
    let table = BlackjackTable::default();

    s.set_user_data(table);

    update_dealer_hand(s, cfg.ascii);
    update_player_hand(s, cfg.ascii);

    set_message(s, "Press q any time to quit");

    s.call_on_name("game_dialog", move |d: &mut Dialog| {
        d.clear_buttons();
        d.add_button("Deal", move |s| deal_round(s, cfg));
    });

}

/// Shuffle the deck, deal, and begin the player's turn
pub(crate) fn deal_round(s: &mut Cursive, cfg: Config) {
    if let Some(table) = s.take_user_data::<BlackjackTable<NotStarted>>() {

        match table.shuffle().deal() {
            Ok(GameStartResult::Natural(t)) => {
                s.set_user_data(t);

                end_game(s, cfg);
            },
            Ok(GameStartResult::Normal(t)) => {
                s.set_user_data(t);

                start_player_turn(s, cfg);
            },
            Err(_) => error_popup(s, "Unable to deal"),
        }

    } else {
        error_popup(s, "Invalid game state");
    }
}

/// Prompt the player to play their turn by hitting or standing
pub(crate) fn start_player_turn(s: &mut Cursive, cfg: Config) {

    update_dealer_hand(s, cfg.ascii);
    update_player_hand(s, cfg.ascii);

    set_message(s, "It's your turn!");

    s.call_on_name("game_dialog", |d: &mut Dialog| {
        d.clear_buttons();
        d.add_button("Hit", move |s| hit_callback(s, cfg));
        d.add_button("Stand", move |s| stand_callback(s, cfg));
    });
}

/// End the round and display the results
pub(crate) fn end_game(s: &mut Cursive, cfg: Config) {
    update_dealer_hand(s, cfg.ascii);
    update_player_hand(s, cfg.ascii);

    if let Some(table) = s.user_data::<BlackjackTable<GameOver>>() {
        let msg = match table.result() {
            GameResult::DealerWin => "The dealer won",
            GameResult::PlayerWin => "You win!",
            GameResult::DealerBust => "The dealer went bust! You win!",
            GameResult::PlayerBust => "You went bust!",
            GameResult::StandOff => "You and the dealer are in a stand off!",
        };

        set_message(s, msg);
    }

    s.call_on_name("game_dialog", move |d: &mut Dialog| {
        d.clear_buttons();
        d.add_button("Play Again", move |s| init_round(s, cfg));
    });
}
