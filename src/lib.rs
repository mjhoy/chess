pub mod board;
pub mod game;
pub mod m0ve;
pub mod move_description;
pub mod piece;
pub mod player;
pub mod pos;
pub mod square;
pub mod state;

use crate::{
    board::Board, game::Game, piece::Piece, player::Player, player::Player::*, pos::Pos,
    square::Square, state::State,
};

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
