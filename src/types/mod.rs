pub mod card;
pub mod game;
pub mod player;

use self::{card::Card, player::Player};

pub type CardList = Vec<Box<dyn Card>>;
pub type PlayerList = Vec<Player>;
