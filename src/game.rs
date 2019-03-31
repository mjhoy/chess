use crate::board::Board;
use crate::player::Player::*;
use crate::state::State;

#[derive(Clone)]
pub struct Game {
    pub state: State,
}

impl Game {
    pub fn initial() -> Game {
        let board = Board::initial();
        let player = White;
        let state = State { board, player };
        Game { state }
    }
}
