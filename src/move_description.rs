use crate::castling::Castleside;
use crate::m0ve::{Action, Move};
use crate::piece::Piece;
use crate::pos::Pos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveDescription {
    Simple { src_piece: Piece, dst_pos: Pos },
    Castle { castleside: Castleside },
}

impl MoveDescription {
    pub fn match_moves(&self, moves: Vec<Move>) -> Option<Move> {
        for m0ve in moves {
            if self.match_move(&m0ve) {
                return Some(m0ve);
            }
        }
        None
    }

    fn match_move(&self, m0ve: &Move) -> bool {
        match (&m0ve.action, self) {
            (Action::Simple { from: _, to }, MoveDescription::Simple { src_piece, dst_pos }) => {
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
    use crate::algebraic_notation::parse_algebraic_notation;
    use crate::game::Game;
    use crate::player::Player;
    use crate::pos::*;

    fn test_game() -> Game {
        Game::default()
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
