#![allow(non_upper_case_globals)]

use crate::board::Board;
use crate::player::Player;
use crate::pos::Pos;
use crate::pos::*;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CastleAbility {
    pub king: bool,
    pub queen: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Castling {
    pub white: CastleAbility,
    pub black: CastleAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castleside {
    Kingside,
    Queenside,
}

impl fmt::Display for Castleside {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Castleside::Kingside => write!(f, "kingside"),
            Castleside::Queenside => write!(f, "queenside"),
        }
    }
}

impl Castling {
    pub fn initial() -> Self {
        Castling {
            white: CastleAbility {
                king: true,
                queen: true,
            },
            black: CastleAbility {
                king: true,
                queen: true,
            },
        }
    }

    pub fn able(self, player: Player, castleside: Castleside) -> bool {
        match (player, castleside) {
            (Player::White, Castleside::Kingside) => self.white.king,
            (Player::White, Castleside::Queenside) => self.white.queen,
            (Player::Black, Castleside::Kingside) => self.black.king,
            (Player::Black, Castleside::Queenside) => self.black.queen,
        }
    }

    fn without(self, player: Player, kingside: bool, queenside: bool) -> Self {
        match player {
            Player::White => Self {
                white: CastleAbility {
                    king: if kingside { false } else { self.white.king },
                    queen: if queenside { false } else { self.white.queen },
                },
                ..self
            },
            Player::Black => Self {
                black: CastleAbility {
                    king: if kingside { false } else { self.black.king },
                    queen: if queenside { false } else { self.black.queen },
                },
                ..self
            },
        }
    }

    /// If `player` moves a piece at `pos`, what's the next castling state?
    pub fn after_move(self, player: Player, pos: Pos) -> Self {
        match player {
            Player::White => match pos {
                e1 => self.without(player, true, true),
                h1 => self.without(player, true, false),
                a1 => self.without(player, false, true),
                _ => self,
            },
            Player::Black => match pos {
                e8 => self.without(player, true, true),
                h8 => self.without(player, true, false),
                a8 => self.without(player, false, true),
                _ => self,
            },
        }
    }

    /// Castle. Returns the new castling and board state.
    pub fn castle(self, board: &Board, player: Player, castleside: Castleside) -> (Board, Self) {
        let next_castling = self.without(player, true, true);
        let next_board = match (player, castleside) {
            (Player::White, Castleside::Kingside) => board.move_piece(e1, g1).move_piece(h1, f1),
            (Player::White, Castleside::Queenside) => board.move_piece(e1, c1).move_piece(a1, d1),
            (Player::Black, Castleside::Kingside) => board.move_piece(e8, g8).move_piece(h8, f8),
            (Player::Black, Castleside::Queenside) => board.move_piece(e8, c8).move_piece(a8, d8),
        };
        (next_board, next_castling)
    }

    /// Is the castling for `player` unobstructed at `kingside` on a given `board`?
    pub fn free(board: &Board, player: Player, castleside: Castleside) -> bool {
        match (player, castleside) {
            (Player::White, Castleside::Kingside) => board.all_empty(&[f1, g1]),
            (Player::White, Castleside::Queenside) => board.all_empty(&[b1, c1, d1]),
            (Player::Black, Castleside::Kingside) => board.all_empty(&[f8, g8]),
            (Player::Black, Castleside::Queenside) => board.all_empty(&[b8, c8, d8]),
        }
    }

    // Returns the two squares through which the king moves.
    pub fn king_tracks(player: Player, castleside: Castleside) -> (Pos, Pos) {
        match (player, castleside) {
            (Player::White, Castleside::Kingside) => (f1, g1),
            (Player::White, Castleside::Queenside) => (d1, c1),
            (Player::Black, Castleside::Kingside) => (f8, g8),
            (Player::Black, Castleside::Queenside) => (d8, c8),
        }
    }
}
