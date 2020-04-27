use crate::{pos::Pos, state::State};
use std::fmt;

pub enum Action {
    Simple { from: Pos, to: Pos },
    Castle { kingside: bool },
}

pub struct Move {
    pub action: Action,
    pub next: State,
}

impl fmt::Display for Move {
    /// Pretty print a move.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.action {
            Action::Simple { from, to } => {
                let from_file = (from.file + b'A') as char;
                let from_rank = from.rank + 1;
                let to_file = (to.file + b'A') as char;
                let to_rank = to.rank + 1;
                write!(f, "{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
            }
            Action::Castle { kingside } => {
                let king_or_queenside = if kingside { "kingside" } else { "queenside" };
                write!(f, "castle {}", king_or_queenside)
            }
        }
    }
}
