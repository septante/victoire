use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

use dyn_clonable::clonable;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::types::Player;

#[clonable]
#[allow(unused_variables)]
#[typetag::serde(tag = "card")]
pub trait Card: Clone + Send + Sync {
    /// Name of the card
    fn name(&self) -> &str;
    /// The card's types
    fn types(&self) -> Vec<CardType>;
    /// A description of the card's effects
    fn description(&self) -> &str {
        ""
    }
    /// What does this card cost?
    fn cost(&self) -> Cost;
    /// If this card is a treasure card, what is it worth?
    fn treasure_value(&self) -> Value {
        Value::default()
    }
    /// The number of points the card is worth (if it is a victory/curse card)
    fn victory_points(&self, player: &Player) -> isize {
        0
    }

    /// Effects when this card is played
    fn effects_on_play(&self) {}

    /// Print out the card's types
    fn print_types(&self) -> String {
        format!("{}", self.types().iter().format(", "))
    }
    /// Check if this card is an Action
    fn is_action(&self) -> bool {
        self.types().contains(&CardType::Action)
    }
    /// Check if this card is an Attack
    fn is_attack(&self) -> bool {
        self.types().contains(&CardType::Attack)
    }
    /// Check if this card is a Reaction
    fn is_reaction(&self) -> bool {
        self.types().contains(&CardType::Reaction)
    }
    /// Check if this card is a Treasure
    fn is_treasure(&self) -> bool {
        self.types().contains(&CardType::Treasure)
    }
    /// Check if this card is a Victory card
    fn is_victory(&self) -> bool {
        self.types().contains(&CardType::Victory)
    }
    /// Check if this card is a Curse card
    fn is_curse(&self) -> bool {
        self.types().contains(&CardType::Curse)
    }
}

impl fmt::Display for dyn Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for dyn Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Hash for dyn Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().to_lowercase().hash(state);
    }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &Self) -> bool {
        self.name().to_lowercase().eq(&other.name().to_lowercase())
    }
}

impl Eq for dyn Card {}

impl Ord for dyn Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().to_lowercase().cmp(&other.name().to_lowercase())
    }
}

impl PartialOrd for dyn Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum CardType {
    Treasure,
    Victory,
    Curse,
    Action,
    Attack,
    Reaction,
}

impl Display for CardType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Cost {
    pub coins: usize,
}

#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Value {
    pub coins: usize,
}
