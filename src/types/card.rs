use std::{
    cmp::Ordering,
    fmt::{self, Formatter},
    hash::{Hash, Hasher},
};

use dyn_clonable::clonable;

#[clonable]
#[typetag::serde(tag = "card")]
pub trait Card: Clone + Send + Sync {
    fn name(&self) -> &str;
    fn types(&self) -> Vec<CardType>;
    fn description(&self) -> &str {
        ""
    }
    fn cost(&self) -> Cost;
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CardType {
    Treasure,
    Victory,
    Curse,
    Action,
    Attack,
    Reaction,
}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Cost {
    pub coins: usize,
}
