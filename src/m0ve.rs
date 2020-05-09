use crate::{castling::Castleside, pos::Pos, state::State};
use std::fmt;

pub enum Action {
    Simple { from: Pos, to: Pos },
    Castle { castleside: Castleside },
}

pub struct Move {
    pub action: Action,
    pub next: State,
}

impl fmt::Display for Move {
    /// Pretty print a move.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.action {
            Action::Simple { from, to } => {
                let from_file = (from.file + b'A') as char;
                let from_rank = from.rank + 1;
                let to_file = (to.file + b'A') as char;
                let to_rank = to.rank + 1;
                write!(f, "{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
            }
            Action::Castle { castleside } => write!(f, "castle {}", castleside),
        }
    }
}
