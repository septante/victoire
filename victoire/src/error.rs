//! Dominion error types

use crate::types::{Card, CardType};

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Clone, Debug, ThisError, Serialize, Deserialize)]
pub enum Error {
    #[error("Card doesn't have expected type! Expected: {expected:?}")]
    CardTypeMisMatch { expected: CardType },
    #[error("Pile is empty: {card:?}")]
    EmptyPile { card: Box<dyn Card> },
    #[error("Not enough resources to buy that card!")]
    InsufficientFunds,
    #[error("Not enough players to start!")]
    NotEnoughPlayers,
    #[error("Not that player's turn!")]
    OutOfTurn,
    #[error("Can't perform that action during this phase!")]
    WrongPhase,
}
