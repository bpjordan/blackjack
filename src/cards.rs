
use std::fmt::Display;

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardFace {
    Number(u8),
    Jack,
    King,
    Queen,
    Ace
}

#[derive(Debug)]
pub struct CardValueError(u8);

impl TryFrom<u8> for CardFace {
    type Error = CardValueError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ace),
            v @ 2..=10 => Ok(Self::Number(v)),
            11 => Ok(Self::Jack),
            12 => Ok(Self::Queen),
            13 => Ok(Self::King),
            e => Err(CardValueError(e))
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
            CardFace::Ace => 1,
        }
    }
}

impl Display for CardFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_str = match self {
            CardFace::Number(v) => v.to_string(),
            CardFace::Jack => "Jack".into(),
            CardFace::Queen => "Queen".into(),
            CardFace::King => "King".into(),
            CardFace::Ace => "Ace".into(),
        };

        write!(f, "{face_str}")
    }
}

/// A standard playing card (without Jokers).
/// 
/// Contains a Suit and a Face.
/// 
/// # Example
/// 
/// Generate a new card with new():
/// ```
/// use blackjack::cards::{Card, CardFace, CardSuit};
/// 
/// let card = Card::new(CardFace::try_from(4).unwrap(), CardSuit::Clubs);
/// assert_eq!(card.face(), &CardFace::Number(4));
/// assert_eq!(card.suit(), &CardSuit::Clubs);
/// assert_eq!(card.to_string(), "4 of Clubs".to_string());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

        let face = CardFace::try_from(rng.gen_range(1..=12))?;

        Ok(Self { suit, face })
    }

    pub fn face(&self) -> &CardFace {
        &self.face
    }

    pub fn suit(&self) -> &CardSuit {
        &self.suit
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
            Card::new(CardFace::try_from(3).unwrap(), CardSuit::Hearts).to_string(),
            "3 of Hearts".to_owned()
        );
    }

    #[test]
    fn card_values() {
        assert_eq!(
            u8::from(
                Card::new(CardFace::try_from(8).unwrap(), CardSuit::Diamonds)
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