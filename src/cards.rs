
use std::fmt::Display;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit_str = match self {
            CardSuit::Hearts => "Hearts",
            CardSuit::Diamonds => "Diamonds",
            CardSuit::Spades => "Spades",
            CardSuit::Clubs => "Clubs",
        };
        write!(f, "{suit_str}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardFace {
    Number(u8),
    Jack,
    King,
    Queen,
    Ace
}

#[derive(Debug)]
pub struct CardValueError(u8);

impl CardFace {
    pub fn number(val: u8) -> Result<Self, CardValueError> {
        if (1..=9).contains(&val) {
            Ok(Self::Number(val))
        } else {
            Err(CardValueError(val))
        }
    }
}

impl From<CardFace> for u8 {
    fn from(value: CardFace) -> Self {
        match value {
            CardFace::Number(v) => v,
            CardFace::Jack => 10,
            CardFace::King => 10,
            CardFace::Queen => 10,
            CardFace::Ace => 11,
        }
    }
}

impl Display for CardFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_str = match self {
            CardFace::Number(v) => v.to_string(),
            CardFace::Jack => "Jack".into(),
            CardFace::King => "King".into(),
            CardFace::Queen => "Queen".into(),
            CardFace::Ace => "Ace".into(),
        };

        write!(f, "{face_str}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: CardSuit,
    face: CardFace
}

impl Card {
    pub fn new(face: CardFace, suit: CardSuit) -> Self {
        Self { suit, face }
    }

    /// Generate a card with a random face and suit
    pub fn random() -> Result<Self, CardValueError> {
        let mut rng = rand::thread_rng();

        let suit = match rng.gen_range(1..=4) {
            1 => CardSuit::Hearts,
            2 => CardSuit::Diamonds,
            3 => CardSuit::Spades,
            4 => CardSuit::Clubs,
            e => return Err(CardValueError(e))
        };

        let face = match rng.gen_range(1..=12) {
            1 => CardFace::Ace,
            10 => CardFace::Jack,
            11 => CardFace::Queen,
            12 => CardFace::King,
            v => CardFace::number(v)?
        };

        Ok(Self { suit, face })
    }
}

impl From<Card> for u8 {
    fn from(value: Card) -> Self {
        Self::from(value.face)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.face, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, CardFace, CardSuit};

    #[test]
    fn card_strings() {
        assert_eq!(
            Card::new(CardFace::Ace, CardSuit::Spades).to_string(),
            "Ace of Spades".to_owned()
        );

        assert_eq!(
            Card::new(CardFace::number(3).unwrap(), CardSuit::Hearts).to_string(),
            "3 of Hearts".to_owned()
        );
    }

    #[test]
    fn card_values() {
        assert_eq!(
            u8::from(
                Card::new(CardFace::number(8).unwrap(), CardSuit::Diamonds)
            ),
            8
        );

        assert_eq!(
            u8::from(
                Card::new(CardFace::King, CardSuit::Clubs)
            ),
            10
        )
    }

    #[test]
    fn random_cards() {
        for _ in 0..100 {
            Card::random().expect("Random card generation failed");
        }
    }
}