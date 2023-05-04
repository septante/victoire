//! The callbacks that need to be provided when building a client

use std::io;

use crate::types::{Card, CardList, Supply};

use itertools::Itertools;

/// Trait for getting input from players while card effects are occurring
pub trait Callbacks: Send + Sync {
    /// Prompt the given player for a card from the supply
    fn choose_card_from_supply(
        &self,
        player_index: usize,
        supply: &Supply,
    ) -> Option<Box<dyn Card>>;
    /// Prompt the player for one or more cards from their hand
    fn choose_cards_from_hand(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from their discard
    fn choose_cards_from_discard(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from the trash
    fn choose_cards_from_trash(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize>;
    /// Prompt the player for one or more cards from a list of options
    fn choose_cards_from_selection(
        &self,
        count: usize,
        card_choices: &CardList,
        message: &str,
    ) -> Vec<usize>;
    /// Prompt the given player with a yes/no question
    fn yes_or_no(&self, player_index: usize, prompt: &str) -> bool;
    /// Prompt player for one or more player indices
    fn choose_players(
        &self,
        player_index: usize,
        count: &ChoiceCountOptions,
        prompt: &str,
    ) -> Vec<usize>;
}

/// How many items the player can choose
pub enum ChoiceCountOptions {
    /// The player must choose exactly `count` items from the given choices
    Exact {
        /// The number of items to choose
        count: usize,
    },
    /// The player can choose any number of items up to `max` from the given choices
    UpTo {
        /// The maximum number of items that can be chosen
        max: usize,
    },
    /// The player can choose any number of items from the given choices
    Any,
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

    fn choose_cards_from_hand(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize> {
        let mut input = String::new();
        let mut output = vec![];
        let prompt = "Enter a card index from your hand, or -1 to stop:";
        println!("{prompt}");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match count {
            ChoiceCountOptions::Exact { count } => {
                let mut i = input.parse::<isize>().unwrap();
                let mut j = 0;
                while i >= 0 && j < *count {
                    #[allow(clippy::cast_sign_loss)] // i must be >= 0 here
                    output.push(i as usize);
                    println!("{prompt}");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("error: unable to read user input");
                    i = input.parse::<isize>().unwrap();
                    j += 1;
                }
            }
            ChoiceCountOptions::UpTo { max } => {
                let mut i = input.parse::<isize>().unwrap();
                let mut j = 0;
                while i >= 0 && j < *max {
                    #[allow(clippy::cast_sign_loss)] // i must be >= 0 here
                    output.push(i as usize);
                    println!("{prompt}");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("error: unable to read user input");
                    i = input.parse::<isize>().unwrap();
                    j += 1;
                }
            }
            ChoiceCountOptions::Any => {
                let mut i = input.parse::<isize>().unwrap();
                let mut j = 0;
                while i >= 0 {
                    #[allow(clippy::cast_sign_loss)] // i must be >= 0 here
                    output.push(i as usize);
                    println!("{prompt}");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("error: unable to read user input");
                    i = input.parse::<isize>().unwrap();
                    j += 1;
                }
            }
        }

        output
    }

    fn choose_cards_from_discard(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize> {
        todo!()
    }

    fn choose_cards_from_trash(&self, count: &ChoiceCountOptions, message: &str) -> Vec<usize> {
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

    fn yes_or_no(&self, player_index: usize, prompt: &str) -> bool {
        let mut input = String::new();
        println!("(y)es/(n)o");
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        input.to_lowercase().starts_with('y')
    }

    fn choose_players(
        &self,
        player_index: usize,
        count: &ChoiceCountOptions,
        prompt: &str,
    ) -> Vec<usize> {
        todo!()
    }
}
