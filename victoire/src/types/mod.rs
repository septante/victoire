pub mod card;
pub mod game;
pub mod player;

use crate::{
    cards::base::*,
    error::{Error::*, Result},
};

pub use self::{
    card::{Card, CardType},
    game::Game,
    player::Player,
};
use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

pub type CardList = Vec<Box<dyn Card>>;
pub type CardDeck = VecDeque<Box<dyn Card>>;
pub type PlayerList = Vec<Player>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Supply {
    entries: HashMap<String, SupplyEntry>,
}

impl Supply {
    /// Generates the supply piles for a game given a list of cards to use
    pub fn new(&mut self, player_count: usize, cards: CardList) -> Result<Supply> {
        let (victory_card_count, province_count, curse_count) = match player_count {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => return Err(NotEnoughPlayers),
        };

        let mut supply: Supply = Supply::default();
        supply.insert(Copper, 40);
        supply.insert(Silver, 40);
        supply.insert(Gold, 40);

        supply.insert(Estate, victory_card_count);
        supply.insert(Duchy, victory_card_count);
        supply.insert(Province, province_count);
        supply.insert(BasicCurse, curse_count);

        for card in cards {
            // If card is victory card, count matches other victory cards
            // Otherwise use 10 copies
            let count = if card.is_victory() {
                victory_card_count
            } else {
                10
            };

            supply.insert_boxed(card, count);
        }

        Ok(supply)
    }

    pub fn insert(&mut self, card: impl Card + 'static, count: usize) {
        self.entries.insert(
            card.name().to_string(),
            SupplyEntry {
                card: Box::new(card),
                count,
            },
        );
    }

    pub fn insert_boxed(&mut self, card: Box<dyn Card>, count: usize) {
        self.entries
            .insert(card.name().to_string(), SupplyEntry { card, count });
    }
}

impl AsRef<HashMap<String, SupplyEntry>> for Supply {
    fn as_ref(&self) -> &HashMap<String, SupplyEntry> {
        &self.entries
    }
}

impl AsMut<HashMap<String, SupplyEntry>> for Supply {
    fn as_mut(&mut self) -> &mut HashMap<String, SupplyEntry> {
        &mut self.entries
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplyEntry {
    pub card: Box<dyn Card>,
    pub count: usize,
}
