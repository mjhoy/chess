use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

use self::Player::*;

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

    pub fn is_white(self) -> bool {
        match self {
            White => true,
            Black => false,
        }
    }

    pub fn is_black(self) -> bool {
        match self {
            White => false,
            Black => true,
        }
    }
}
