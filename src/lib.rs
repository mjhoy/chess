extern crate nalgebra as na;
#[macro_use]
extern crate itertools;

pub mod player;
use player::Player;
use player::Player::*;

pub mod piece;
use piece::Piece;

pub mod square;
use square::Square;

pub mod pos;
use pos::Pos;

pub mod board;
use board::Board;

pub mod state;
use state::State;

pub mod game;
use game::Game;

pub mod m0ve;

/// Initial game.
pub fn new_game() -> Game {
    let board = Board::initial();
    let player = White;
    let state = State { board, player };
    Game { state }
}

#[cfg(test)]
mod test {
    use *;

    #[test]
    fn test_new_game_starts_white() {
        let game = new_game();
        assert_eq!(game.state.player, White);
    }
}
