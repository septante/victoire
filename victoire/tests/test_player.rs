//! Tests for player-related methods and structs

use victoire::callbacks::TestClient;
use victoire::cards::dominion::*;
use victoire::types::{Game, Player};

#[test]
fn test_player_init() {
    let player = Player::new_with_default_deck(0);

    //we check to see if everything is initialized correctly
    assert!(player.hand.len() == 5);
    assert!(player.deck.len() == 5);
    assert!(player.resources.actions == 0);
    //player.print_deck(); --> should be shuffled correctly
}

#[test]
fn test_player_draw() {
    let mut player = Player::new_with_default_deck(0);
    //draw 5, make sure everything checks out
    player.draw_cards(5);
    assert!(player.hand.len() == 10 && player.deck.is_empty());

    //test unreasonable draw
    player.draw_cards(5);
    assert!(player.hand.len() == 10 && player.deck.is_empty());

    player = Player::new_with_default_deck(0);
    player.draw_cards(14);
    assert!(player.hand.len() == 10 && player.deck.is_empty());
}

#[test]
fn test_player_discard() {
    let mut player = Player::new_with_default_deck(0);
    let first_vec = vec![0, 2, 4];
    player.discard_given_indexes(first_vec);
    assert!(player.hand.len() == 2 && player.discard.len() == 3);

    let second_vec = vec![0];
    player.discard_given_indexes(second_vec);
    assert!(player.hand.len() == 1 && player.discard.len() == 4);

    let third_vec = vec![0];
    player.discard_given_indexes(third_vec);
    assert!(player.hand.is_empty() && player.discard.len() == 5);

    let fourth_vec = vec![0];
    player.discard_given_indexes(fourth_vec);
    assert!(player.hand.is_empty() && player.discard.len() == 5);
}

#[test]
fn test_player_trash() {
    let mut player = Player::new_with_default_deck(0);
    let mut game = Game::default();
    player.trash_given_indexes(vec![0, 1, 2, 3], &mut game.trash);
    assert!(player.hand.len() == 1 && game.trash.len() == 4 && player.discard.is_empty());

    player.trash_given_indexes(vec![0], &mut game.trash);
    assert!(player.hand.is_empty() && game.trash.len() == 5 && player.discard.is_empty());

    player.trash_given_indexes(vec![0], &mut game.trash);
    assert!(player.hand.is_empty() && game.trash.len() == 5 && player.discard.is_empty());
}

#[test]
fn test_player_play_action() {
    let mut game = Game::default();
    let callbacks = TestClient;

    let _ = game.gain_to_hand(0, Box::new(Market), &callbacks);
    let player1 = &mut game.players[0];
    let temp_coins_b4add = player1.resources.temp_coins;
    player1.resources.actions = 1;
    game.play_action_from_hand(0, 5, &callbacks).unwrap();

    let player1 = &game.players[0];
    println!(
        "actions: {}, buys: {}, hand size: {}",
        player1.resources.actions,
        player1.resources.buys,
        player1.hand.len()
    );
    assert!(player1.resources.actions == 1);
    assert!(player1.resources.buys == 1);
    assert!(player1.hand.len() == 6);
    assert!(player1.resources.temp_coins == temp_coins_b4add + 1);
}

#[test]
fn test_player_buy() {}

#[test]
fn test_player_gain() {}
