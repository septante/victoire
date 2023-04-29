use crate::types::PlayerList;

#[derive(Clone, Debug)]
pub struct Game {
    pub started: bool,
    pub current_turn: usize,
    pub players: PlayerList,
}
