use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

use Player::*;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            White => write!(f, "White"),
            Black => write!(f, "Black"),
        }
    }
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            White => Black,
            Black => White,
        }
    }
}
