//! The callbacks that need to be provided when building a client

use std::io;

use crate::types::{Card, CardList, Supply};

use dyn_clonable::clonable;
use itertools::Itertools;

#[clonable]
/// Trait for getting input from players while card effects are occuring
pub trait Callbacks: Clone + Send + Sync {
    /// Prompt the given player for a card from the supply
    fn choose_card_from_supply(
        &self,
        player_index: usize,
        supply: &Supply,
    ) -> Option<Box<dyn Card>>;
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
    fn get_player_consent(&self, player_index: usize, prompt: &str) -> bool;
    /// Prompt player for one or more player indices
    fn choose_players(&self, player_index: usize, count: usize, prompt: &str) -> Vec<usize>;
}

/// An example implementation of [`Callbacks`] for testing purposes
#[derive(Clone)]
pub struct TestClient;

#[allow(unused_variables)]
impl Callbacks for TestClient {
    fn choose_card_from_supply(
        &self,
        _player_index: usize,
        supply: &Supply,
    ) -> Option<Box<dyn Card>> {
        let mut alphabetized = supply
            .as_ref()
            .values()
            .map(|entry| entry.card.clone())
            .collect_vec();
        alphabetized.sort_unstable();
        println!("Cards:");
        println!("{alphabetized:?}");

        println!("Enter index to select:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let i = input.parse::<usize>().unwrap();
        let card = alphabetized.get(i).expect("Index out of bounds");

        Some(card.clone())
    }

    fn choose_cards_from_hand(&self, count: usize, message: &str) -> Vec<usize> {
        let mut input = String::new();
        let mut output = vec![];
        let prompt = "Enter a card index from your hand, or -1 to stop:";
        println!("{prompt}");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let mut i = input.parse::<isize>().unwrap();
        let mut j = 0;
        while i >= 0 && j < count {
            output.push(i as usize);
            println!("{prompt}");
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            i = input.parse::<isize>().unwrap();
            j += 1;
        }
        output
    }

    fn choose_cards_from_discard(&self, count: usize, message: &str) -> Vec<usize> {
        todo!()
    }

    fn choose_cards_from_trash(&self, count: usize, message: &str) -> Vec<usize> {
        todo!()
    }

    fn choose_cards_from_selection(
        &self,
        count: usize,
        card_choices: &CardList,
        message: &str,
    ) -> Vec<usize> {
        todo!()
    }

    fn choose_cards_from_hand_opt(&self, count: usize, message: &str) -> Option<Vec<usize>> {
        todo!()
    }

    fn choose_cards_from_discard_opt(&self, count: usize, message: &str) -> Option<Vec<usize>> {
        todo!()
    }

    fn choose_cards_from_trash_opt(&self, count: usize, message: &str) -> Option<Vec<usize>> {
        todo!()
    }

    fn get_player_consent(&self, player_index: usize, prompt: &str) -> bool {
        let mut input = String::new();
        println!("(y)es/(n)o");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        input.to_lowercase().starts_with('y')
    }

    fn choose_players(&self, player_index: usize, count: usize, prompt: &str) -> Vec<usize> {
        todo!()
    }
}
