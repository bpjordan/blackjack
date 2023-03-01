
use cursive::view::Nameable;
use cursive::{Cursive, CursiveExt};
use cursive::views::{LinearLayout, Dialog, TextView, };

use crate::Config;
use crate::game_rules::round::{
    BlackjackTable,
    PlayerTurn,
    PlayerTurnResult,
    GameStartResult,
    NotStarted, GameOver, GameResult
};

mod hand_display;
mod dealer_turn;

use self::hand_display::{display_dealer_hand, display_player_hand};
use dealer_turn::run_dealer_turn;


pub fn run_game(cfg: Config) {
    let mut tui = Cursive::default();

    tui.add_global_callback('q', |s| s.quit());

    let hands = LinearLayout::vertical()
    .child(Dialog::text("Dealer's hand placeholder").title("Dealer").with_name("dealer_hand"))
    .child(Dialog::text("Player's hand placeholder").title("Your Hand").with_name("player_hand"))
    .child(Dialog::text("message placeholder").with_name("message_box"));

    tui.add_layer(
        Dialog::around(hands).with_name("game_dialog")
    );

    init_round(&mut tui, cfg);

    tui.run();

}

fn hit_callback(s: &mut Cursive, cfg: Config) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        match table.hit() {
            Ok(PlayerTurnResult::Hit(new_table)) => s.set_user_data(new_table),
            Ok(PlayerTurnResult::Bust(new_table)) => {
                s.set_user_data(new_table);

                end_game(s, cfg)
            },
            Err(_) => todo!(),
        }

        hand_display::display_player_hand(s, cfg.ascii)
    } else {
        error_popup(s, "Invalid game state");
    };
}

fn stand_callback(s: &mut Cursive, cfg: Config) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        s.set_user_data(table.stand());

        run_dealer_turn(s, cfg);
    }
}

fn error_popup<S: Into<String>>(s: &mut Cursive, msg: S) {

    s.add_layer(
        Dialog::text(msg)
        .title("Error")
        .button("Quit", |s| s.quit())
    )

}

fn init_round(s: &mut Cursive, cfg: Config) {
    let table = BlackjackTable::default();

    s.set_user_data(table);

    hand_display::display_dealer_hand(s, cfg.ascii);
    hand_display::display_player_hand(s, cfg.ascii);

    set_message(s, "Press q any time to quit");

    s.call_on_name("game_dialog", move |d: &mut Dialog| {
        d.clear_buttons();
        d.add_button("Deal", move |s| deal_round(s, cfg));
    });

}

fn deal_round(s: &mut Cursive, cfg: Config) {
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

fn start_player_turn(s: &mut Cursive, cfg: Config) {

    display_dealer_hand(s, cfg.ascii);
    display_player_hand(s, cfg.ascii);

    set_message(s, "It's your turn!");

    s.call_on_name("game_dialog", |d: &mut Dialog| {
        d.clear_buttons();
        d.add_button("Hit", move |s| hit_callback(s, cfg));
        d.add_button("Stand", move |s| stand_callback(s, cfg));
    });
}

fn end_game(s: &mut Cursive, cfg: Config) {
    hand_display::display_dealer_hand(s, cfg.ascii);
    hand_display::display_player_hand(s, cfg.ascii);

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

fn set_message<S: Into<String>>(s: &mut Cursive, msg: S) {
    s.call_on_name("message_box", |d: &mut Dialog| {
        d.set_content(TextView::new(msg));
    });
}