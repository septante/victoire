pub mod card;
pub mod game;
pub mod player;

pub use self::{
    card::{Card, CardType},
    game::{Game, PartialGame},
    player::{Phase, Player},
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

    pub fn get(&self, k: &str) -> Option<&SupplyEntry> {
        self.entries.get(k)
    }

    pub fn get_mut(&mut self, k: &str) -> Option<&mut SupplyEntry> {
        self.entries.get_mut(k)
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

impl From<HashMap<String, SupplyEntry>> for Supply {
    fn from(value: HashMap<String, SupplyEntry>) -> Self {
        Supply { entries: value }
    }
}

impl From<Supply> for HashMap<String, SupplyEntry> {
    fn from(value: Supply) -> Self {
        value.entries
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplyEntry {
    pub card: Box<dyn Card>,
    pub count: usize,
}
