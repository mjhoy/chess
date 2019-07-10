use crate::{pos::Pos, state::State};
use std::fmt;

pub struct Move {
    pub index: (Pos, Pos),
    pub next: State,
}

impl fmt::Display for Move {
    /// Pretty print a move.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (from, to) = self.index;
        let from_file = (from.file + b'A') as char;
        let from_rank = from.rank + 1;
        let to_file = (to.file + b'A') as char;
        let to_rank = to.rank + 1;
        write!(f, "{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
    }
}
