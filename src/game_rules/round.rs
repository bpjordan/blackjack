
use super::{deck::Deck, hand::Hand, cards::Card};
use std::fmt::Debug;

// The game is a state machine with the following states
// These states are types so that we can guard certain actions
// So they can only occur during the appropriate game state.

#[derive(Debug, Default)]
pub struct NotStarted;

#[derive(Debug, Default)]
pub struct PlayerTurn;

#[derive(Debug, Default)]
pub struct DealerTurn;

#[derive(Debug, Default)]
pub struct GameOver(GameResult);

#[derive(Debug, Default)]
pub enum GameResult {
    #[default] DealerWin,
    PlayerWin,
    DealerBust,
    PlayerBust,
    StandOff
}

pub trait BlackjackTableState: Debug + Default + Sized {}

impl BlackjackTableState for NotStarted {}
impl BlackjackTableState for PlayerTurn {}
impl BlackjackTableState for DealerTurn {}
impl BlackjackTableState for GameOver {}

#[derive(Debug)]
pub enum BlackjackRoundError {
    DeckEmpty
}

/// The blackjack table, containing all state necessary to keep
/// track of an in-progress game
/// 
/// The game has a deck, from which cards are pulled,
/// a hand for the dealer,
/// and a hand for the player.
/// 
/// The state of the game is also tracked, so that actions
/// can only be taken during the appropriate game state
pub struct BlackjackTable<S: BlackjackTableState = NotStarted> {
    deck: Deck,
    dealer: Hand,
    player: Hand,
    game_state: S
}

pub enum GameStartResult {
    Natural(BlackjackTable<GameOver>),
    Normal(BlackjackTable<PlayerTurn>)
}

impl BlackjackTable<NotStarted> {


    /// Start the game by dealing cards from the deck
    /// into the dealer's and player's hand.
    /// 
    /// The game can end immediately if the player, dealer, or both
    /// draw a natural
    pub fn deal(mut self) -> Result<GameStartResult, BlackjackRoundError> {

        for _ in 0..2 {
            self.player.push(
                self.deck.draw()
                .ok_or(BlackjackRoundError::DeckEmpty)?
            );

            self.dealer.push(
                self.deck.draw()
                .ok_or(BlackjackRoundError::DeckEmpty)?
            );

        }

        let player_score = self.player.total_value();
        let dealer_score = self.dealer.total_value();

        if player_score >= 21 && dealer_score >= 21 {
            Ok(GameStartResult::Natural(
                BlackjackTable::new(
                    self.deck, 
                    self.player,
                    self.dealer
                )
                .with_result(GameResult::StandOff)
            ))
        } else if player_score >= 21 {
            Ok(GameStartResult::Natural(
                BlackjackTable::new(
                    self.deck, 
                    self.player,
                    self.dealer
                )
                .with_result(GameResult::PlayerWin)
            ))
        } else if dealer_score >= 21 {
            Ok(GameStartResult::Natural(
                BlackjackTable::new(
                    self.deck, 
                    self.player,
                    self.dealer
                )
                .with_result(GameResult::DealerWin)
            ))
        } else {
            Ok(GameStartResult::Normal(
                BlackjackTable::new(
                    self.deck, 
                    self.player,
                    self.dealer
                )
            ))
        }

    }
}

pub enum PlayerTurnResult {
    Hit(BlackjackTable<PlayerTurn>),
    Bust(BlackjackTable<GameOver>)
}

impl BlackjackTable<PlayerTurn> {

    /// Draw a card into the player's hand
    /// 
    /// The game can end immediately if the player goes bust.
    /// In this case, function returns a [PlayerTurnResult::Bust],
    /// which ends the game.
    /// 
    /// Otherwise, returns a [PlayerTurnResult::Hit]
    pub fn hit(mut self) -> Result<PlayerTurnResult, BlackjackRoundError> {

        self.player.push(
            self.deck.draw()
            .ok_or(BlackjackRoundError::DeckEmpty)?
        );


        if self.player.total_value() > 21 {
            Ok(PlayerTurnResult::Bust(
                BlackjackTable::new(
                    self.deck,
                    self.player,
                    self.dealer
                )
                .with_result(GameResult::PlayerBust)
            ))
        } else {
            Ok(PlayerTurnResult::Hit(
                BlackjackTable::new(
                    self.deck,
                    self.player,
                    self.dealer
                )
            ))
        }
    }

    /// End the player's turn and start the dealer's turn
    pub fn stand(self) -> BlackjackTable<DealerTurn> {

        BlackjackTable::new(
            self.deck,
            self.player,
            self.dealer
        )
    }
}

pub enum DealerTurnResult {
    Hit(BlackjackTable<DealerTurn>),
    Stand(BlackjackTable<GameOver>)
}

impl BlackjackTable<DealerTurn> {

    /// Draw a card into the dealer's hand
    /// 
    /// The game can end immediately if the dealer goes bust.
    /// In this case, function returns a [DealerTurnResult::Stand],
    /// which ends the game.
    /// 
    /// Otherwise, returns a [PlayerTurnResult::Hit]
    pub fn hit(mut self) -> Result<DealerTurnResult, BlackjackRoundError> {

        self.dealer.push(
            self.deck.draw()
            .ok_or(BlackjackRoundError::DeckEmpty)?
        );

        let resulting_value = self.dealer.total_value();

        if resulting_value < 17 {
            Ok(DealerTurnResult::Hit(
                BlackjackTable::new(
                    self.deck,
                    self.player,
                    self.dealer
                )
            ))
        } else {
            Ok(DealerTurnResult::Stand(self.stand()))
        }
    }

    /// End the Dealer's turn immediately, ending the game
    pub fn stand(self) -> BlackjackTable<GameOver> {
        
        let dealer_value = self.dealer.total_value();

        if dealer_value > 21 {
            BlackjackTable::new(
                self.deck,
                self.player,
                self.dealer
            )
            .with_result(GameResult::DealerBust)
        } else if dealer_value > self.player.total_value() {
            BlackjackTable::new(
                self.deck,
                self.player,
                self.dealer
            )
            .with_result(GameResult::DealerWin)
        } else {
            BlackjackTable::new(
                self.deck,
                self.player,
                self.dealer
            )
            .with_result(GameResult::PlayerWin)
        }
    }
}

impl BlackjackTable<GameOver> {

    /// Returns a reference to the [GameResult] enum
    /// stored in the game's state
    pub fn result(&self) -> &GameResult {
        &self.game_state.0
    }

    /// Set the result of the finished game to the
    /// supplied [GameResult] enum
    pub fn with_result(self, res: GameResult) -> Self {
        Self {
            deck: self.deck,
            dealer: self.dealer,
            player: self.player,
            game_state: GameOver(res)
        }
    }
}

impl<S: BlackjackTableState> BlackjackTable<S> {
    pub fn new(deck: Deck, player: Hand, dealer: Hand) -> Self {
        Self { deck, dealer, player, game_state: S::default() }
    }

    /// Shuffle the deck so that drawn cards are random
    pub fn shuffle(mut self) -> Self {
        self.deck.shuffle();

        Self::new(self.deck, self.player, self.dealer)
    }

    /// Returns an optional reference to the first
    /// [Card] in the dealer's hand, i.e. the card
    /// that is visible to players during their turn
    /// 
    /// Should only return [None] when the dealer's hand
    /// is empty, i.e. before the game has been dealt
    pub fn showing_card(&self) -> Option<&Card> {
        self.dealer.cards().get(0)
    }

    /// Returns a reference to the player's [Hand]
    pub fn player_hand(&self) -> &Hand {
        &self.player
    }

    /// Returns a reference to the dealer's [Hand]
    pub fn dealer_hand(&self) -> &Hand {
        &self.dealer
    }
}

impl Default for BlackjackTable {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default()
        )
    }
}