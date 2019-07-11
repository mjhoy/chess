use crate::{
    board::Board, from_to_step::FromToStep, m0ve::Move, piece::Piece::*, player::Player,
    player::Player::*, pos::Pos,
};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct State {
    pub board: Board,
    pub player: Player,
}

impl State {
    /// Is the current player in check?
    pub fn in_check(&self) -> bool {
        let to_pos = self.board.get_king_pos(self.player);
        let next_move_state = State {
            board: self.board.clone(),
            player: self.player.other(),
        };

        for from_pos in self.board.coords() {
            if next_move_state.can_move_pseudo(from_pos, to_pos) {
                return true;
            }
        }
        false
    }

    // Can the current player move the piece, not taking into account
    // whether the king is in check?
    fn can_move_pseudo(&self, from_pos: Pos, to_pos: Pos) -> bool {
        let from = self.board.piece_at(from_pos);
        let to = self.board.piece_at(to_pos);

        match (from, to) {
            (None, _) => false,
            (Some((fp, _)), _) if fp != self.player => false,
            (_, Some((tp, _))) if tp == self.player => false,
            (Some((_, piece)), _) => match piece {
                Pawn => can_move_pawn(self.player, from_pos, to_pos, to.is_some()),
                Bishop => can_move_bishop(&self.board, from_pos, to_pos),
                King => can_move_king(from_pos, to_pos),
                Rook => can_move_rook(&self.board, from_pos, to_pos),
                Queen => can_move_queen(&self.board, from_pos, to_pos),
                Knight => can_move_knight(from_pos, to_pos),
            },
        }
    }

    /// Can the current player move the piece in `from_pos` to `to_pos`?
    pub fn can_move(&self, from_pos: Pos, to_pos: Pos) -> bool {
        if !self.can_move_pseudo(from_pos, to_pos) {
            return false;
        }

        let next_state = &State {
            player: self.player,
            board: self.board.move_piece(from_pos, to_pos),
        };
        !next_state.in_check()
    }

    /// Generate the next legal moves for this game state.
    /// On^2 for n squares
    pub fn gen_moves(&self) -> Vec<Move> {
        let coords = self.board.coords();
        coords
            .iter()
            .cartesian_product(coords.iter())
            .filter_map(|(from_pos, to_pos)| {
                if self.can_move(*from_pos, *to_pos) {
                    Some(Move {
                        index: (*from_pos, *to_pos),
                        next: State {
                            board: self.board.move_piece(*from_pos, *to_pos),
                            player: self.player.other(),
                        },
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

fn can_move_pawn(player: Player, from_pos: Pos, to_pos: Pos, capture: bool) -> bool {
    let next_rank = i32::from(from_pos.rank) + if player == White { 1 } else { -1 };
    if to_pos.rank != next_rank as u8 {
        return false;
    }

    if capture {
        (to_pos.file > 0 /* u8 guard */ && from_pos.file == to_pos.file - 1)
            || from_pos.file == to_pos.file + 1
    } else {
        from_pos.file == to_pos.file
    }
}

fn can_move_king(from_pos: Pos, to_pos: Pos) -> bool {
    (i32::from(from_pos.rank) - i32::from(to_pos.rank)).abs() <= 1
        && (i32::from(from_pos.file) - i32::from(to_pos.file)).abs() <= 1
}

fn can_move_rook(board: &Board, from_pos: Pos, to_pos: Pos) -> bool {
    can_move_laterally(board, from_pos, to_pos)
}

fn can_move_bishop(board: &Board, from_pos: Pos, to_pos: Pos) -> bool {
    can_move_diagonally(board, from_pos, to_pos)
}

fn can_move_queen(board: &Board, from_pos: Pos, to_pos: Pos) -> bool {
    can_move_diagonally(board, from_pos, to_pos) || can_move_laterally(board, from_pos, to_pos)
}

fn can_move_knight(from_pos: Pos, to_pos: Pos) -> bool {
    let diff = from_pos.abs_diff(to_pos);
    diff.rank >= 1 && diff.file >= 1 && diff.rank + diff.file == 3
}

fn can_move_laterally(board: &Board, from_pos: Pos, to_pos: Pos) -> bool {
    if from_pos == to_pos {
        return false;
    }

    if to_pos.file == from_pos.file {
        let range = if to_pos.rank > from_pos.rank {
            (from_pos.rank + 1)..to_pos.rank
        } else {
            (to_pos.rank + 1)..from_pos.rank
        };

        for next_rank in range {
            let next_pos = Pos {
                rank: next_rank,
                file: to_pos.file,
            };
            if board.piece_at(next_pos).is_some() {
                return false;
            }
        }

        true
    } else if to_pos.rank == from_pos.rank {
        let range = if to_pos.file > from_pos.file {
            (from_pos.file + 1)..to_pos.file
        } else {
            (to_pos.file + 1)..from_pos.file
        };

        for next_file in range {
            let next_pos = Pos {
                rank: to_pos.rank,
                file: next_file,
            };
            if board.piece_at(next_pos).is_some() {
                return false;
            }
        }

        true
    } else {
        false
    }
}

fn can_move_diagonally(board: &Board, from_pos: Pos, to_pos: Pos) -> bool {
    let diff = from_pos.abs_diff(to_pos);

    if diff.rank == diff.file && diff.rank > 0 {
        let ranks = FromToStep::from_to(from_pos.rank, to_pos.rank);
        let files = FromToStep::from_to(from_pos.file, to_pos.file);
        let coords = ranks.zip(files);
        for (rank, file) in coords {
            let pos = Pos { rank, file };
            if board.piece_at(pos).is_some() {
                return false;
            }
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fen::{fen, piece_to_fen};
    use crate::piece::Piece;
    use crate::pos::*;

    fn test_board() -> Board {
        Board::initial()
    }

    fn test_simple_board_for_piece_lateral_king(piece: Piece) -> Board {
        // White king at d4, black king at b7, and white's variable
        // piece at c4. E.g., the queen on this board:
        // https://lichess.org/analysis/standard/8/1k6/8/8/2QK4/8/8/8_w_-_-
        let piece_str = piece_to_fen((Player::White, piece));
        let (_, state) = fen(format!("8/1k6/8/8/2{}K4/8/8/8 w", piece_str).as_str()).unwrap();
        state.board
    }

    fn test_simple_board_for_piece_diagonal_king(piece: Piece) -> Board {
        // White king at d3, black king at b7, and white's variable
        // piece at c4. E.g., the queen on this board:
        // https://lichess.org/analysis/standard/8/1k6/8/8/2Q5/3K4/8/8/8_w_-_-
        let piece_str = piece_to_fen((Player::White, piece));
        let (_, state) = fen(format!("8/1k6/8/8/2{}5/3K4/8/8 w", piece_str).as_str()).unwrap();
        state.board
    }

    #[test]
    fn test_can_move_pseudo() {
        let board = test_board();

        let white_move = State {
            board: board.clone(),
            player: White,
        };
        let black_move = State {
            board: board.clone(),
            player: Black,
        };

        assert!(white_move.can_move_pseudo(e2, e3));
        assert!(!white_move.can_move_pseudo(a1, a3));
        assert!(!white_move.can_move_pseudo(b7, b6));
        assert!(black_move.can_move_pseudo(b7, b6));
    }

    #[test]
    fn test_rook_moves() {
        let board = test_simple_board_for_piece_lateral_king(Piece::Rook);

        let white_move = State {
            board,
            player: White,
        };

        assert!(white_move.can_move(c4, c1));
        assert!(white_move.can_move(c4, c7));
        assert!(white_move.can_move(c4, a4));
        assert!(!white_move.can_move(c4, h4)); // can't move through the king
    }

    #[test]
    fn test_bishop_moves() {
        let board = test_simple_board_for_piece_diagonal_king(Piece::Bishop);

        let white_move = State {
            board,
            player: White,
        };

        assert!(white_move.can_move(c4, d5));
        assert!(white_move.can_move(c4, e6));
        assert!(white_move.can_move(c4, g8));
        assert!(white_move.can_move(c4, a2));
        assert!(white_move.can_move(c4, a6));
        assert!(!white_move.can_move(c4, f1)); // can't move through white king
    }

    #[test]
    fn test_queen_diagonal_moves() {
        let board = test_simple_board_for_piece_diagonal_king(Piece::Queen);

        let white_move = State {
            board,
            player: White,
        };

        assert!(white_move.can_move(c4, d5));
        assert!(white_move.can_move(c4, e6));
        assert!(white_move.can_move(c4, g8));
        assert!(white_move.can_move(c4, a2));
        assert!(white_move.can_move(c4, a6));
        assert!(!white_move.can_move(c4, f1)); // can't move through white king
    }

    #[test]
    fn test_queen_lateral_moves() {
        let board = test_simple_board_for_piece_lateral_king(Piece::Queen);

        let white_move = State {
            board,
            player: White,
        };

        assert!(white_move.can_move(c4, c1));
        assert!(white_move.can_move(c4, c7));
        assert!(white_move.can_move(c4, a4));
        assert!(!white_move.can_move(c4, h4)); // can't move through the king
    }

    #[test]
    fn test_knight_moves() {
        let board = test_simple_board_for_piece_lateral_king(Piece::Knight);

        let white_move = State {
            board,
            player: White,
        };

        let valid_moves = vec![b6, a5, a3, b2, d2, e3, e5, d6];
        for pos in white_move.board.coords().iter() {
            if valid_moves.contains(pos) {
                assert!(white_move.can_move(c4, *pos));
            } else {
                assert!(!white_move.can_move(c4, *pos));
            }
        }
    }

    #[test]
    fn test_in_check() {
        let (_, not_in_check_state) = fen("8/8/8/8/8/pkp5/8/PKP5 w").unwrap();
        assert!(!not_in_check_state.in_check());

        let (_, in_check_state_1) = fen("8/8/8/8/8/1kp5/p7/PKP5 w").unwrap();
        assert!(in_check_state_1.in_check());

        let (_, in_check_state_2) = fen("8/8/8/8/8/pk6/P1p5/1KP5 b").unwrap();
        assert!(in_check_state_2.in_check());
    }
}
