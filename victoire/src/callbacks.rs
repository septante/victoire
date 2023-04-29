//! The callbacks that need to be provided when building a client

use crate::types::Card;

use dyn_clonable::clonable;

#[clonable]
/// Trait for getting input from players while card effects are occuring
pub trait Callbacks: Clone + Send + Sync {
    /// Prompt the given player for a card from the supply
    fn choose_card_from_supply(&self, player_number: usize) -> Option<Box<dyn Card>>;
    /// Prompt the player for a card in their hand
    fn choose_card_from_hand(&self, message: &str) -> usize;
    /// Prompt the player for a card from the supply, or nothing
    fn choose_card_from_hand_opt(&self, message: &str) -> Option<usize>;
    /// Prompt the player for multiple cards from their hand
    fn choose_cards_from_hand(&self, count: usize, message: &str) -> Vec<usize>;
    /// Prompt the given player with a yes/no question
    fn get_player_consent(&self, player_number: usize, prompt: &str) -> bool;
}
