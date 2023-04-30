use std::collections::VecDeque;
use std::mem;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cards::base::{Copper, Estate};
use crate::types::{CardDeck, CardList};
use crate::utils;
use victoire_macros::card_vec;

/// Struct representing a player
#[non_exhaustive]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Player {
    pub uuid: Uuid,
    pub player_number: usize,
    pub hand: CardDeck,
    pub deck: CardDeck,
    pub discard: CardDeck,
    pub in_play: CardDeck,
    pub resources: Resources,
    pub state: State,
    pub phase: Phase,
}

impl Player {
    /// Constructs a new Player with the default deck (3 estates and 7 copper)
    pub fn new_with_default_deck(player_number: usize) -> Player {
        let deck = card_vec![
            Copper, Copper, Copper, Copper, Copper, Copper, Copper, Estate, Estate, Estate
        ];
        Player::new(player_number, deck)
    }

    /// Constructs a new Player with a given deck
    pub fn new(player_number: usize, cards: CardList) -> Player {
        let uuid = Uuid::new_v4();
        let mut hand: CardDeck = VecDeque::new();
        let mut deck: CardDeck = VecDeque::from(cards);
        let discard: CardDeck = VecDeque::new();
        let in_play: CardDeck = VecDeque::new();
        let resources = Resources::default();
        let state = State::default();
        let phase = Phase::OutOfTurn;

        utils::shuffle(&mut deck);

        // Initial hand of 5 cards
        for _ in 0..5 {
            hand.push_back(deck.pop_front().unwrap());
        }

        Player {
            uuid,
            player_number,
            hand,
            deck,
            discard,
            in_play,
            resources,
            state,
            phase,
        }
    }

    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    /// Draws x cards for the player
    pub fn draw_cards(&mut self, cards: usize) {
        for _ in 0..cards {
            // If deck is empty, shuffle discard and swap it with the empty deck
            if self.deck.is_empty() {
                // If discard is also empty, there is nothing to draw
                if self.discard.is_empty() {
                    return;
                }

                utils::shuffle(&mut self.discard);
                mem::swap(&mut self.deck, &mut self.discard);
            }

            self.hand.push_back(self.deck.pop_front().unwrap());
        }
    }

    /// Gives the player extra actions for this turn
    pub fn add_actions(&mut self, actions: usize) {
        self.resources.actions += actions;
    }

    /// Gives the player extra buys for this turn
    pub fn add_buys(&mut self, buys: usize) {
        self.resources.buys += buys;
    }

    /// Gives the player extra coins for this turn
    pub fn add_coins(&mut self, coins: usize) {
        self.resources.temp_coins += coins;
    }

    /// Discards cards from hand given an array of indexes of said cards
    ///
    /// Will panic if indexes are invalid
    pub fn discard_given_indexes(&mut self, mut indexes: Vec<usize>) {
        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            //if hand is empty, return from function
            if self.hand.is_empty() {
                return;
            }
            self.discard.push_back(self.hand.remove(i).unwrap());
        }
    }

    /// Moves cards given indexes to hand
    ///
    /// Will panic if indexes are invalid
    pub fn move_given_indexes_discard_to_hand(&mut self, indexes: Vec<usize>) {
        for i in indexes {
            if self.discard.is_empty() {
                return;
            }
            self.hand.push_back(self.discard.remove(i).unwrap());
        }
    }

    /// Trashes cards from hand given an array of indexes of said cards
    ///
    /// Will panic if indexes are invalid
    pub fn trash_given_indexes(&mut self, mut indexes: Vec<usize>, trash: &mut CardDeck) {
        indexes.sort_unstable();
        indexes.reverse();
        for i in indexes {
            //if hand is empty, return from function
            if self.hand.is_empty() {
                return;
            }
            trash.push_back(self.hand.remove(i).unwrap());
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    pub actions: usize,
    pub buys: usize,
    pub coins: usize,
    pub temp_coins: usize,
    pub coins_remaining: usize,
}

/// What phase are we in
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    OutOfTurn,
    ActionPhase,
    BuyPhase,
    CleanupPhase,
}

impl Phase {
    pub fn next(&self) -> Self {
        match self {
            Self::OutOfTurn => Self::ActionPhase,
            Self::ActionPhase => Self::BuyPhase,
            Self::BuyPhase => Self::CleanupPhase,
            Self::CleanupPhase => Self::OutOfTurn,
        }
    }
}

/// Struct to keep track of certain conditions
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    /// Is the player immune to attacks
    pub immune: bool,
    pub temp_immune: bool,
    pub merchant_bonus: usize,
}
