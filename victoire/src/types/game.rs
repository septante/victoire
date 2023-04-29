use serde::{Deserialize, Serialize};

use victoire_macros::card_vec;

use crate::{
    callbacks::Callbacks,
    cards::{base::*, dominion::*},
    error::{Error::*, Result},
    types::{Card, CardDeck, CardList, Player, PlayerList, Supply},
};

/// The data for a game of Dominion.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub started: bool,
    pub current_turn: usize,
    pub players: PlayerList,
    pub supply: Supply,
    pub trash: CardDeck,
    pub extras: Supply,
}

impl Default for Game {
    fn default() -> Self {
        let mut game = Game::new();
        for i in 0..2 {
            let player = Player::new_with_default_deck(i);
            game.add_player(player);
        }

        game.create_supply(Game::default_supply_list()).unwrap();

        game
    }
}

impl Game {
    pub fn default_supply_list() -> CardList {
        card_vec![Cellar, Market, Merchant, Militia, Mine, Moat, Remodel, Smithy, Village, Workshop]
    }

    /// Generates the supply piles for a game given a list of cards to use
    pub fn create_supply(&mut self, cards: CardList) -> Result<()> {
        let player_count = self.player_count();

        let (victory_card_count, province_count, curse_count) = match player_count {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => return Err(NotEnoughPlayers),
        };

        let mut supply: Supply = Supply::default();
        supply.insert(Copper, 40);
        supply.insert(Silver, 40);
        supply.insert(Gold, 40);

        supply.insert(Estate, victory_card_count);
        supply.insert(Duchy, victory_card_count);
        supply.insert(Province, province_count);
        supply.insert(BasicCurse, curse_count);

        for card in cards {
            // If card is victory card, count matches other victory cards
            // Otherwise use 10 copies
            let count = if card.is_victory() {
                victory_card_count
            } else {
                10
            };

            supply.insert_boxed(card, count);
        }

        self.supply = supply;

        Ok(())
    }

    /// Create an empty game
    pub fn new() -> Game {
        let started = false;
        let current_turn = 0;
        let players = PlayerList::new();
        let supply = Supply::default();
        let trash = CardDeck::new();
        let extras = Supply::default();
        Game {
            started,
            current_turn,
            players,
            supply,
            trash,
            extras,
        }
    }

    /// Returns the number of players in the game
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Add a player to the game
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    /// Gain a copy of a card to the discard pile
    pub fn gain(
        &mut self,
        player_index: usize,
        card: Box<dyn Card>,
        callbacks: &dyn Callbacks,
    ) -> Result<()> {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile { card });
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.discard.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to hand
    pub fn gain_to_hand(
        &mut self,
        player_index: usize,
        card: Box<dyn Card>,
        callbacks: &dyn Callbacks,
    ) -> Result<()> {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(EmptyPile { card });
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.hand.push_back(card);
        Ok(())
    }
}
