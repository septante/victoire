use serde::{Deserialize, Serialize};

use victoire_macros::card_vec;

use crate::{
    callbacks::{Callbacks, ChoiceCountOptions},
    cards::{base::*, dominion::*},
    error::{Error, Result},
    types::{
        card::{AttackTarget, ReactionTrigger},
        Card, CardDeck, CardList, CardType, Phase, Player, PlayerList, Supply,
    },
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialGame {}

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
    pub fn create_supply(&mut self, cards: CardList) -> Result {
        let player_count = self.player_count();

        let (victory_card_count, province_count, curse_count) = match player_count {
            2 => (8, 8, 10),
            3 => (12, 12, 20),
            4 => (12, 12, 30),
            5 => (12, 15, 40),
            6 => (12, 18, 50),
            _ => return Err(Error::NotEnoughPlayers),
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

    /// Generate a [`PartialGame`]
    pub fn partial_game(&self, player_number: usize) -> PartialGame {
        PartialGame {}
    }

    /// Returns the number of players in the game
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Add a player to the game
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    /// Get reference to a player given index
    pub fn get_player(&self, index: usize) -> Option<&Player> {
        self.players.get(index)
    }

    /// Get mutable reference to a player given index
    pub fn get_player_mut(&mut self, index: usize) -> Option<&mut Player> {
        self.players.get_mut(index)
    }

    /// Gain a copy of a card to the discard pile
    pub fn gain(
        &mut self,
        player_index: usize,
        card: Box<dyn Card>,
        callbacks: &dyn Callbacks,
    ) -> Result {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(Error::EmptyPile { card });
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
    ) -> Result {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(Error::EmptyPile { card });
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.hand.push_back(card);
        Ok(())
    }

    /// Gain a copy of a card to the top of the deck
    pub fn gain_to_deck_top(
        &mut self,
        player_index: usize,
        card: Box<dyn Card>,
        callbacks: &dyn Callbacks,
    ) -> Result {
        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(Error::EmptyPile { card });
        }

        self.supply.get_mut(card.name()).unwrap().count -= 1;
        card.effects_on_gain(self, player_index, callbacks);

        let player = &mut self.players[player_index];
        player.deck.push_front(card);
        Ok(())
    }

    pub fn remove_from_hand(&mut self, player_index: usize, card_index: usize) -> Box<dyn Card> {
        let player = self.get_player_mut(player_index).unwrap();
        player.hand.remove(card_index).unwrap()
    }

    pub fn reveal(&mut self, player_index: usize, count: usize) -> CardList {
        let mut cards = CardList::new();
        for _ in 0..count {
            cards.push(self.players[player_index].deck.pop_front().unwrap());
        }

        cards
    }

    /// Plays an action [card](Card) from the hand of the player corresponding
    /// to the given index
    ///
    /// This is the function to call when a player plays a card directly
    pub fn play_action_from_hand(
        &mut self,
        player_index: usize,
        card_index: usize,
        callbacks: &dyn Callbacks,
    ) -> Result {
        // Remove card from hand
        let player = &mut self.players[player_index];
        let card = player.hand.get(card_index).unwrap();
        if card.is_action() {
            let card = player.hand.remove(card_index).unwrap();
            player.in_play.push_back(card.clone());

            player.resources.actions -= 1;
            self.action_effects(player_index, &*card, callbacks);

            Ok(())
        } else {
            Err(Error::CardTypeMisMatch {
                expected: CardType::Action,
            })
        }
    }

    /// Gives the player the effects of an action card as if they had played it
    ///
    /// Does not subtract actions from the player's total. Should only be called
    /// in the effects() function of other cards (e.g. Throne Room)
    pub fn action_effects(
        &mut self,
        player_index: usize,
        card: &dyn Card,
        callbacks: &dyn Callbacks,
    ) {
        // Effects on the player who played the card
        card.effects_on_play(self, player_index, callbacks);

        // Attack effects, if any
        if card.is_attack() {
            let targets = self.get_targets(
                player_index,
                card.attack_target()
                    .expect("Card has Attack type but does not define targets!"),
                callbacks,
            );

            self.check_reactions(
                player_index,
                ReactionTrigger::OtherPlayerPlaysAttack,
                callbacks,
            );

            for index in targets {
                let player = &self.players[player_index];

                if !(player.state.immune) {
                    card.attack_effects(self, index, callbacks);
                }

                let mut player = &mut self.players[player_index];
                player.state.immune = false;
            }
        }
    }

    /// Convert the attack target type into a vec of player indices
    pub fn get_targets(
        &mut self,
        player_index: usize,
        target_type: AttackTarget,
        callbacks: &dyn Callbacks,
    ) -> Vec<usize> {
        match target_type {
            AttackTarget::EveryoneElse => {
                let mut indices = vec![];
                for i in 0..self.player_count() {
                    indices.push(i);
                }
                indices.remove(player_index);

                indices
            }

            AttackTarget::PlayerToLeft => {
                vec![player_index + 1]
            }

            AttackTarget::PlayerOfChoice => callbacks.choose_players(
                player_index,
                &ChoiceCountOptions::Exact { count: 1 },
                "Choose a player to target",
            ),
        }
    }

    pub fn check_reactions(
        &mut self,
        player_index: usize,
        reaction_trigger: ReactionTrigger,
        callbacks: &dyn Callbacks,
    ) {
        // TODO: prompt player and perform reaction
    }

    /// Plays a single treasure card from the player's hand. Returns
    /// `Err(CardTypeMisMatch { expected: Treasure })` if the card is not a
    /// treasure card
    pub fn play_treasure(
        &mut self,
        player_index: usize,
        card_index: usize,
        callbacks: &dyn Callbacks,
    ) -> Result {
        let player = &mut self.players[player_index];

        // Remove card from hand
        let c = player.hand.get(card_index).unwrap();
        if !c.is_treasure() {
            return Err(Error::CardTypeMisMatch {
                expected: CardType::Treasure,
            });
        }

        let card = player.hand.remove(card_index).unwrap();
        card.effects_on_play(self, player_index, callbacks);
        let player = &mut self.players[player_index];

        player.resources.coins += card.treasure_value().coins;

        player.in_play.push_back(card.clone());

        Ok(())
    }

    /// Play all treasure cards from the players hand
    pub fn play_all_treasures(&mut self, player_index: usize, callbacks: &dyn Callbacks) {
        let range = self.players[player_index].hand.len();

        for i in 0..range {
            let player = &mut self.players[player_index];
            let card = player.hand.get(i).unwrap();
            if card.is_treasure() {
                // We know the card is a treasure card, so unwrap
                self.play_treasure(player_index, i, callbacks).unwrap();
            }
        }
    }

    /// Buy a card
    pub fn buy_card(
        &mut self,
        player_index: usize,
        card: Box<dyn Card>,
        callbacks: &dyn Callbacks,
    ) -> Result {
        if player_index != self.current_turn {
            return Err(Error::OutOfTurn);
        }

        let player = &mut self.players[player_index];

        if player.phase != Phase::BuyPhase {
            return Err(Error::WrongPhase);
        }

        if player.resources.coins_remaining < card.cost().coins {
            return Err(Error::InsufficientFunds);
        }

        if self.supply.get(card.name()).unwrap().count == 0 {
            return Err(Error::EmptyPile { card });
        }

        card.effects_on_buy(self, player_index, callbacks);
        card.effects_on_gain(self, player_index, callbacks);

        self.gain(player_index, card.clone(), callbacks)?;

        let player = &mut self.players[player_index];
        player.resources.temp_coins -= card.cost().coins;

        player.resources.buys -= 1;

        // Hovel check
        if card.is_victory() {
            self.check_reactions(player_index, ReactionTrigger::BuyAVictoryCard, callbacks);
        }

        Ok(())
    }

    /// Take a turn
    pub fn turn(&mut self, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut self.players[player_index];

        player.reset_state();

        player.phase = Phase::ActionPhase;
        self.action_phase(player_index, callbacks);

        let player = &mut self.players[player_index];
        player.phase = Phase::BuyPhase;
        self.buy_phase(player_index, callbacks);

        let player = &mut self.players[player_index];
        player.phase = Phase::CleanupPhase;
        player.cleanup();

        player.phase = Phase::OutOfTurn;
    }

    /// Action phase
    pub fn action_phase(&mut self, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut self.players[player_index];

        if player.resources.actions > 0 {
            let mut indices = Vec::new();
            while !indices.is_empty() {
                indices = callbacks.choose_cards_from_hand(
                    &ChoiceCountOptions::UpTo { max: 1 },
                    "Choose an action card to play",
                );

                let card_index = indices[0];

                if self
                    .play_action_from_hand(player_index, card_index, callbacks)
                    .is_err()
                {
                    continue;
                }

                if self.players[player_index].resources.actions == 0 {
                    break;
                }
            }
        }

        // Do any end of action phase stuff here
    }

    /// Buy phase
    pub fn buy_phase(&mut self, player_index: usize, callbacks: &dyn Callbacks) {
        // TODO: allow player to choose which treasures they play?
        self.play_all_treasures(player_index, callbacks);

        let player = &mut self.players[player_index];
        let player_number = player.player_number;

        player.resources.coins_remaining = player.resources.coins + player.resources.temp_coins;

        if player.resources.buys > 0 {
            while let Some(card) = callbacks.choose_card_from_supply(player_number, &self.supply) {
                // If player chooses a card they cannot buy, loop
                if self
                    .buy_card(player_index, card.clone(), callbacks)
                    .is_err()
                {
                    continue;
                }

                if self.players[player_index].resources.buys == 0 {
                    break;
                }
            }
        }

        // Do any end of buy phase stuff here
    }
}
