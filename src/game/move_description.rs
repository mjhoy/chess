use crate::game::castles::Castleside;
use crate::game::m0ve::{Action, Move};
use crate::game::piece::Piece;
use crate::game::pos::Pos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveDescription {
    Simple {
        src_piece: Piece,
        src_rank: Option<u8>,
        src_file: Option<u8>,
        dst_pos: Pos,
    },
    Castle {
        castleside: Castleside,
    },
}

impl MoveDescription {
    pub fn match_moves(&self, moves: Vec<Move>) -> Option<Move> {
        let matched: Vec<Move> = moves.into_iter().filter(|m| self.match_move(m)).collect();
        if matched.len() == 1 {
            matched.into_iter().next()
        } else {
            None
        }
    }

    fn match_move(&self, m0ve: &Move) -> bool {
        match (&m0ve.action, self) {
            (
                Action::Simple { from, to },
                MoveDescription::Simple {
                    src_file,
                    src_rank,
                    src_piece,
                    dst_pos,
                },
            ) => {
                if src_file.is_some() && src_file != &Some(from.file) {
                    return false;
                }

                if src_rank.is_some() && src_rank != &Some(from.rank) {
                    return false;
                }

                let dst_piece = m0ve.next.board.piece_at(*to).map(|(_, piece)| piece);

                dst_pos == to && Some(*src_piece) == dst_piece
            }
            (
                Action::Castle {
                    castleside: action_castleside,
                },
                MoveDescription::Castle {
                    castleside: description_castleside,
                },
            ) => action_castleside == description_castleside,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::player::Player;
    use crate::game::pos::*;
    use crate::game::Game;
    use crate::parsing::parse_algebraic_notation;
    use crate::parsing::parse_fen;

    fn test_game() -> Game {
        Game::default()
    }

    #[test]
    fn test_match_moves_needs_disambiguating_file() {
        let state = parse_fen("8/3k4/8/8/8/2N1N3/3K4/8 w - - 0 1").unwrap();
        let moves = state.gen_moves();
        let desc = MoveDescription::Simple {
            src_file: None,
            src_rank: None,
            src_piece: Piece::Knight,
            dst_pos: d5,
        };
        let matched = desc.match_moves(moves);
        assert_eq!(matched, None);
    }

    #[test]
    fn test_match_moves_has_disambiguating_file() {
        let state = parse_fen("8/3k4/8/8/8/2N1N3/3K4/8 w - - 0 1").unwrap();
        let moves = state.gen_moves();
        let desc = MoveDescription::Simple {
            src_file: Some(2),
            src_rank: None,
            src_piece: Piece::Knight,
            dst_pos: d5,
        };
        let matched = desc.match_moves(moves);
        assert_ne!(matched, None);
    }

    #[test]
    fn test_match_moves_needs_disambiguating_rank() {
        let state = parse_fen("8/3k4/8/1N6/8/1N6/3K4/8 w - - 0 1").unwrap();
        let moves = state.gen_moves();
        let desc = MoveDescription::Simple {
            src_file: None,
            src_rank: None,
            src_piece: Piece::Knight,
            dst_pos: d4,
        };
        let matched = desc.match_moves(moves);
        assert_eq!(matched, None);
    }

    #[test]
    fn test_match_moves_has_disambiguating_rank() {
        let state = parse_fen("8/3k4/8/1N6/8/1N6/3K4/8 w - - 0 1").unwrap();
        let moves = state.gen_moves();
        let desc = MoveDescription::Simple {
            src_file: None,
            src_rank: Some(2),
            src_piece: Piece::Knight,
            dst_pos: d4,
        };
        let matched = desc.match_moves(moves);
        assert_ne!(matched, None);
    }

    #[test]
    fn test_match_moves_simple() {
        let mut game = test_game();
        for desc in &["e3", "e6", "Ke2", "e5", "Kd3", "e4"] {
            let next_moves = game.state.gen_moves();
            let move_desc = parse_algebraic_notation(desc).unwrap();
            game = Game {
                state: move_desc.match_moves(next_moves).unwrap().next,
            };
        }

        assert_eq!(
            game.state.board.piece_at(d3),
            Some((Player::White, Piece::King))
        );

        assert_eq!(
            game.state.board.piece_at(e4),
            Some((Player::Black, Piece::Pawn))
        );
    }

    #[test]
    fn test_match_moves_castles() {
        let mut game = test_game();
        for desc in &[
            "e4", "e6", "Bc4", "Nc6", "Nf3", "d6", "O-O", "Bd7", "d3", "Qf6", "Nc3", "O-O-O",
        ] {
            let next_moves = game.state.gen_moves();
            let move_desc = parse_algebraic_notation(desc).unwrap();
            game = Game {
                state: move_desc.match_moves(next_moves).unwrap().next,
            };
        }

        assert_eq!(
            game.state.board.piece_at(g1),
            Some((Player::White, Piece::King))
        );
        assert_eq!(
            game.state.board.piece_at(f1),
            Some((Player::White, Piece::Rook))
        );

        assert_eq!(
            game.state.board.piece_at(c8),
            Some((Player::Black, Piece::King))
        );
        assert_eq!(
            game.state.board.piece_at(d8),
            Some((Player::Black, Piece::Rook))
        );
    }
}
