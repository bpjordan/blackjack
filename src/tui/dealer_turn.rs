use super::{end_game, set_message};

use super::hand_display::display_dealer_hand;

use crate::Config;
use crate::game_rules::round::{DealerTurnResult, DealerTurn, BlackjackTable};

use std::sync::mpsc;
use std::time::Duration;

use cursive::Cursive;
use cursive::views::Dialog;

pub(crate) fn run_dealer_turn(s: &mut Cursive, cfg: Config) {
    s.call_on_name("game_dialog", |d: &mut Dialog| {
        d.clear_buttons();
    });

    set_message(s, "The dealer is playing...");

    let cb_sink = s.cb_sink().clone();

    std::thread::spawn(move || {
        let pause_time = Duration::from_millis(500);

        let (tx, rx) = mpsc::channel();

        loop {
            std::thread::sleep(pause_time);

            match rx.try_recv() {
                Ok(()) | Err(mpsc::TryRecvError::Disconnected) => {
                    break
                },
                Err(mpsc::TryRecvError::Empty) => {}
            }

            let shutdown_tx = tx.clone();

            cb_sink.send(Box::new(move |s| {
                if let Some(table) = s.take_user_data::<BlackjackTable<DealerTurn>>() {
                    match table.hit() {
                        Ok(DealerTurnResult::Hit(t)) => {
                            s.set_user_data(t);
                            display_dealer_hand(s, cfg.ascii);
                        },
                        Ok(DealerTurnResult::Stand(t)) => {
                            shutdown_tx.send(()).unwrap();

                            s.set_user_data(t);
                            display_dealer_hand(s, cfg.ascii);

                            end_game(s, cfg);
                        }
                        Err(_) => todo!(),
                    }
                }
            })).unwrap();
        }
    });
}
