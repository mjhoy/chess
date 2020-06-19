pub mod from_to_step;
pub mod game;
pub mod parsing;

use crate::game::Game;

/// Initial game.
pub fn new_game() -> Game {
    Game::default()
}
