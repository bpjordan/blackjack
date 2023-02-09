
use super::cards::{Card, CardFace};

/// A player's hand. Wrapper struct for a vec of Cards.
/// 
/// Can also provide the total value of the hand, accounting
/// for Aces being either 11 or 1
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    fn values(&self) -> Vec<ValueInHand> {
        self.cards().iter().map(|c| {
            match c.face() {
                CardFace::Number(v) => ValueInHand::Set(v.clone()),
                CardFace::Jack | CardFace::King | CardFace::Queen => ValueInHand::Set(10),
                CardFace::Ace => ValueInHand::Wild,
            }
        }).collect()
    }

    pub fn total_value(&self) -> u8 {

        let vals = self.values();

        // Start by summing all non-wild card values.
        // We need this to calculate the value of any wilds
        let mut sum: u8 = vals.iter().filter_map(|v| match v {
            ValueInHand::Set(s) => Some(s),
            ValueInHand::Wild => None
        }).sum();

        let num_wilds = vals.iter()
            .filter(|v| matches!(v, ValueInHand::Wild))
            .count()
            .try_into()
            .unwrap_or(21); //Since the exact value ultimately doesn't matter once it's over 21

        // Add the wilds to the total value of the hand.
        // The first wild has value 11, unless that would cause the sum to be over 21.
        // All other wilds are worth 1
        if num_wilds > 0 && sum + 11 + (num_wilds - 1) <= 21 {
            sum += 11 + (num_wilds - 1);
        } else {
            sum += num_wilds;
        }

        sum
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ValueInHand {
    Set(u8),
    Wild
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, CardFace, CardSuit};

    use super::{Hand, ValueInHand};


    #[test]
    fn hand_values() {
        let h = Hand::new(vec![
            Card::new(CardFace::try_from(8).unwrap(), CardSuit::Spades),
            Card::new(CardFace::try_from(12).unwrap(), CardSuit::Diamonds)
        ]);

        let mut i = h.values().into_iter();

        assert_eq!(i.next(), Some(ValueInHand::Set(8)));
        assert_eq!(i.next(), Some(ValueInHand::Set(10)));
        assert_eq!(i.next(), None);

        let h = Hand::new(vec![
            Card::new(CardFace::try_from(1).unwrap(), CardSuit::Clubs),
            Card::new(CardFace::try_from(11).unwrap(), CardSuit::Hearts)
        ]);

        let mut i = h.values().into_iter();

        assert_eq!(i.next(), Some(ValueInHand::Wild));
        assert_eq!(i.next(), Some(ValueInHand::Set(10)));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn hand_total_values() {
        let h = Hand::new(vec![
            Card::new(CardFace::try_from(8).unwrap(), CardSuit::Spades),
            Card::new(CardFace::try_from(12).unwrap(), CardSuit::Diamonds)
        ]);

        assert_eq!(h.total_value(), 18);

        let h = Hand::new(vec![
            Card::new(CardFace::try_from(1).unwrap(), CardSuit::Clubs),
            Card::new(CardFace::try_from(11).unwrap(), CardSuit::Hearts)
        ]);

        assert_eq!(h.total_value(), 21);

        let h = Hand::new(vec![
            Card::new(CardFace::try_from(1).unwrap(), CardSuit::Clubs),
            Card::new(CardFace::try_from(8).unwrap(), CardSuit::Hearts),
            Card::new(CardFace::try_from(6).unwrap(), CardSuit::Hearts)
        ]);

        assert_eq!(h.total_value(), 15)
    }

    #[test]
    fn push_to_values() {
        let mut h = Hand::new(vec![
            Card::new(CardFace::try_from(8).unwrap(), CardSuit::Spades),
            Card::new(CardFace::try_from(12).unwrap(), CardSuit::Diamonds)
        ]);

        let mut i = h.values().into_iter();

        assert_eq!(i.next(), Some(ValueInHand::Set(8)));
        assert_eq!(i.next(), Some(ValueInHand::Set(10)));
        assert_eq!(i.next(), None);

        h.push(Card::new(CardFace::try_from(6).unwrap(), CardSuit::Diamonds));

        let mut i = h.values().into_iter();

        assert_eq!(i.next(), Some(ValueInHand::Set(8)));
        assert_eq!(i.next(), Some(ValueInHand::Set(10)));
        assert_eq!(i.next(), Some(ValueInHand::Set(6)));
        assert_eq!(i.next(), None);
    }
}