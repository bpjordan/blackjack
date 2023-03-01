
use cursive::view::Nameable;
use cursive::{Cursive, CursiveExt};
use cursive::views::{LinearLayout, Dialog, TextView};

use crate::game_rules::cards::Card;
use crate::{Config};
use crate::game_rules::round::{BlackjackTable, PlayerTurn, PlayerTurnResult, GameOver, GameStartResult};

pub fn run_game(cfg: Config) {
    let mut tui = Cursive::default();

    match BlackjackTable::default().shuffle().deal() {
        Ok(GameStartResult::Natural(s)) => tui.set_user_data(s),
        Ok(GameStartResult::Normal(s)) => tui.set_user_data(s),
        Err(_) => todo!(),
    }
    
    tui.add_global_callback('q', |s| s.quit());

    let hands = LinearLayout::vertical()
    .child(Dialog::text("Dealer's hand placeholder").title("Dealer").with_name("dealer_hand"))
    .child(Dialog::text("Player's hand placeholder").title("Your Hand").with_name("player_hand"));

    tui.add_layer(
        Dialog::around(hands)
        .button("Hit", hit_callback)
        .button("Stand", stand_callback)
    );

    display_dealer_hand(&mut tui, cfg.ascii);

    tui.run();

}

fn hit_callback(s: &mut Cursive) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        match table.hit() {
            Ok(PlayerTurnResult::Hit(new_table)) => s.set_user_data(new_table),
            Ok(PlayerTurnResult::Bust(new_table)) => s.set_user_data(new_table),
            Err(_) => todo!(),
        }
    };
}

fn stand_callback(s: &mut Cursive) {
    if let Some(table) = s.take_user_data::<BlackjackTable<PlayerTurn>>() {
        s.set_user_data(table.stand());
    }
}

fn display_dealer_hand(s: &mut Cursive, ascii: bool) {
    if let Some(table) = s.user_data::<BlackjackTable<PlayerTurn>>() {
        let showing_card_icon = table.showing_card().unwrap().icon(ascii);

        let card_display = LinearLayout::horizontal()
        .child(TextView::new(showing_card_icon))
        .child(TextView::new(Card::flipped_icon(ascii)));

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_content(card_display);
        });

    } else if let Some(table) = s.user_data::<BlackjackTable<GameOver>>() {
        
        let mut card_display = LinearLayout::horizontal();

        for c in table.dealer_hand().cards() {
            card_display.add_child(TextView::new(c.icon(ascii)));
        }

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_content(card_display);
        });

    } else {
        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_content(TextView::new("Failed to display dealer's hand"));
        });
    }
}