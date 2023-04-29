use serde::{Deserialize, Serialize};

use crate::types::PlayerList;

use super::Supply;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub started: bool,
    pub current_turn: usize,
    pub players: PlayerList,
    pub supply: Supply,
}
