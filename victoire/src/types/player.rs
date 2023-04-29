use serde::{Deserialize, Serialize};

use crate::types::{CardDeck, CardList};

#[non_exhaustive]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Player {
    pub hand: CardList,
    pub deck: CardDeck,
    pub resources: Resources,
}

#[non_exhaustive]
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resources {
    pub actions: usize,
    pub buys: usize,
    pub coins: usize,
}
