
use crate::{deck::Deck, hand::Hand, cards::Card};
use std::fmt::Debug;

// Types to guard certain game actions

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
    pub fn shuffle(mut self) -> Self {
        self.deck.shuffle();

        Self::new(self.deck, self.player, self.dealer)
    }

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
    Stand(BlackjackTable<DealerTurn>),
    Bust(BlackjackTable<GameOver>)
}

impl BlackjackTable<PlayerTurn> {
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

    pub fn stand(self) -> Result<PlayerTurnResult, BlackjackRoundError> {

        Ok(PlayerTurnResult::Stand(
            BlackjackTable::new(
                self.deck,
                self.player,
                self.dealer
            )
        ))
    }
}

pub enum DealerTurnResult {
    Hit(BlackjackTable<DealerTurn>),
    Stand(BlackjackTable<GameOver>)
}

impl BlackjackTable<DealerTurn> {
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
    pub fn result(&self) -> &GameResult {
        &self.game_state.0
    }

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

    pub fn showing_card(&self) -> Option<&Card> {
        self.dealer.cards().get(0)
    }

    pub fn player_hand(&self) -> &Hand {
        &self.player
    }

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