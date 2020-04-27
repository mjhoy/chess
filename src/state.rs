use crate::{
    board::Board, castling::Castling, from_to_step::FromToStep, m0ve::Action, m0ve::Move,
    piece::Piece, piece::Piece::*, player::Player, player::Player::*, pos::Pos,
};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct State {
    pub board: Board,
    pub player: Player,
    pub en_passant: Option<Pos>,
    pub castling: Castling,
}

impl State {
    /// Is the current player in check?
    pub fn in_check(&self) -> bool {
        let to_pos = self.board.get_king_pos(self.player);
        let next_move_state = State {
            board: self.board.clone(),
            player: self.player.other(),
            en_passant: None,
            castling: self.castling.clone(),
        };

        for from_pos in self.board.coords() {
            if next_move_state.can_move_pseudo(from_pos, to_pos) {
                return true;
            }
        }
        false
    }

    fn can_move_piece(&self, piece: Piece, from: Pos, to: Pos) -> bool {
        let (board, player) = (&self.board, self.player);
        let to_piece = board.piece_at(to);
        match piece {
            Pawn => can_move_pawn(board, player, from, to, to_piece.is_some(), self.en_passant),
            Bishop => can_move_bishop(board, from, to),
            King => can_move_king(from, to),
            Rook => can_move_rook(board, from, to),
            Queen => can_move_queen(board, from, to),
            Knight => can_move_knight(from, to),
        }
    }

    // Can the current player move the piece, not taking into account
    // whether the king is in check?
    fn can_move_pseudo(&self, from: Pos, to: Pos) -> bool {
        let from_piece = self.board.piece_at(from);
        let to_piece = self.board.piece_at(to);

        match (from_piece, to_piece) {
            (None, _) => false,
            (Some((fp, _)), _) if fp != self.player => false,
            (_, Some((tp, _))) if tp == self.player => false,
            (Some((_, piece)), _) => self.can_move_piece(piece, from, to),
        }
    }

    /// Can the current player move the piece in `from_pos` to `to_pos`?
    pub fn can_move(&self, from_pos: Pos, to_pos: Pos) -> bool {
        if !self.can_move_pseudo(from_pos, to_pos) {
            return false;
        }

        let next_state = State {
            player: self.player,
            board: self.board.move_piece(from_pos, to_pos),
            en_passant: None,
            castling: Castling::initial(),
        };
        !next_state.in_check()
    }

    fn en_passant_pos(&self, from: Pos, to: Pos) -> Option<Pos> {
        match self.board.piece_at(from) {
            Some((_, Pawn)) if from.abs_diff(to).rank == 2 => {
                let en_passant_rank = if from.rank > to.rank {
                    from.rank - 1
                } else {
                    from.rank + 1
                };
                Some(Pos {
                    rank: en_passant_rank,
                    file: from.file,
                })
            }
            _ => None,
        }
    }

    fn build_move(&self, from: Pos, to: Pos) -> Move {
        let is_en_passant_capture = match self.board.piece_at(from) {
            Some((_, Pawn)) => Some(to) == self.en_passant,
            _ => false,
        };
        let next_board = if is_en_passant_capture {
            let captured_pawn = Pos {
                rank: from.rank,
                file: to.file,
            };
            self.board
                .move_piece(from, to)
                .move_piece(from, captured_pawn)
        } else {
            self.board.move_piece(from, to)
        };
        let next_state = State {
            board: next_board,
            player: self.player.other(),
            en_passant: self.en_passant_pos(from, to),
            castling: Castling::initial(),
        };
        Move {
            action: Action::Simple { from, to },
            next: next_state,
        }
    }

    /// Generate the next legal moves for this game state.
    /// On^2 for n squares
    pub fn gen_moves(&self) -> Vec<Move> {
        let coords = self.board.coords();
        coords
            .iter()
            .cartesian_product(coords.iter())
            .filter_map(|(&from, &to)| {
                if self.can_move(from, to) {
                    Some(self.build_move(from, to))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn can_move_pawn(
    board: &Board,
    player: Player,
    from: Pos,
    to: Pos,
    capture: bool,
    en_passant: Option<Pos>,
) -> bool {
    // check two-square advance
    if !capture && from.file == to.file {
        match (player, from.rank, to.rank) {
            (White, 1, 3) => {
                return board
                    .piece_at(Pos {
                        rank: 2,
                        file: from.file,
                    })
                    .is_none();
            }
            (Black, 6, 4) => {
                return board
                    .piece_at(Pos {
                        rank: 5,
                        file: from.file,
                    })
                    .is_none();
            }
            _ => (),
        }
    }

    let next_rank = i32::from(from.rank) + if player == White { 1 } else { -1 };
    if to.rank != next_rank as u8 {
        return false;
    }

    if capture {
        (to.file > 0 /* u8 guard */ && from.file == to.file - 1) || from.file == to.file + 1
    } else {
        Some(to) == en_passant || from.file == to.file
    }
}

fn can_move_king(from: Pos, to: Pos) -> bool {
    let diff = from.abs_diff(to);
    diff.rank <= 1 && diff.file <= 1
}

fn can_move_rook(board: &Board, from: Pos, to: Pos) -> bool {
    can_move_laterally(board, from, to)
}

fn can_move_bishop(board: &Board, from: Pos, to: Pos) -> bool {
    can_move_diagonally(board, from, to)
}

fn can_move_queen(board: &Board, from: Pos, to: Pos) -> bool {
    can_move_diagonally(board, from, to) || can_move_laterally(board, from, to)
}

fn can_move_knight(from: Pos, to: Pos) -> bool {
    let diff = from.abs_diff(to);
    diff.rank >= 1 && diff.file >= 1 && diff.rank + diff.file == 3
}

fn can_move_laterally(board: &Board, from: Pos, to: Pos) -> bool {
    if from == to {
        return false;
    }

    if to.file == from.file {
        let range = if to.rank > from.rank {
            (from.rank + 1)..to.rank
        } else {
            (to.rank + 1)..from.rank
        };

        for next_rank in range {
            let next_pos = Pos {
                rank: next_rank,
                file: to.file,
            };
            if board.piece_at(next_pos).is_some() {
                return false;
            }
        }

        true
    } else if to.rank == from.rank {
        let range = if to.file > from.file {
            (from.file + 1)..to.file
        } else {
            (to.file + 1)..from.file
        };

        for next_file in range {
            let next_pos = Pos {
                rank: to.rank,
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

fn can_move_diagonally(board: &Board, from: Pos, to: Pos) -> bool {
    let diff = from.abs_diff(to);

    if diff.rank == diff.file && diff.rank > 0 {
        let ranks = FromToStep::from_to(from.rank, to.rank);
        let files = FromToStep::from_to(from.file, to.file);
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

    fn simple_state(board: Board, player: Player) -> State {
        State {
            board,
            player,
            en_passant: None,
            castling: Castling::initial(),
        }
    }

    fn test_board() -> Board {
        Board::initial()
    }

    fn test_simple_board_for_piece_lateral_king(piece: Piece) -> Board {
        // White king at d4, black king at b7, and white's variable
        // piece at c4. E.g., the queen on this board:
        // https://lichess.org/analysis/standard/8/1k6/8/8/2QK4/8/8/8_w_-_-
        let piece_str = piece_to_fen((Player::White, piece));
        let (_, state) = fen(format!("8/1k6/8/8/2{}K4/8/8/8 w - -", piece_str).as_str()).unwrap();
        state.board
    }

    fn test_simple_board_for_piece_diagonal_king(piece: Piece) -> Board {
        // White king at d3, black king at b7, and white's variable
        // piece at c4. E.g., the queen on this board:
        // https://lichess.org/analysis/standard/8/1k6/8/8/2Q5/3K4/8/8/8_w_-_-
        let piece_str = piece_to_fen((Player::White, piece));
        let (_, state) = fen(format!("8/1k6/8/8/2{}5/3K4/8/8 w - -", piece_str).as_str()).unwrap();
        state.board
    }

    #[test]
    fn test_can_move_pseudo() {
        let board = test_board();

        let white_move = simple_state(board.clone(), White);
        let black_move = simple_state(board.clone(), Black);

        assert!(white_move.can_move_pseudo(e2, e3));
        assert!(!white_move.can_move_pseudo(a1, a3));
        assert!(!white_move.can_move_pseudo(b7, b6));
        assert!(black_move.can_move_pseudo(b7, b6));
    }

    #[test]
    fn test_rook_moves() {
        let board = test_simple_board_for_piece_lateral_king(Piece::Rook);

        let white_move = simple_state(board, White);

        assert!(white_move.can_move(c4, c1));
        assert!(white_move.can_move(c4, c7));
        assert!(white_move.can_move(c4, a4));
        assert!(!white_move.can_move(c4, h4)); // can't move through the king
    }

    #[test]
    fn test_bishop_moves() {
        let board = test_simple_board_for_piece_diagonal_king(Piece::Bishop);

        let white_move = simple_state(board, White);

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

        let white_move = simple_state(board, White);

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

        let white_move = simple_state(board, White);

        assert!(white_move.can_move(c4, c1));
        assert!(white_move.can_move(c4, c7));
        assert!(white_move.can_move(c4, a4));
        assert!(!white_move.can_move(c4, h4)); // can't move through the king
    }

    #[test]
    fn test_knight_moves() {
        let board = test_simple_board_for_piece_lateral_king(Piece::Knight);

        let white_move = simple_state(board, White);

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
        let (_, not_in_check_state) = fen("8/8/8/8/8/pkp5/8/PKP5 w - -").unwrap();
        assert!(!not_in_check_state.in_check());

        let (_, in_check_state_1) = fen("8/8/8/8/8/1kp5/p7/PKP5 w - -").unwrap();
        assert!(in_check_state_1.in_check());

        let (_, in_check_state_2) = fen("8/8/8/8/8/pk6/P1p5/1KP5 b - -").unwrap();
        assert!(in_check_state_2.in_check());
    }

    #[test]
    fn test_one_square_pawn_advance() {
        let (_, initial_state) = fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -").unwrap();
        assert!(initial_state.can_move(e2, e3));

        let next_state = initial_state.build_move(e2, e3).next;
        assert_eq!(next_state.en_passant, None);

        let (_, one_e4) = fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b - -").unwrap();
        assert!(one_e4.can_move(e7, e6));

        let next_state = one_e4.build_move(e7, e6).next;
        assert_eq!(next_state.en_passant, None);
    }

    #[test]
    fn test_two_square_pawn_advance() {
        let (_, initial_state) = fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -").unwrap();
        assert!(initial_state.can_move(e2, e4));

        let next_state = initial_state.build_move(e2, e4).next;
        assert_eq!(next_state.en_passant, Some(e3));

        let (_, one_e4) = fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b - -").unwrap();
        assert!(one_e4.can_move(e7, e5));

        let next_state = one_e4.build_move(e7, e5).next;
        assert_eq!(next_state.en_passant, Some(e6));

        let (_, blocking) = fen("rnbqk1nr/pppp1ppp/4p3/8/4P3/b2P4/PPP2PPP/RNBQKBNR w - -").unwrap();
        assert!(!blocking.can_move(a2, a4));

        let (_, blocking2) = fen("rnbqk1nr/pppp1ppp/8/3Pp3/4P3/b7/PPP2PPP/RNBQKBNR b - -").unwrap();
        assert!(!blocking2.can_move(d7, d5));
    }

    #[test]
    fn test_en_passant_capture() {
        let (_, initial_state) =
            fen("rnbqkbnr/ppppp1p1/7p/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6").unwrap();
        assert!(initial_state.can_move(e5, f6));
        let next_state = initial_state.build_move(e5, f6).next;
        assert_eq!(next_state.board.piece_at(f5), None);

        let (_, initial_state) =
            fen("rnbqkbnr/ppppp1p1/7p/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq -").unwrap();
        assert!(!initial_state.can_move(e5, f6));
    }
}
