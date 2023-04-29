pub mod card;
pub mod game;
pub mod player;

pub use self::{card::Card, game::Game, player::Player};
use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

pub type CardList = Vec<Box<dyn Card>>;
pub type CardDeck = VecDeque<Box<dyn Card>>;
pub type PlayerList = Vec<Player>;
pub type Supply = HashMap<String, SupplyEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplyEntry {
    pub card: Box<dyn Card>,
    pub count: usize,
}
