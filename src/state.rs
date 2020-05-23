use crate::{
    board::Board, castles::Castles, castles::Castleside, m0ve::Action, m0ve::Move, piece::Piece::*,
    player::Player, pos::Pos,
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub board: Board,
    pub player: Player,
    pub en_passant: Option<Pos>,
    pub castling: Castles,
}

impl State {
    /// Is the current player in check?
    fn in_check(&self) -> bool {
        let to_pos = self.board.get_king_pos(self.player);
        let next_move_state = State {
            board: self.board.clone(),
            player: self.player.other(),
            en_passant: None,
            castling: self.castling,
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
    fn can_move_pseudo(&self, from: Pos, to: Pos) -> bool {
        let from_piece = self.board.piece_at(from);
        let to_piece = self.board.piece_at(to);

        match (from_piece, to_piece) {
            (None, _) => false,
            (Some((fp, _)), _) if fp != self.player => false,
            (_, Some((tp, _))) if tp == self.player => false,
            (Some((_, piece)), _) => piece.eyes(from, to, self),
        }
    }

    fn move_puts_current_player_in_check(&self, from_pos: Pos, to_pos: Pos) -> bool {
        let next_state = State {
            player: self.player,
            board: self.board.move_piece(from_pos, to_pos),
            en_passant: None,
            castling: self.castling,
        };
        next_state.in_check()
    }

    /// Can the current player move the piece in `from_pos` to `to_pos`?
    fn can_move(&self, from_pos: Pos, to_pos: Pos) -> bool {
        if !self.can_move_pseudo(from_pos, to_pos) {
            return false;
        }

        !self.move_puts_current_player_in_check(from_pos, to_pos)
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

    fn build_simple_move(&self, from: Pos, to: Pos) -> Move {
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
        let next_castling = self.castling.after_move(self.player, from);
        let next_state = State {
            board: next_board,
            player: self.player.other(),
            en_passant: self.en_passant_pos(from, to),
            castling: next_castling,
        };
        Move {
            action: Action::Simple { from, to },
            next: next_state,
        }
    }

    fn can_castle(&self, castleside: Castleside) -> bool {
        // Return early if it's not possible to castle, before
        // calculating passing through checks.
        if !(self.castling.able(self.player, castleside)
            && Castles::free(&self.board, self.player, castleside))
        {
            return false;
        }

        let king_pos = self.board.get_king_pos(self.player);
        let (pos_1, pos_2) = Castles::king_tracks(self.player, castleside);

        !self.in_check()
            && !self.move_puts_current_player_in_check(king_pos, pos_1)
            && !self.move_puts_current_player_in_check(king_pos, pos_2)
    }

    fn build_castle_move(&self, castleside: Castleside) -> Move {
        let (next_board, next_castling) =
            self.castling.castle(&self.board, self.player, castleside);
        let next_state = State {
            board: next_board,
            player: self.player.other(),
            en_passant: self.en_passant,
            castling: next_castling,
        };
        Move {
            action: Action::Castle { castleside },
            next: next_state,
        }
    }

    fn make_castle_move(&self, castleside: Castleside) -> Option<Move> {
        if self.can_castle(castleside) {
            Some(self.build_castle_move(castleside))
        } else {
            None
        }
    }

    fn make_simple_move(&self, from: Pos, to: Pos) -> Option<Move> {
        if self.can_move(from, to) {
            Some(self.build_simple_move(from, to))
        } else {
            None
        }
    }

    /// Generate the next legal moves for this game state.
    /// On^2 for n squares
    pub fn gen_moves(&self) -> Vec<Move> {
        let coords = self.board.coords();
        let castles = [Castleside::Kingside, Castleside::Queenside]
            .iter()
            .filter_map(|&castleside| self.make_castle_move(castleside));
        let simples = coords
            .iter()
            .cartesian_product(coords.iter())
            .filter_map(|(&from, &to)| self.make_simple_move(from, to));
        castles.chain(simples).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fen::fen;
    use crate::piece::Piece;
    use crate::player::Player::*;
    use crate::pos::*;

    fn simple_state(board: Board, player: Player) -> State {
        State {
            board,
            player,
            en_passant: None,
            castling: Castles::initial(),
        }
    }

    fn test_board() -> Board {
        Board::initial()
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
    fn test_in_check() {
        let (_, not_in_check_state) = fen("8/8/8/8/8/pkp5/8/PKP5 w - -").unwrap();
        assert!(!not_in_check_state.in_check());

        let (_, in_check_state_1) = fen("8/8/8/8/8/1kp5/p7/PKP5 w - -").unwrap();
        assert!(in_check_state_1.in_check());

        let (_, in_check_state_2) = fen("8/8/8/8/8/pk6/P1p5/1KP5 b - -").unwrap();
        assert!(in_check_state_2.in_check());
    }

    #[test]
    fn test_castling_white_kingside_allowed() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp2pppp/3p1n2/2p5/2B5/4PN2/PPPP1PPP/RNBQK2R w KQkq - 0 4").unwrap();
        assert!(initial_state.can_castle(Castleside::Kingside));
        let next_state = initial_state.build_castle_move(Castleside::Kingside).next;
        assert_eq!(
            next_state.board.piece_at(g1),
            Some((Player::White, Piece::King))
        );
        assert_eq!(
            next_state.board.piece_at(f1),
            Some((Player::White, Piece::Rook))
        );
        assert_eq!(next_state.castling.white.kingside, false);
        assert_eq!(next_state.castling.white.queenside, false);
        assert_eq!(next_state.castling.black.kingside, true);
        assert_eq!(next_state.castling.black.queenside, true);
    }

    #[test]
    fn test_castling_white_kingside_not_allowed() {
        let (_, initial_state) =
            fen("rnbqk2r/pp2ppbp/3p1np1/2p5/2B5/4PN2/PPPP1PPP/RNBQK2R w Qkq - 2 6").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_white_kingside_not_allowed_after_rook_move() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp2pppp/3p1n2/2p5/2B5/4PN2/PPPP1PPP/RNBQK2R w KQkq - 0 4").unwrap();
        assert!(initial_state.can_castle(Castleside::Kingside));
        let next_state = initial_state.build_simple_move(h1, g1).next;
        assert_eq!(next_state.castling.white.kingside, false);
        assert_eq!(next_state.castling.white.queenside, true);
    }

    #[test]
    fn test_castling_white_queenside_allowed() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp3ppp/2p1pn2/3p4/3P1B2/2NQ4/PPP1PPPP/R3KBNR w KQkq - 0 5").unwrap();
        assert!(initial_state.can_castle(Castleside::Queenside));
        let next_state = initial_state.build_castle_move(Castleside::Queenside).next;
        assert_eq!(
            next_state.board.piece_at(c1),
            Some((Player::White, Piece::King))
        );
        assert_eq!(
            next_state.board.piece_at(d1),
            Some((Player::White, Piece::Rook))
        );
        assert_eq!(next_state.castling.white.kingside, false);
        assert_eq!(next_state.castling.white.queenside, false);
        assert_eq!(next_state.castling.black.kingside, true);
        assert_eq!(next_state.castling.black.queenside, true);
    }

    #[test]
    fn test_castling_white_queenside_not_allowed() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp3ppp/2p1pn2/3p4/3P1B2/2NQ4/PPP1PPPP/1R2KBNR b Kkq - 1 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }

    #[test]
    fn test_castling_white_not_allowed_after_rook_move() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp3ppp/2p1pn2/3p4/3P1B2/2NQ4/PPP1PPPP/R3KBNR w KQkq - 0 5").unwrap();
        assert!(initial_state.can_castle(Castleside::Queenside));
        let next_state = initial_state.build_simple_move(a1, b1).next;
        assert_eq!(next_state.castling.white.kingside, true);
        assert_eq!(next_state.castling.white.queenside, false);
    }

    #[test]
    fn test_castling_white_queenside_not_allowed_after_king_move() {
        let (_, initial_state) =
            fen("rnbqkb1r/pp3ppp/2p1pn2/3p4/3P1B2/2NQ4/PPP1PPPP/R3KBNR w KQkq - 0 5").unwrap();
        assert!(initial_state.can_castle(Castleside::Queenside));
        let next_state = initial_state.build_simple_move(e1, f1).next;
        assert_eq!(next_state.castling.white.kingside, false);
        assert_eq!(next_state.castling.white.queenside, false);
    }

    #[test]
    fn test_castling_white_kingside_not_allowed_out_of_check_move() {
        let (_, initial_state) =
            fen("rnbqk1nr/pp1p2pp/2p1pp2/8/1b6/3PPN2/PPP1BPPP/RNBQK2R w KQkq - 2 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_white_kingside_not_allowed_through_check_on_f1_move() {
        let (_, initial_state) =
            fen("rn1qkbnr/ppp1pppp/B2p4/8/2b5/4PN2/PPPP1PPP/RNBQK2R w KQkq - 4 4").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_white_kingside_not_allowed_through_check_on_g1_move() {
        let (_, initial_state) =
            fen("rnbqk2r/pp1p1ppp/2p1p2n/2b5/4PP2/3B1N2/PPPP2PP/RNBQK2R w KQkq - 2 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_white_queenside_not_allowed_through_check_on_d1_move() {
        let (_, initial_state) =
            fen("rn1qk2r/ppp1bppp/3p1n2/4p1B1/3PP1b1/2NQ4/PPP2PPP/R3KBNR w KQkq - 4 6").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }

    #[test]
    fn test_castling_white_queenside_not_allowed_through_check_on_c1_move() {
        let (_, initial_state) =
            fen("rnbqk2r/ppp1pp1p/3p1npb/8/3P4/2NQ4/PPP1PPPP/R3KBNR w KQkq - 2 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }

    #[test]
    fn test_castling_black_kingside_allowed() {
        let (_, initial_state) =
            fen("rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/3P4/PPPB1PPP/RN1QKBNR b KQkq - 2 4").unwrap();
        assert!(initial_state.can_castle(Castleside::Kingside));
        let next_state = initial_state.build_castle_move(Castleside::Kingside).next;
        assert_eq!(
            next_state.board.piece_at(g8),
            Some((Player::Black, Piece::King))
        );
        assert_eq!(
            next_state.board.piece_at(f8),
            Some((Player::Black, Piece::Rook))
        );
        assert_eq!(next_state.castling.black.kingside, false);
        assert_eq!(next_state.castling.black.queenside, false);
        assert_eq!(next_state.castling.white.kingside, true);
        assert_eq!(next_state.castling.white.queenside, true);
    }

    #[test]
    fn test_castling_black_kingside_not_allowed() {
        let (_, initial_state) =
            fen("rnbqk1r1/pppp1ppp/5n2/2b1p3/4P3/3P4/PPPB1PPP/RN1QKBNR w KQq - 3 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_black_kingside_not_allowed_after_rook_move() {
        let (_, initial_state) =
            fen("rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/3P4/PPPB1PPP/RN1QKBNR b KQkq - 2 4").unwrap();
        assert!(initial_state.can_castle(Castleside::Kingside));
        let next_state = initial_state.build_simple_move(h8, g8).next;
        assert_eq!(next_state.castling.black.kingside, false);
        assert_eq!(next_state.castling.black.queenside, true);
    }

    #[test]
    fn test_castling_black_not_allowed_after_king_move() {
        let (_, initial_state) =
            fen("rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/3P4/PPPB1PPP/RN1QKBNR b KQkq - 2 4").unwrap();
        assert!(initial_state.can_castle(Castleside::Kingside));
        let next_state = initial_state.build_simple_move(e8, f8).next;
        assert_eq!(next_state.castling.black.kingside, false);
        assert_eq!(next_state.castling.black.queenside, false);
    }

    #[test]
    fn test_castling_black_queenside_allowed() {
        let (_, initial_state) =
            fen("r3kbnr/pppqpppp/2npb3/8/3P4/2P1PN2/PP3PPP/RNBQKB1R b KQkq - 0 5").unwrap();
        assert!(initial_state.can_castle(Castleside::Queenside));
        let next_state = initial_state.build_castle_move(Castleside::Queenside).next;
        assert_eq!(
            next_state.board.piece_at(c8),
            Some((Player::Black, Piece::King))
        );
        assert_eq!(
            next_state.board.piece_at(d8),
            Some((Player::Black, Piece::Rook))
        );
        assert_eq!(next_state.castling.black.kingside, false);
        assert_eq!(next_state.castling.black.queenside, false);
        assert_eq!(next_state.castling.white.kingside, true);
        assert_eq!(next_state.castling.white.queenside, true);
    }

    #[test]
    fn test_castling_black_queenside_not_allowed() {
        let (_, initial_state) =
            fen("1r2kbnr/pppqpppp/2npb3/8/3P4/2P1PN2/PP3PPP/RNBQKB1R w KQk - 1 6").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }

    #[test]
    fn test_castling_black_queenside_not_allowed_after_rook_move() {
        let (_, initial_state) =
            fen("r3kbnr/pppqpppp/2npb3/8/3P4/2P1PN2/PP3PPP/RNBQKB1R b KQkq - 0 5").unwrap();
        assert!(initial_state.can_castle(Castleside::Queenside));
        let next_state = initial_state.build_simple_move(a8, b8).next;
        assert_eq!(next_state.castling.black.kingside, true);
        assert_eq!(next_state.castling.black.queenside, false);
    }

    #[test]
    fn test_castling_black_kingside_not_allowed_out_of_check_move() {
        let (_, initial_state) =
            fen("rnbqk2r/ppp1pp1p/5npb/1B1p4/8/2N1PN1P/PPPP1PP1/R1BQK2R b KQkq - 2 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_black_kingside_not_allowed_through_check_on_f8_move() {
        let (_, initial_state) =
            fen("rnbqk2r/pppp1ppp/5n2/4p3/8/BP1P1N2/P1P1PPPP/RN1QKB1R b KQkq - 2 4").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_black_kingside_not_allowed_through_check_on_g8_move() {
        let (_, initial_state) =
            fen("rnbqk2r/pppp2pp/5p1n/2b1p3/2B5/2NPPN2/PPP2PPP/R1BQK2R b KQkq - 0 5").unwrap();
        assert!(!initial_state.can_castle(Castleside::Kingside));
    }

    #[test]
    fn test_castling_black_queenside_not_allowed_through_check_on_d8_move() {
        let (_, initial_state) =
            fen("r3kbnr/pp1qpppp/n1p5/B2p1b2/8/3PPN1P/PPP2PP1/RN1QKB1R b KQkq - 2 6").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }

    #[test]
    fn test_castling_black_queenside_not_allowed_through_check_on_c8_move() {
        let (_, initial_state) =
            fen("r3kbnr/p1pqpppp/Bpnp4/8/3P4/2P1PNP1/PP3P1P/RNBQK2R b KQkq - 0 6").unwrap();
        assert!(!initial_state.can_castle(Castleside::Queenside));
    }
}
