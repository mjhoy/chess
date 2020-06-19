pub mod game;
pub mod parsing;
pub mod util;

use crate::game::Game;

/// Initial game.
pub fn new_game() -> Game {
    Game::default()
}
