#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

use Player::*;

impl Player {
    pub fn other(self) -> Player {
        match self {
            White => Black,
            Black => White,
        }
    }
}
