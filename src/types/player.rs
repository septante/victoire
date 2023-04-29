use crate::types::CardList;

#[non_exhaustive]
pub struct Player {
    pub hand: CardList,
    pub deck: CardList,
    pub resources: Resources,
}

#[non_exhaustive]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Resources {
    pub actions: usize,
    pub buys: usize,
    pub coins: usize,
}
