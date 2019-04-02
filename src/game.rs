use crate::board::Board;
use crate::player::Player::*;
use crate::state::State;

pub struct Game {
    pub state: State,
}

impl Default for Game {
    fn default() -> Self {
        let board = Board::initial();
        let player = White;
        let state = State { board, player };
        Game { state }
    }
}
