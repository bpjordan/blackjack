
use rand::seq::SliceRandom;

use crate::cards::{CardFace, CardSuit};

use super::cards::Card;

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new(num_decks: u8) -> Self {
        let mut v = Vec::with_capacity(52 * <u8 as Into<usize>>::into(num_decks));

        for _ in 0..num_decks {
            for c in 1..=13 {
                v.push(Card::new(CardFace::try_from(c).unwrap(), CardSuit::Clubs));
                v.push(Card::new(CardFace::try_from(c).unwrap(), CardSuit::Diamonds));
                v.push(Card::new(CardFace::try_from(c).unwrap(), CardSuit::Hearts));
                v.push(Card::new(CardFace::try_from(c).unwrap(), CardSuit::Spades));
            }
        };

        Self(v)
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new(6)
    }
}

#[cfg(test)]
mod test {
    use crate::cards::{Card, CardFace, CardSuit};

    use super::Deck;


    #[test]
    fn initialize() {
        let mut d = Deck::default();

        assert_eq!(d.0.len(), 312);
        assert_eq!(d.0[0], Card::new(CardFace::Ace, CardSuit::Clubs));
        assert_eq!(d.0[311], Card::new(CardFace::King, CardSuit::Spades));

        assert_eq!(d.draw(), Some(Card::new(CardFace::King,CardSuit::Spades)));
        assert_eq!(d.0.len(), 311);
    }
}