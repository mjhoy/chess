pub mod player;
use crate::player::Player;
use crate::player::Player::*;

pub mod piece;
use crate::piece::Piece;

pub mod square;
use crate::square::Square;

pub mod pos;
use crate::pos::Pos;

pub mod board;
use crate::board::Board;

pub mod state;
use crate::state::State;

pub mod game;
use crate::game::Game;

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
    use super::*;

    #[test]
    fn test_new_game_starts_white() {
        let game = new_game();
        assert_eq!(game.state.player, White);
    }
}
