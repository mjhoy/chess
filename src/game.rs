pub mod board;
pub mod castles;
pub mod m0ve;
pub mod move_description;
pub mod piece;
pub mod player;
pub mod pos;
pub mod state;

use self::board::Board;
use self::castles::Castles;
use self::player::Player::*;
use self::state::State;

pub struct Game {
    pub state: State,
}

impl Default for Game {
    fn default() -> Self {
        let board = Board::initial();
        let player = White;
        let state = State {
            board,
            player,
            en_passant: None,
            castling: Castles::initial(),
        };
        Game { state }
    }
}

impl Game {
    pub fn with_state(state: State) -> Game {
        Game { state }
    }
}
