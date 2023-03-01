
use cursive::theme::{PaletteColor, Color, BaseColor};
use cursive::view::Nameable;
use cursive::{Cursive, CursiveExt};
use cursive::views::{LinearLayout, Dialog, TextView};

use crate::Config;
use crate::game_rules::round::{
    BlackjackTable,
    PlayerTurn,
    PlayerTurnResult,
    
};

mod update_hands;
mod dealer_turn;

use dealer_turn::run_dealer_turn;

/// Start a tui game.
/// 
/// Returns when tui is exited
pub fn run_game(cfg: Config) {
    let mut tui = Cursive::default();

    tui.add_global_callback('q', |s| s.quit());

    tui.update_theme(|t| {
        t.palette[PaletteColor::Background] = Color::Dark(BaseColor::Green);
    });

    let hands = LinearLayout::vertical()
    .child(Dialog::text("Dealer's hand placeholder").title("Dealer").with_name("dealer_hand"))
    .child(Dialog::text("Player's hand placeholder").title("Your Hand").with_name("player_hand"))
    .child(Dialog::text("message placeholder").with_name("message_box"));

    tui.add_layer(
        Dialog::around(hands).with_name("game_dialog")
    );

    game_states::init_round(&mut tui, cfg);

    tui.run();

}

/// Callback for a player hitting during their turn.
/// 
/// Will error if called outside of a player's turn
fn hit_callback(s: &mut Cursive, cfg: Config) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        match table.hit() {
            Ok(PlayerTurnResult::Hit(new_table)) => s.set_user_data(new_table),
            Ok(PlayerTurnResult::Bust(new_table)) => {
                s.set_user_data(new_table);

                game_states::end_game(s, cfg)
            },
            Err(_) => todo!(),
        }

        update_hands::update_player_hand(s, cfg.ascii)
    } else {
        error_popup(s, "Invalid game state");
    };
}

/// Callback for a player standing to end their turn
/// 
/// Will error if called outside of a player's turn
fn stand_callback(s: &mut Cursive, cfg: Config) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        s.set_user_data(table.stand());

        run_dealer_turn(s, cfg);
    }
}

/// Display an error dialog box with the supplied message
/// 
/// Player cannot return to the game once this popup
/// is active, they can only quit
fn error_popup<S: Into<String>>(s: &mut Cursive, msg: S) {

    s.add_layer(
        Dialog::text(msg)
        .title("Error")
        .button("Quit", |s| s.quit())
    )

}

mod game_states;

/// Set the message at the bottom of the screen to the supplied string
fn set_message<S: Into<String>>(s: &mut Cursive, msg: S) {
    s.call_on_name("message_box", |d: &mut Dialog| {
        d.set_content(TextView::new(msg));
    });
}