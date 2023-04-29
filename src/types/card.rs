pub trait Card: Clone + Send + Sync {
    fn name(&self) -> &str;
    fn types(&self) -> Vec<CardType>;
    fn description(&self) -> &str {
        ""
    }
    fn cost(&self) -> Cost;
}

pub enum CardType {
    Treasure,
    Victory,
    Curse,
    Action,
    Attack,
    Reaction,
}

pub struct Cost {
    pub coins: usize,
}
