use super::*;
use std::{thread, time};

#[test]
fn test_player_up() {
    let mut game = Game::new();
    assert_eq!(game.player_up().is_ok(), true);
    assert_eq!(game.player_y_pos, 32);
    assert_eq!(game.board[31][63], Entity::Empty);
    assert_eq!(game.board[32][63], Entity::Player);
}

#[test]
fn test_player_down() {
    let mut game = Game::new();
    assert_eq!(game.player_down().is_ok(), true);
    assert_eq!(game.player_y_pos, 30);
    assert_eq!(game.board[30][63], Entity::Player);
    assert_eq!(game.board[31][63], Entity::Empty);
}

#[test]
fn test_player_up_fail() {
    let mut game = Game::new();
    for _ in 0..32 {
        assert_eq!(game.player_up().is_ok(), true);
    }

    assert_eq!(game.player_up().is_ok(), false);
    assert_eq!(game.player_y_pos, 63);
    assert_eq!(game.board[63][63], Entity::Player);
}

#[test]
fn test_player_down_fail() {
    let mut game = Game::new();
    for _ in 0..30 {
        assert_eq!(game.player_down().is_ok(), true);
    }

    assert_eq!(game.player_down().is_ok(), false);
    assert_eq!(game.player_y_pos, 1);
    game.display();
    assert_eq!(game.board[1][63], Entity::Player);
}

#[test]
fn test_ball() {
    let mut game = Game::new();

    for _ in 0..32 {
        game.move_ball();

        let ten_millis = time::Duration::from_millis(10);

        thread::sleep(ten_millis);

        game.display();
    }
}
