pub mod card;
pub mod game;
pub mod player;

pub use self::{card::Card, game::Game, player::Player};
use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

pub type CardList = Vec<Box<dyn Card>>;
pub type CardDeck = VecDeque<Box<dyn Card>>;
pub type PlayerList = Vec<Player>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Supply {
    entries: HashMap<String, SupplyEntry>,
}

impl Supply {
    pub fn insert(&mut self, card: impl Card + 'static, count: usize) {
        self.entries.insert(
            card.name().to_string(),
            SupplyEntry {
                card: Box::new(card),
                count,
            },
        );
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
