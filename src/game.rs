use crate::board::Board;
use crate::castles::Castles;
use crate::player::Player::*;
use crate::state::State;

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
