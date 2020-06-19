use crate::from_to_step::FromToStep;
use crate::game::board::Board;
use crate::game::player::Player::*;
use crate::game::pos::Pos;
use crate::game::state::State;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Bishop,
    King,
    Rook,
    Queen,
    Knight,
}

impl Piece {
    fn lateral_eyes(board: &Board, from: Pos, to: Pos) -> bool {
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

    fn diagonal_eyes(board: &Board, from: Pos, to: Pos) -> bool {
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

    /// Is this piece able to move from `from` to `to` for a given `state`? This piece
    /// is owned by the current player. This does not take into account whether or not
    /// the king is in check.
    pub fn eyes(self, from: Pos, to: Pos, state: &State) -> bool {
        let board = &state.board;
        let player = state.player;

        match self {
            Piece::Pawn => {
                let capture = board.piece_at(to).is_some();
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
                    (to.file > 0 /* u8 guard */ && from.file == to.file - 1)
                        || from.file == to.file + 1
                } else {
                    Some(to) == state.en_passant || from.file == to.file
                }
            }

            Piece::King => {
                let diff = from.abs_diff(to);
                diff.rank <= 1 && diff.file <= 1
            }

            Piece::Rook => Piece::lateral_eyes(board, from, to),

            Piece::Bishop => Piece::diagonal_eyes(board, from, to),

            Piece::Queen => {
                Piece::lateral_eyes(board, from, to) || Piece::diagonal_eyes(board, from, to)
            }

            Piece::Knight => {
                let diff = from.abs_diff(to);
                diff.rank >= 1 && diff.file >= 1 && diff.rank + diff.file == 3
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::castles::Castles;
    use crate::game::player::Player;
    use crate::game::pos::*;
    use crate::parsing::parse_fen;

    fn simple_state(board: Board, player: Player) -> State {
        State {
            board,
            player,
            en_passant: None,
            castling: Castles::initial(),
        }
    }

    fn test_simple_board_for_piece_lateral_king() -> Board {
        // White king at d4, black king at b7. See here:
        // https://lichess.org/analysis/8/1k6/8/8/3K4/8/8/8_w_-_-_0_1
        let state = parse_fen("8/1k6/8/8/3K4/8/8/8 w - - 0 1").unwrap();
        state.board
    }

    fn test_simple_board_for_piece_diagonal_king() -> Board {
        // White king at d3, black king at b7. See here:
        // https://lichess.org/analysis/standard/8/1k6/8/8/2Q5/3K4/8/8/8_w_-_-
        let state = parse_fen("8/1k6/8/8/8/3K4/8/8 w - - 0 1").unwrap();
        state.board
    }

    #[test]
    fn test_rook_eyes() {
        let board = test_simple_board_for_piece_lateral_king();
        let white_move = &simple_state(board, White);
        let piece = Piece::Rook;

        assert!(piece.eyes(c4, c1, white_move));
        assert!(piece.eyes(c4, c7, white_move));
        assert!(piece.eyes(c4, a4, white_move));
        assert!(!piece.eyes(c4, h4, white_move)); // can't move through the king
    }

    #[test]
    fn test_bishop_eyes() {
        let board = test_simple_board_for_piece_diagonal_king();
        let white_move = &simple_state(board, White);
        let piece = Piece::Bishop;

        assert!(piece.eyes(c4, d5, white_move));
        assert!(piece.eyes(c4, e6, white_move));
        assert!(piece.eyes(c4, g8, white_move));
        assert!(piece.eyes(c4, a2, white_move));
        assert!(piece.eyes(c4, a6, white_move));
        assert!(!piece.eyes(c4, f1, white_move)); // can't move through white king
    }

    #[test]
    fn test_queen_diagonal_moves() {
        let board = test_simple_board_for_piece_diagonal_king();
        let white_move = &simple_state(board, White);
        let piece = Piece::Queen;

        assert!(piece.eyes(c4, d5, white_move));
        assert!(piece.eyes(c4, e6, white_move));
        assert!(piece.eyes(c4, g8, white_move));
        assert!(piece.eyes(c4, a2, white_move));
        assert!(piece.eyes(c4, a6, white_move));
        assert!(!piece.eyes(c4, f1, white_move)); // can't move through white king
    }

    #[test]
    fn test_queen_lateral_moves() {
        let board = test_simple_board_for_piece_lateral_king();
        let white_move = &simple_state(board, White);
        let piece = Piece::Queen;

        assert!(piece.eyes(c4, c1, white_move));
        assert!(piece.eyes(c4, c7, white_move));
        assert!(piece.eyes(c4, a4, white_move));
        assert!(!piece.eyes(c4, h4, white_move)); // can't move through the king
    }

    #[test]
    fn test_knight_moves() {
        let board = test_simple_board_for_piece_lateral_king();
        let white_move = &simple_state(board, White);
        let piece = Piece::Knight;

        let valid_moves = vec![b6, a5, a3, b2, d2, e3, e5, d6];
        for pos in white_move.board.coords().iter() {
            if valid_moves.contains(pos) {
                assert!(piece.eyes(c4, *pos, white_move));
            } else {
                assert!(!piece.eyes(c4, *pos, white_move));
            }
        }
    }

    #[test]
    fn test_one_square_pawn_advance() {
        let initial_state = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -").unwrap();
        assert!(Piece::Pawn.eyes(e2, e3, &initial_state));

        let one_e4 = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b - -").unwrap();
        assert!(Piece::Pawn.eyes(e7, e6, &one_e4));
    }

    #[test]
    fn test_two_square_pawn_advance() {
        let initial_state = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -").unwrap();
        assert!(Piece::Pawn.eyes(e2, e4, &initial_state));

        let one_e4 = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b - -").unwrap();
        assert!(Piece::Pawn.eyes(e7, e5, &one_e4));

        let blocking =
            parse_fen("rnbqk1nr/pppp1ppp/4p3/8/4P3/b2P4/PPP2PPP/RNBQKBNR w - -").unwrap();
        assert!(!Piece::Pawn.eyes(a2, a4, &blocking));

        let blocking2 =
            parse_fen("rnbqk1nr/pppp1ppp/8/3Pp3/4P3/b7/PPP2PPP/RNBQKBNR b - -").unwrap();
        assert!(!Piece::Pawn.eyes(d7, d5, &blocking2));
    }

    #[test]
    fn test_en_passant_capture() {
        let initial_state =
            parse_fen("rnbqkbnr/ppppp1p1/7p/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6").unwrap();
        assert!(Piece::Pawn.eyes(e5, f6, &initial_state));

        let initial_state =
            parse_fen("rnbqkbnr/ppppp1p1/7p/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq -").unwrap();
        assert!(!Piece::Pawn.eyes(e5, f6, &initial_state));
    }
}
