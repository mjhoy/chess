pub mod board;
pub mod castles;
pub mod from_to_step;
pub mod game;
pub mod m0ve;
pub mod move_description;
pub mod parsing;
pub mod piece;
pub mod player;
pub mod pos;
pub mod state;

use crate::game::Game;

/// Initial game.
pub fn new_game() -> Game {
    Game::default()
}
