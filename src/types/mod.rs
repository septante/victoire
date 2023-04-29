pub mod card;
pub mod game;
pub mod player;

use self::card::Card;

pub type CardList = Vec<Box<dyn Card>>;
