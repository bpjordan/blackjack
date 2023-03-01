use cursive::{theme::{Theme, BorderStyle, Palette}, With};

pub fn blackjack_theme() -> Theme {
    Theme {
        shadow: true,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|p| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::PaletteColor::*;

            p[Background] = Green.light();
        })
    }
}