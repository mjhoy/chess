use crate::board::Board;
use crate::piece::Piece;
use crate::player::Player;
use crate::pos;
use crate::pos::Pos;

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

    pub fn able(self, player: Player, kingside: bool) -> bool {
        match (player, kingside) {
            (Player::White, true) => self.white.king,
            (Player::White, false) => self.white.queen,
            (Player::Black, true) => self.black.king,
            (Player::Black, false) => self.black.queen,
        }
    }

    /// If `player` moves a piece at `pos`, what's the next castling state?
    pub fn after_move(self, player: Player, pos: Pos) -> Self {
        match player {
            Player::White => match pos {
                pos::e1 => Castling {
                    white: CastleAbility {
                        king: false,
                        queen: false,
                    },
                    ..self
                },
                pos::h1 => Castling {
                    white: CastleAbility {
                        king: false,
                        queen: self.white.queen,
                    },
                    ..self
                },
                pos::a1 => Castling {
                    white: CastleAbility {
                        king: self.white.king,
                        queen: false,
                    },
                    ..self
                },
                _ => self,
            },
            Player::Black => match pos {
                pos::e8 => Castling {
                    black: CastleAbility {
                        king: false,
                        queen: false,
                    },
                    ..self
                },
                pos::h8 => Castling {
                    black: CastleAbility {
                        king: false,
                        queen: self.black.queen,
                    },
                    ..self
                },
                pos::a8 => Castling {
                    black: CastleAbility {
                        king: self.black.king,
                        queen: false,
                    },
                    ..self
                },
                _ => self,
            },
        }
    }

    /// Castle. Returns the new castling and board state.
    pub fn castle(self, board: &Board, player: Player, kingside: bool) -> (Board, Self) {
        if player.is_white() {
            let next_castling = Castling {
                white: CastleAbility {
                    king: false,
                    queen: false,
                },
                ..self
            };
            let next_board = if kingside {
                board
                    .move_piece(pos::e1, pos::g1)
                    .move_piece(pos::h1, pos::f1)
            } else {
                board
                    .move_piece(pos::e1, pos::c1)
                    .move_piece(pos::a1, pos::d1)
            };
            (next_board, next_castling)
        } else {
            let next_castling = Castling {
                black: CastleAbility {
                    king: false,
                    queen: false,
                },
                ..self
            };
            let next_board = if kingside {
                board
                    .move_piece(pos::e8, pos::g8)
                    .move_piece(pos::h8, pos::f8)
            } else {
                board
                    .move_piece(pos::e8, pos::c8)
                    .move_piece(pos::a8, pos::d8)
            };
            (next_board, next_castling)
        }
    }

    /// Is the castling for `player` unobstructed at `kingside` on a given `board`?
    pub fn free(board: &Board, player: Player, kingside: bool) -> bool {
        match (player, kingside) {
            (Player::White, true) => {
                board.piece_at(pos::e1) == Some((player, Piece::King))
                    && board.piece_at(pos::h1) == Some((player, Piece::Rook))
                    && board.empty_at(pos::f1)
                    && board.empty_at(pos::g1)
            }
            (Player::White, false) => {
                board.piece_at(pos::e1) == Some((player, Piece::King))
                    && board.piece_at(pos::a1) == Some((player, Piece::Rook))
                    && board.empty_at(pos::b1)
                    && board.empty_at(pos::c1)
                    && board.empty_at(pos::d1)
            }
            (Player::Black, true) => {
                board.piece_at(pos::e8) == Some((player, Piece::King))
                    && board.piece_at(pos::h8) == Some((player, Piece::Rook))
                    && board.empty_at(pos::f8)
                    && board.empty_at(pos::g8)
            }
            (Player::Black, false) => {
                board.piece_at(pos::e8) == Some((player, Piece::King))
                    && board.piece_at(pos::a8) == Some((player, Piece::Rook))
                    && board.empty_at(pos::b8)
                    && board.empty_at(pos::c8)
                    && board.empty_at(pos::d8)
            }
        }
    }

    // Returns the two squares through which the king moves.
    pub fn king_tracks(player: Player, kingside: bool) -> (Pos, Pos) {
        match (player, kingside) {
            (Player::White, true) => (pos::f1, pos::g1),
            (Player::White, false) => (pos::d1, pos::c1),
            (Player::Black, true) => (pos::f8, pos::g8),
            (Player::Black, false) => (pos::d8, pos::c8),
        }
    }
}
