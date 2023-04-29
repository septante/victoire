pub mod card;
pub mod game;
pub mod player;

pub use self::{card::Card, game::Game, player::Player};

pub type CardList = Vec<Box<dyn Card>>;
pub type PlayerList = Vec<Player>;
