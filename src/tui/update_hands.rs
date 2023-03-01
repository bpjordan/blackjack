use crate::game_rules::round::{GameOver, DealerTurn, PlayerTurn, NotStarted, BlackjackTable};

use crate::game_rules::cards::Card;

use cursive::Cursive;
use cursive::views::{Dialog, DummyView, TextView, LinearLayout};


/// Update the display to reflect the dealer's current hand
pub fn update_dealer_hand(s: &mut Cursive, ascii: bool) {
    if let Some(_) = s.user_data::<BlackjackTable<NotStarted>>() {
        let card_display = LinearLayout::horizontal()
        .child(Dialog::text(Card::flipped_icon(ascii)))
        .child(DummyView)
        .child(Dialog::text(Card::flipped_icon(ascii)))
        .child(DummyView)
        .child(Dialog::text(Card::flipped_icon(ascii)));

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_title("Dealer's hand");
            v.set_content(card_display);
        });
    } else if let Some(table) = s.user_data::<BlackjackTable<PlayerTurn>>() {
        let showing_card_icon = table.showing_card().unwrap().icon(ascii);

        let card_display = LinearLayout::horizontal()
        .child(Dialog::text(showing_card_icon))
        .child(DummyView)
        .child(Dialog::text(Card::flipped_icon(ascii)));

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_title("Dealer's hand");
            v.set_content(card_display);
        });

    } else if let Some(table) = s.user_data::<BlackjackTable<DealerTurn>>() {
        let card_display = display_full_hand(table.dealer_hand().cards(), ascii);
        let score = table.dealer_hand().total_value();

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_title(format!("Dealer's hand (score: {}):", score));
            v.set_content(card_display);
        });

    } else if let Some(table) = s.user_data::<BlackjackTable<GameOver>>() {
    
        let card_display = display_full_hand(table.dealer_hand().cards(), ascii);
        let score = table.dealer_hand().total_value();

        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_title(format!("Dealer's hand (score: {}):", score));
            v.set_content(card_display);
        });

    } else {
        s.call_on_name("dealer_hand", |v: &mut Dialog| {
            v.set_content(TextView::new("Failed to display dealer's hand"));
        });
    }
}

/// Update the display to reflect the player's current hand
pub fn update_player_hand(s: &mut Cursive, ascii: bool) {
    if let Some(_) = s.user_data::<BlackjackTable<NotStarted>>() {
        let card_display = LinearLayout::horizontal()
        .child(Dialog::text(Card::flipped_icon(ascii)))
        .child(DummyView)
        .child(Dialog::text(Card::flipped_icon(ascii)))
        .child(DummyView)
        .child(Dialog::text(Card::flipped_icon(ascii)));

        s.call_on_name("player_hand", |v: &mut Dialog| {
            v.set_title("Your hand");
            v.set_content(card_display);
        });
    } else if let Some(table) = s.user_data::<BlackjackTable<PlayerTurn>>() {

        let card_display = display_full_hand(table.player_hand().cards(), ascii);
        let score = table.player_hand().total_value();

        s.call_on_name("player_hand", |v: &mut Dialog| {
            v.set_title(format!("Your hand (score: {}):", score));
            v.set_content(card_display);
        });

    } else if let Some(table) = s.user_data::<BlackjackTable<DealerTurn>>() {

        let card_display = display_full_hand(table.player_hand().cards(), ascii);
        let score = table.player_hand().total_value();

        s.call_on_name("player_hand", |v: &mut Dialog| {
            v.set_title(format!("Your hand (score: {}):", score));
            v.set_content(card_display);
        });

    } else if let Some(table) = s.user_data::<BlackjackTable<GameOver>>() {

        let card_display = display_full_hand(table.player_hand().cards(), ascii);
        let score = table.player_hand().total_value();

        s.call_on_name("player_hand", |v: &mut Dialog| {
            v.set_title(format!("Your hand (score: {}):", score));
            v.set_content(card_display);
        });

    } else {
        s.call_on_name("player_hand", |v: &mut Dialog| {
            v.set_content(TextView::new("Failed to display player's hand"));
        });
    }
}

fn display_full_hand(hand: &Vec<Card>, ascii: bool) -> LinearLayout {
    let mut l = LinearLayout::horizontal();

    for c in hand {
        l.add_child(Dialog::text(c.icon(ascii)));
        l.add_child(DummyView);
    }

    l
}