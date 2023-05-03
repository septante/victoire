//! Cards from the original Dominion set (2nd edition)
// TODO: provide brief documentation on all effects for each card just for convenience
// TODO: Add description fn for cards that have it

#![allow(clippy::wildcard_imports)]

use crate::callbacks::ChoiceCountOptions;

use super::base::*;
use super::prelude::*;

declare_card!(
    Artisan,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Artisan)"
);
#[typetag::serde]
impl Card for Artisan {
    name!("Artisan");
    card_cost!(6);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let card = callbacks
            .choose_card_from_supply(player_index, &game.supply)
            .unwrap();
        let result = game.gain_to_hand(player_index, card, callbacks);
        if result.is_err() {
            // TODO: get new card
        }
        let card_index = callbacks.choose_cards_from_hand(
            &ChoiceCountOptions::Exact { count: 1 },
            "Choose a card to place on top of your deck",
        )[0];
        let card = game.remove_from_hand(player_index, card_index);
        let player = game.get_player_mut(player_index).unwrap();
        player.deck.push_front(card);
    }
}

declare_card!(
    Bandit,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Bandit)"
);
#[typetag::serde]
impl Card for Bandit {
    name!("Bandit");
    card_cost!(5);
    types!(vec![Action, Attack]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let _ = game.gain(player_index, Box::new(Gold), callbacks);
    }

    fn attack_target(&self) -> Option<AttackTarget> {
        Some(EveryoneElse)
    }

    fn attack_effects(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let mut cards = game.reveal(player_index, 2);
        let trash_choices: Vec<usize> = (0..2)
            .filter(|i| cards[*i].is_treasure() && cards[*i].name() != "Copper")
            .collect();
        match trash_choices.len() {
            1 => {
                cards.remove(trash_choices[0]);
            }
            2 => {
                let index =
                    callbacks.choose_cards_from_selection(1, &cards, "Choose a card to trash.")[0];
                cards.remove(index);
            }
            _ => {}
        }

        let player = game.get_player_mut(player_index).unwrap();
        for card in cards {
            player.discard.push_back(card);
        }
    }
}

declare_card!(
    Bureaucrat,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Bureaucrat)"
);
placeholder_effects!(Bureaucrat, "Bureaucrat", 4);

// Cellar
// +1 Action, discard any number of cards, then draw that many
declare_card!(
    Cellar,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Cellar)"
);
#[typetag::serde]
impl Card for Cellar {
    name!("Cellar");
    card_cost!(2);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut game.players[player_index];
        let indexes: Vec<usize> = callbacks.choose_cards_from_hand(
            &ChoiceCountOptions::UpTo {
                max: player.hand.len(),
            },
            "Choose cards to discard",
        );
        let count = indexes.len();

        player.discard_given_indexes(indexes);
        player.draw_cards(count);
    }
}

declare_card!(
    Chapel,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Chapel)"
);
#[typetag::serde]
impl Card for Chapel {
    name!("Chapel");
    card_cost!(2);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut game.players[player_index];
        let indexes: Vec<usize> = callbacks.choose_cards_from_hand(
            &ChoiceCountOptions::UpTo { max: 4 },
            "Choose up to 4 cards to trash",
        );
        player.trash_given_indexes(indexes, &mut game.trash);
    }
}

// Council Room
// +4 cards, +1 buy, each other player draws a card
declare_card!(
    CouncilRoom,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Council_Room)"
);
#[typetag::serde]
impl Card for CouncilRoom {
    name!("Council Room");
    card_cost!(5);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _callbacks: &dyn Callbacks) {
        let player = &mut game.players[player_index];
        player.draw_cards(4);
        player.add_buys(1);

        let player_count = game.players.len();

        for i in 1..player_count {
            let index = (i + player_index) % player_count;
            let player = &mut game.players[index];
            player.draw_cards(1);
        }
    }
}

basic_action!(
    Festival,
    "Festival",
    cost = 5,
    cards = 0,
    actions = 2,
    buys = 1,
    coins = 2,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Festival)"
);

// Gardens
//
// Effect: victory card, worth 1 per 10 cards you have(round down)
declare_card!(
    Gardens,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Gardens)"
);
#[typetag::serde]
impl Card for Gardens {
    name!("Gardens");
    card_cost!(4);
    types!(vec![Victory]);

    //integer division should be fine
    fn victory_points(&self, player: &Player) -> isize {
        ((player.deck.len() + player.hand.len() + player.discard.len()) / 10) as isize
    }
}

declare_card!(
    Harbinger,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Harbinger)"
);
#[typetag::serde]
impl Card for Harbinger {
    name!("Harbinger");
    card_cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut game.players[player_index];
        player.add_actions(1);
        player.draw_cards(1);

        let indexes = callbacks.choose_cards_from_discard(
            &ChoiceCountOptions::Exact { count: 1 },
            "Choose a card from your discard to put onto your deck.",
        );

        player.move_given_indexes_discard_to_hand(indexes);
    }
}

declare_card!(
    Laboratory,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Laboratory)"
);
#[typetag::serde]
impl Card for Laboratory {
    name!("Laboratory");
    card_cost!(5);
    types!(vec![Action]);
    basic_on_play_effects!(cards = 2, actions = 1, buys = 0, coins = 0);
}

declare_card!(
    Library,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Library)"
);
#[typetag::serde]
impl Card for Library {
    name!("Library");
    card_cost!(5);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let player = &mut game.players[player_index];
        while player.hand.len() < 7 {
            if player.deck.front().unwrap().is_action() {
                //TODO: get player consent to draw or discard the card
                if callbacks.get_player_consent(player.player_number, "discard?") {
                    player.discard.push_back(player.deck.pop_front().unwrap());
                }
            } else {
                player.draw_cards(1);
            }
        }
    }
}

// Market
// effects: +1 Action, +1 Buy, +1 temp_coins, +1 Card
basic_action!(
    Market,
    "Market",
    cost = 5,
    cards = 1,
    actions = 1,
    buys = 1,
    coins = 1,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Market)"
);

// Merchant
declare_card!(
    Merchant,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Merchant)"
);
#[typetag::serde]
impl Card for Merchant {
    name!("Merchant");
    card_cost!(3);
    types!(vec![Action]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _: &dyn Callbacks) {
        let p = game.players.get_mut(player_index).unwrap();
        p.add_actions(1);
        p.draw_cards(1);

        //TODO: add method on game
    }
}

declare_card!(
    Militia,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Militia)"
);
placeholder_effects!(Militia, "Militia", 4);

declare_card!(
    Mine,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Mine)"
);
placeholder_effects!(Mine, "Mine", 5);

declare_card!(
    Moat,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Moat)"
);
#[typetag::serde]
impl Card for Moat {
    name!("Moat");
    card_cost!(2);
    types!(vec![Action, Reaction]);
    fn effects_on_play(&self, game: &mut Game, player_index: usize, _callbacks: &dyn Callbacks) {
        let p = game.players.get_mut(player_index).unwrap();
        p.draw_cards(2);
    }

    fn reaction_effects(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        // TODO: Fix this to make it a choice per attack, rather than making
        // the player completely immune until their next turn
        if callbacks.get_player_consent(player_index, "Use moat?") {
            let p = game.players.get_mut(player_index).unwrap();
            p.state.immune = true;
        }
    }

    fn reaction_trigger(&self) -> Option<ReactionTrigger> {
        Some(OtherPlayerPlaysAttack)
    }
}

declare_card!(
    Moneylender,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Moneylender)"
);
placeholder_effects!(Moneylender, "Moneylender", 4);

declare_card!(
    Poacher,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Poacher)"
);
placeholder_effects!(Poacher, "Poacher", 4);

declare_card!(
    Remodel,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Remodel)"
);
placeholder_effects!(Remodel, "Remodel", 4);

declare_card!(
    Sentry,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Sentry)"
);
placeholder_effects!(Sentry, "Sentry", 5);

basic_action!(
    Smithy,
    "Smithy",
    cost = 4,
    cards = 3,
    actions = 0,
    buys = 0,
    coins = 0,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Smithy)"
);

declare_card!(
    ThroneRoom,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Throne_Room)"
);
#[typetag::serde]
impl Card for ThroneRoom {
    name!("Throne Room");
    card_cost!(4);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let card_index = callbacks.choose_cards_from_hand(
            &ChoiceCountOptions::UpTo { max: 1 },
            "Choose card to play twice",
        )[0];

        let player = &mut game.players[player_index];
        let mut card = player.hand.remove(card_index).unwrap();

        while !card.is_action() {
            let card_index = callbacks.choose_cards_from_hand(
                &ChoiceCountOptions::UpTo { max: 1 },
                "Choose card to play twice",
            )[0];

            card = player.hand.remove(card_index).unwrap();
        }

        game.action_effects(player_index, &*card, callbacks);
        game.action_effects(player_index, &*card, callbacks);

        let player = &mut game.players[player_index];
        player.in_play.push_back(card);
    }
}

declare_card!(
    Vassal,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Vassal)"
);
placeholder_effects!(Vassal, "Vassal", 3);

basic_action!(
    Village,
    "Village",
    cost = 3,
    cards = 1,
    actions = 2,
    buys = 0,
    coins = 0,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Village)"
);

declare_card!(
    Witch,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Witch)"
);
#[typetag::serde]
impl Card for Witch {
    name!("Witch");
    card_cost!(5);
    types!(vec![Action, Attack]);
    basic_on_play_effects!(cards = 2, actions = 0, buys = 0, coins = 0);

    fn attack_target(&self) -> Option<AttackTarget> {
        Some(EveryoneElse)
    }

    fn attack_effects(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        let _ = game.gain(player_index, Box::new(BasicCurse), callbacks);
    }
}

declare_card!(
    Workshop,
    "[Wiki link](http://wiki.dominionstrategy.com/index.php/Workshop)"
);
#[typetag::serde]
impl Card for Workshop {
    name!("Workshop");
    card_cost!(3);
    types!(vec![Action]);

    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &dyn Callbacks) {
        todo!();
    }
}
