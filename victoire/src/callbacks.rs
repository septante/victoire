//! The callbacks that need to be provided when building a client

use crate::types::{Card, CardList};

use dyn_clonable::clonable;

#[clonable]
/// Trait for getting input from players while card effects are occuring
pub trait Callbacks: Clone + Send + Sync {
    /// Prompt the given player for a card from the supply
    fn choose_card_from_supply(&self, player_number: usize) -> Option<Box<dyn Card>>;
    /// Prompt the player for one or more cards from their hand
    fn choose_cards_from_hand(&self, count: usize, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from their discard
    fn choose_cards_from_discard(&self, count: usize, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from the trash
    fn choose_cards_from_trash(&self, count: usize, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from a list of options
    fn choose_cards_from_selection(
        &self,
        count: usize,
        card_choices: &CardList,
        message: &str,
    ) -> Vec<usize>;
    /// Optionally prompt the player for one or more cards from their hand
    fn choose_cards_from_hand_opt(&self, count: usize, message: &str) -> Option<Vec<usize>>;
    /// Optionally prompt the player for one or more cards cards from their discard
    fn choose_cards_from_discard_opt(&self, count: usize, message: &str) -> Option<Vec<usize>>;
    /// Optionally prompt the player for one or more cards cards from the trash
    fn choose_cards_from_trash_opt(&self, count: usize, message: &str) -> Option<Vec<usize>>;
    /// Prompt the given player with a yes/no question
    fn get_player_consent(&self, player_number: usize, prompt: &str) -> bool;
}
