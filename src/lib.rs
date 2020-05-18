pub mod algebraic_notation;
pub mod board;
pub mod castles;
pub mod fen;
pub mod from_to_step;
pub mod game;
pub mod m0ve;
pub mod move_description;
pub mod piece;
pub mod player;
pub mod pos;
pub mod state;

use crate::game::Game;

/// Initial game.
pub fn new_game() -> Game {
    Game::default()
}
