
use std::fmt::Display;

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
            Card::new(CardFace::Number(3), CardSuit::Hearts).to_string(),
            "3 of Hearts".to_owned()
        );
    }
}