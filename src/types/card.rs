pub trait Card: Clone + Send + Sync {
    fn name(&self) -> &str;
    fn types(&self) -> Vec<CardType>;
    fn description(&self) -> &str {
        ""
    }
    fn cost(&self) -> Cost;
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
